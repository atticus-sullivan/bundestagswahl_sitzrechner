// SPDX-FileCopyrightText: 2025 Lukas Heindl
//
// SPDX-License-Identifier: MIT

mod banzhaf;
mod parsing;
mod parsing_types;
mod sls;
mod types;

mod wahl;
mod wahl2021;
mod wahl2025;

use std::collections::BTreeMap;
use std::path::PathBuf;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, Color, Table};
use log::debug;

use anyhow::{Context, Result};
use types::{Bund, GruppeNr};

const COLOR_ALT_BG: Color = Color::Rgb {
    r: 105,
    g: 112,
    b: 153,
};

pub trait ElectionCalc {
    fn calc(
        &self,
        bund: Bund,
        parteinr_name: &BTreeMap<GruppeNr, String>,
    ) -> Result<(BTreeMap<GruppeNr, u64>, u64, Bund)>;
}

struct ElectionCalc2021 {}
impl ElectionCalc for ElectionCalc2021 {
    fn calc(
        &self,
        bund: Bund,
        parteinr_name: &BTreeMap<GruppeNr, String>,
    ) -> Result<(BTreeMap<GruppeNr, u64>, u64, Bund)> {
        wahl2021::calc(bund, parteinr_name)
    }
}
struct ElectionCalc2025 {}
impl ElectionCalc for ElectionCalc2025 {
    fn calc(
        &self,
        bund: Bund,
        parteinr_name: &BTreeMap<GruppeNr, String>,
    ) -> Result<(BTreeMap<GruppeNr, u64>, u64, Bund)> {
        wahl2025::calc(bund, parteinr_name)
    }
}

