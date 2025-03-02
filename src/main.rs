mod parsing;
mod parsing_types;
mod sls;
mod types;

mod wahl;
mod wahl2021;
mod wahl2025;

use std::collections::BTreeMap;
use std::path::Path;

use log::debug;

use anyhow::Result;

fn election_2021() -> Result<()> {
    let stimmen = parsing::parse_xml(Path::new(
        "/media/daten/coding/bundestagswahl_sitzrechner/data/2021-gesamtergebnis_01.xml",
    ))?;
    let struktur = parsing::parse_csv(Path::new(
        "/media/daten/coding/bundestagswahl_sitzrechner/data/2021-btw21_strukturdaten_corr.csv",
    ))?;

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

    let (sitze, total) = wahl2021::calc(bund.clone(), &parteinr_name)?;
    println!("Total sitze {}", total);
    for (p, s) in sitze.iter() {
        println!("{} -> {}", parteinr_name[p], s);
    }

    Ok(())
}

fn election_2025() -> Result<()> {
    let stimmen = parsing::parse_xml(Path::new(
        "/media/daten/coding/bundestagswahl_sitzrechner/data/2025_gesamtergebnis_01.xml",
    ))?;
    let struktur = parsing::parse_csv(Path::new(
        "/media/daten/coding/bundestagswahl_sitzrechner/data/2025-btw2025_strukturdaten.csv",
    ))?;

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

    let (sitze, total) = wahl2025::calc(bund.clone(), &parteinr_name)?;
    println!("Total sitze {}", total);
    for (p, s) in sitze.iter() {
        println!("{} -> {}", parteinr_name[p], s);
    }

    Ok(())
}

fn main() -> Result<()> {
    // let logger = Logger::from_default_env();
    env_logger::init();

    // election_2021()?;
    election_2025()?;

    Ok(())
}
