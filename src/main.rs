mod parsing;
mod parsing_types;
mod sls;
mod types;

mod wahl;
mod wahl2021;

use std::collections::BTreeMap;

use env_logger::Logger;
use log::debug;

use anyhow::Result;

fn main() -> Result<()> {
    // let logger = Logger::from_default_env();
    env_logger::init();

    let stimmen = parsing::parse_xml()?;
    let struktur = parsing::parse_csv()?;

    let (bund, parteinr_name) = types::convert_data(stimmen, &struktur)?;

    // println!("{:#?}", bund);
    // println!("{:#?}", bund_laender);
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

    // wahl::wahl_2021::calc();
    let (sitze, total) = wahl2021::calc(bund.clone(), &parteinr_name)?;
    println!("Total sitze {}", total);
    for (p, s) in sitze.iter() {
        println!("{} -> {}", parteinr_name[p], s);
    }

    Ok(())
}