fn elections(
    inputs: &[(&str, PathBuf, PathBuf)],
    calcs: Vec<(&str, Box<dyn ElectionCalc>)>,
) -> Result<()> {
    for (name, stimmen, struktur) in inputs.iter() {
        println!("{name}");
        let stimmen = parsing::parse_xml(stimmen)?;
        let struktur = parsing::parse_csv(struktur)?;

        let (bund, parteinr_name) = types::convert_data(stimmen, &struktur)?;

        debug!("{:#?}", parteinr_name);
        debug!(
            "{:#?}",
            bund.laender
                .iter()
                .enumerate()
                .map(|(i, l)| (i, l.name.to_owned()))
                .collect::<BTreeMap<_, _>>()
        );
        debug!(
            "{:#?}",
            bund.laender
                .iter()
                .enumerate()
                .map(|(i, l)| (i, l.einwohner.to_owned()))
                .collect::<BTreeMap<_, _>>()
        );

        // collect the main body of the table here
        // maps parteiname to values (one row in the table)
        let mut tab_content: BTreeMap<String, Vec<_>> = Default::default();
        // collect the header columns here
        let mut header: Vec<_> = Default::default();
        // as last row, there is one with some totals
        let mut totals: Vec<_> = Default::default();
        // store at which columns offset the current scheme should be placed in case the partei was
        // skipped in one scheme or so
        let mut col_offset: usize = 0;

        // values in the individual rows depends on which partei get enough votes -> is determined
        // later
        header.push("Partei");

        // polulate everything with the "Stimmen" column values
        header.push("Stimmen");
        let total_votes = bund
            .parteien
            .values()
            .filter_map(|x| x.zweitstimmen)
            .sum::<u64>();
        // collect how many votes are shown in the table in order to display how many percent of
        // all the votes are shown (rest is "sonstige" >= 0.5 %)
        let mut shown_votes_p: f64 = 0.0;
        for (i, p) in bund.parteien.iter() {
            let z = p
                .zweitstimmen
                .with_context(|| format!("no zweitstimmen for partei {i}"))?;
            let percentage = (z as f64 / total_votes as f64) * 100.0;

            // hide parteien which got very little votes
            if percentage < 0.5 {
                continue;
            }

            let x = tab_content.entry(parteinr_name[i].clone()).or_default();
            shown_votes_p += percentage;
            x.push(format!("{:.2}", percentage));
        }
        totals.push(format!("{:.2}", shown_votes_p));

        // add columns for each scheme that should be calculated
        for (scheme_n, scheme) in calcs.iter() {
            // add one empty column as separator before each scheme
            totals.extend([""].into_iter().map(|x| x.to_owned()));
            header.push("");
            col_offset += 1;

            // add the headers for the columns which are inserted in the following
            header.push(scheme_n);
            // percentage of votes for partei, not considering parteien which were neglected (too
            // few votes) in this scheme
            header.push("% > H");
            // percentage of seats for that partei
            header.push("% Sitz");
            // Banzhaf’sche Machtindex of the respective partei which results from the seats
            // distribution
            // see: https://www.math-it.org/Mathematik/MathPol/Banzhaf_de.html
            header.push("Banzhaf");

            // calculate the seat-distribution of this scheme with the election results
            let (sitze, total, bund_h) = scheme.calc(bund.clone(), &parteinr_name)?;
            // calculate the power-index
            let banzhaf = banzhaf::banzhaf(&sitze)?;

            // get total amount of filtered (if scheme neglects that partei) votes for percentage
            // calculations
            let total_votes = bund_h
                .parteien
                .values()
                .filter_map(|x| x.zweitstimmen)
                .sum::<u64>();

            //
            for (p, seats) in sitze.iter() {
                // obtain the row should be appended to (potentially create it)
                let x = tab_content
                    .entry(parteinr_name[p].clone())
                    .or_insert(vec!["".to_owned(); col_offset]);

                // fill up with empty cells to ensure the content is aligned properly
                if x.len() <= col_offset {
                    x.extend(
                        vec![""; 1 + col_offset - x.len()]
                            .into_iter()
                            .map(|x| x.to_owned()),
                    );
                }

                x.push(format!("{}", seats));
                // percentage of filtered votes for that partei
                x.push(format!(
                    "{:.2}",
                    (bund_h.parteien[p]
                        .zweitstimmen
                        .with_context(|| format!("no zweitstimmen for partei {p}"))?
                        as f64
                        / total_votes as f64)
                        * 100.0
                ));
                // percentage of seats
                x.push(format!("{:.2}", (*seats as f64 / total as f64) * 100.0));
                // power index
                x.push(format!("{:.3}", banzhaf.get(p).unwrap_or(&0.0)))
            }
            // total amount of needed seats with that scheme
            totals.push(format!("{}", total));
            // skip the columns which do not get total values
            totals.extend(["", "", ""].into_iter().map(|x| x.to_owned()));
            col_offset += 4;
        }

        // assemble the table
        let mut tab = Table::new();
        // style the table
        tab.force_no_tty()
            .enforce_styling()
            .load_preset(UTF8_FULL_CONDENSED)
            .apply_modifier(UTF8_ROUND_CORNERS);

        tab.set_header(header);
        // convert BTreeMap to flat vector with the key prepended to the value
        tab.add_rows(tab_content.into_iter().map(|(pn, xs)| {
            let mut v = vec![pn.clone()];
            v.extend(xs);
            v
        }));
        // prepend "totals" literal to the totals row and style that row
        {
            let mut t = vec!["totals".to_owned()];
            t.extend(totals);
            tab.add_row(t.into_iter().map(|i| Cell::new(i).bg(COLOR_ALT_BG)));
        }

        println!("{}", tab);
    }
    Ok(())
}

fn main() -> Result<()> {
    // let logger = Logger::from_default_env();
    env_logger::init();

    elections(&[
        (
            "2021",
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2021-gesamtergebnis_01.xml"),
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2021-btw21_strukturdaten_corr.csv"),
        ),
        (
            "2025",
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2025_gesamtergebnis_01.xml"),
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2025-btw2025_strukturdaten.csv"),
        ),
    ], vec![
        ("2021", Box::new(ElectionCalc2021{})),
        ("2025", Box::new(ElectionCalc2025{})),
    ])?;

    Ok(())
}
