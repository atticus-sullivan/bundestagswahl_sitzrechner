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
mod wahl_mehrheit;

use std::collections::BTreeMap;
use std::path::PathBuf;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, Color, Table};

use log::debug;

use anyhow::{Context, Result};
use clap::Parser;

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
    ) -> Result<(BTreeMap<GruppeNr, (u64, u64)>, u64, Bund)>;
}

#[derive(Clone, Debug)]
struct ElectionCalc2021 {}
impl ElectionCalc for ElectionCalc2021 {
    fn calc(
        &self,
        bund: Bund,
        parteinr_name: &BTreeMap<GruppeNr, String>,
    ) -> Result<(BTreeMap<GruppeNr, (u64, u64)>, u64, Bund)> {
        wahl2021::calc(bund, parteinr_name)
    }
}
#[derive(Clone, Debug)]
struct ElectionCalc2025 {}
impl ElectionCalc for ElectionCalc2025 {
    fn calc(
        &self,
        bund: Bund,
        parteinr_name: &BTreeMap<GruppeNr, String>,
    ) -> Result<(BTreeMap<GruppeNr, (u64, u64)>, u64, Bund)> {
        wahl2025::calc(bund, parteinr_name)
    }
}
#[derive(Clone, Debug)]
struct ElectionCalcMehrheit {}
impl ElectionCalc for ElectionCalcMehrheit {
    fn calc(
        &self,
        bund: Bund,
        parteinr_name: &BTreeMap<GruppeNr, String>,
    ) -> Result<(BTreeMap<GruppeNr, (u64, u64)>, u64, Bund)> {
        wahl_mehrheit::calc(bund, parteinr_name)
    }
}

fn elections(
    inputs: &[(String, PathBuf, PathBuf)],
    calcs: &Vec<(String, Option<String>, Box<dyn ElectionCalc>)>,
    calc_ops: &[CalcOp],
) -> Result<()> {
    for (name, stimmen, struktur) in inputs.iter() {
        println!("{name}");
        for calc_op in calc_ops.iter() {
            if calc_ops.len() > 1 {
                println!("    {}", calc_op.name());
            }

            let stimmen = parsing::parse_xml(stimmen)
                .with_context(|| format!("error reading {:?}", stimmen))?;
            let struktur = parsing::parse_csv(struktur)
                .with_context(|| format!("error reading {:?}", struktur))?;

            let (mut bund, parteinr_name) = types::convert_data(stimmen, &struktur)?;

            calc_op.exec(&mut bund, &parteinr_name);

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
            for (scheme_n, wkm_name, scheme) in calcs.iter() {
                // add one empty column as separator before each scheme
                totals.extend([""].into_iter().map(|x| x.to_owned()));
                header.push("");
                col_offset += 1;

                // add the headers for the columns which are inserted in the following
                header.push(scheme_n);
                if let Some(wkm_name) = wkm_name {
                    // amount of wahlkreismandate/direktmandate of that party
                    header.push(wkm_name);
                }
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
                    let (wkm, seats) = seats;
                    // obtain the row should be appended to (potentially create it)
                    let x =
                        tab_content
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
                    if wkm_name.is_some() {
                        x.push(format!("{}", wkm));
                    }
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
                totals.push(format!("{}", sitze.values().map(|(_, i)| i).sum::<u64>()));
                // skip the columns which do not get total values
                totals.extend(["", "", ""].into_iter().map(|x| x.to_owned()));
                col_offset += 5;
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
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'd', long = "data", default_value = "./data/")]
    data_stem: PathBuf,
    years: Vec<String>,
    #[arg(short = 's', long = "scheme")]
    schemes: Vec<Scheme>,
    #[arg(long = "op", default_value = "none")]
    calc_ops: Vec<CalcOp>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Scheme {
    Scheme2021,
    Scheme2025,
    SchemeMehrheit,
}
impl Scheme {
    fn to_election_calc(&self) -> Box<dyn ElectionCalc> {
        match self {
            Scheme::Scheme2021 => Box::new(ElectionCalc2021 {}),
            Scheme::Scheme2025 => Box::new(ElectionCalc2025 {}),
            Scheme::SchemeMehrheit => Box::new(ElectionCalcMehrheit{}),
        }
    }
    fn title(&self) -> String {
        match self {
            Scheme::Scheme2021 => "2021",
            Scheme::Scheme2025 => "2025",
            Scheme::SchemeMehrheit => "Mehrheit",
        }
        .to_owned()
    }
    fn wkm_name(&self) -> Option<String> {
        match self {
            Scheme::Scheme2021 => Some("DM".to_owned()),
            Scheme::Scheme2025 => Some("WKM".to_owned()),
            Scheme::SchemeMehrheit => None,
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum CalcOp {
    None,
    MergeCduCsu,
}
impl CalcOp {
    fn exec(&self, bund: &mut Bund, parteinr_name: &BTreeMap<GruppeNr, String>) {
        match self {
            CalcOp::MergeCduCsu => bund.merge_parteien(
                &["CDU", "CSU"]
                    .into_iter()
                    .map(|ps| {
                        parteinr_name
                            .iter()
                            .find_map(|(i, p)| if ps == p { Some(*i) } else { None })
                            .unwrap()
                    })
                    .collect::<Vec<_>>(),
            ),
            CalcOp::None => {}
        }
    }

    fn name(&self) -> String {
        match self {
            CalcOp::None => "none",
            CalcOp::MergeCduCsu => "merged cdu/csu",
        }
        .to_owned()
    }
}

fn main() -> Result<()> {
    // let logger = Logger::from_default_env();
    env_logger::init();

    let args = Cli::parse();

    elections(
        &args
            .years
            .iter()
            .map(|y| {
                (
                    y.to_string(),
                    args.data_stem.join(format!("{y}_gesamtergebnis.xml")),
                    args.data_stem.join(format!("{y}_strukturdaten.csv")),
                )
            })
            .collect::<Vec<_>>(),
        &args
            .schemes
            .iter()
            .map(|s| (s.title(), s.wkm_name(), s.to_election_calc()))
            .collect::<Vec<_>>(),
        &args.calc_ops,
    )?;

    Ok(())
}
