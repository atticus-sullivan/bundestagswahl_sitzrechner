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

const COLOR_ALT_BG: Color = Color::Rgb {
    r: 105,
    g: 112,
    b: 153,
};

// fn election_2021() -> Result<()> {
//     let stimmen = parsing::parse_xml(Path::new(
//         "/media/daten/coding/bundestagswahl_sitzrechner/data/2021-gesamtergebnis_01.xml",
//     ))?;
//     let struktur = parsing::parse_csv(Path::new(
//         "/media/daten/coding/bundestagswahl_sitzrechner/data/2021-btw21_strukturdaten_corr.csv",
//     ))?;
//
//     let (bund, parteinr_name) = types::convert_data(stimmen, &struktur)?;
//
//     debug!("{:#?}", parteinr_name);
//     debug!(
//         "{:#?}",
//         bund.laender
//             .iter()
//             .enumerate()
//             .map(|(i, l)| (i, l.name.to_owned()))
//             .collect::<BTreeMap<_, _>>()
//     );
//     debug!(
//         "{:#?}",
//         bund.laender
//             .iter()
//             .enumerate()
//             .map(|(i, l)| (i, l.einwohner.to_owned()))
//             .collect::<BTreeMap<_, _>>()
//     );
//
//     let (sitze, total) = wahl2021::calc(bund.clone(), &parteinr_name)?;
//     println!("Total sitze {}", total);
//     for (p, s) in sitze.iter() {
//         println!("{} -> {}", parteinr_name[p], s);
//     }
//
//     Ok(())
// }

// fn election_2025() -> Result<()> {
//     let stimmen = parsing::parse_xml(Path::new(
//         "/media/daten/coding/bundestagswahl_sitzrechner/data/2025_gesamtergebnis_01.xml",
//     ))?;
//     let struktur = parsing::parse_csv(Path::new(
//         "/media/daten/coding/bundestagswahl_sitzrechner/data/2025-btw2025_strukturdaten.csv",
//     ))?;
//
//     let (bund, parteinr_name) = types::convert_data(stimmen, &struktur)?;
//
//     debug!("{:#?}", parteinr_name);
//     debug!(
//         "{:#?}",
//         bund.laender
//             .iter()
//             .enumerate()
//             .map(|(i, l)| (i, l.name.to_owned()))
//             .collect::<BTreeMap<_, _>>()
//     );
//     debug!(
//         "{:#?}",
//         bund.laender
//             .iter()
//             .enumerate()
//             .map(|(i, l)| (i, l.einwohner.to_owned()))
//             .collect::<BTreeMap<_, _>>()
//     );
//
//     let mut tab_content: BTreeMap<String, Vec<_>> = Default::default();
//     let mut header: Vec<_> = Default::default();
//     let mut totals: Vec<_> = Default::default();
//     let mut col_offset: usize = 0;
//     header.push("Partei");
//
//     header.push("2021");
//     let (sitze, total) = wahl2021::calc(bund.clone(), &parteinr_name)?;
//     for (p, s) in sitze.iter() {
//         tab_content
//             .entry(parteinr_name[p].clone())
//             .or_insert(vec!["".to_owned(); col_offset])
//             .push(format!("{}", s));
//     }
//     totals.push(format!("{}", total));
//     col_offset += 1;
//
//     header.push("2025");
//     let (sitze, total) = wahl2025::calc(bund.clone(), &parteinr_name)?;
//     for (p, s) in sitze.iter() {
//         tab_content
//             .entry(parteinr_name[p].clone())
//             .or_insert(vec!["".to_owned(); col_offset])
//             .push(format!("{}", s));
//     }
//     totals.push(format!("{}", total));
//
//     // assemble the table
//     let mut tab = Table::new();
//     tab
//         .force_no_tty()
//         .enforce_styling()
//         .load_preset(UTF8_FULL_CONDENSED)
//         .apply_modifier(UTF8_ROUND_CORNERS)
//         // .set_style(comfy_table::TableComponent::VerticalLines, '\u{2192}')
//         ;
//     tab.set_header(header);
//     tab.add_rows(tab_content.into_iter().map(|(pn, xs)| {
//         let mut v = vec![pn.clone()];
//         v.extend(xs);
//         v
//     }));
//     {
//         let mut t = vec!["totals".to_owned()];
//         t.extend(totals);
//         tab.add_row(t.into_iter().map(|i| Cell::new(i).bg(COLOR_ALT_BG)));
//     }
//
//     println!("{}", tab);
//
//     Ok(())
// }

fn elections(inputs: &[(&str, PathBuf, PathBuf)]) -> Result<()> {
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

        let mut tab_content: BTreeMap<String, Vec<_>> = Default::default();
        let mut header: Vec<_> = Default::default();
        let mut totals: Vec<_> = Default::default();
        let mut col_offset: usize = 0;
        header.push("Partei");

        header.push("S");
        let total_votes = bund
            .parteien
            .values()
            .filter_map(|x| x.zweitstimmen)
            .sum::<u64>();
        for (i, p) in bund.parteien.iter() {
            let x = tab_content.entry(parteinr_name[i].clone()).or_default();
            x.push(format!(
                "{:.2}",
                (p.zweitstimmen
                    .with_context(|| format!("no zweitstimmen for partei {i}"))?
                    as f64
                    / total_votes as f64)
                    * 100.0
            ));
        }

        header.push("2021");
        header.push("P");
        // header.push("B");
        let (sitze, total) = wahl2021::calc(bund.clone(), &parteinr_name)?;
        for (p, s) in sitze.iter() {
            let x = tab_content
                .entry(parteinr_name[p].clone())
                .or_insert(vec!["".to_owned(); col_offset]);
            x.push(format!("{}", s));
            x.push(format!("{:.2}", (*s as f64 / total as f64) * 100.0));
        }
        totals.push(format!("{}", total));
        col_offset += 1;

        header.push("2025");
        header.push("P");
        // header.push("B");
        let (sitze, total) = wahl2025::calc(bund.clone(), &parteinr_name)?;
        for (p, s) in sitze.iter() {
            let x = tab_content
                .entry(parteinr_name[p].clone())
                .or_insert(vec!["".to_owned(); col_offset]);
            x.push(format!("{}", s));
            x.push(format!("{:.2}", (*s as f64 / total as f64) * 100.0));
        }
        totals.push(format!("{}", total));

        // assemble the table
        let mut tab = Table::new();
        tab
            .force_no_tty()
            .enforce_styling()
            .load_preset(UTF8_FULL_CONDENSED)
            .apply_modifier(UTF8_ROUND_CORNERS)
        // .set_style(comfy_table::TableComponent::VerticalLines, '\u{2192}')
        ;
        tab.set_header(header);
        tab.add_rows(tab_content.into_iter().map(|(pn, xs)| {
            let mut v = vec![pn.clone()];
            v.extend(xs);
            v
        }));
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

    // election_2021()?;
    // election_2025()?;
    elections(&[
        (
            "2025",
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2025_gesamtergebnis_01.xml"),
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2025-btw2025_strukturdaten.csv"),
        ),
        (
            "2021",
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2021-gesamtergebnis_01.xml"),
            PathBuf::from("/media/daten/coding/bundestagswahl_sitzrechner/data/2021-btw21_strukturdaten_corr.csv"),
        ),
    ])?;

    Ok(())
}
