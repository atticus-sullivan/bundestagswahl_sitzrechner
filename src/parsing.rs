use anyhow::{Context, Result};
use core::str;
use csv::ReaderBuilder;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parsing_types::Gesamtergebnis;
use crate::types::{Gebiet, GebietNr};

pub fn parse_xml(path: &Path) -> Result<Gesamtergebnis> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let x: Gesamtergebnis = quick_xml::de::from_reader(buf_reader)?;
    Ok(x)
}

pub fn parse_csv(path: &Path) -> Result<BTreeMap<GebietNr, Gebiet>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    // Manually advance the file pointer by reading through lines
    let mut line = String::new();
    line.clear();
    buf_reader.read_line(&mut line)?;

    // remove potential BOM at the start
    let lb = line.as_bytes();
    if lb[0] == 0xEF && lb[1] == 0xBB && lb[2] == 0xBF {
        line = str::from_utf8(&lb[3..])?.to_owned();
    }

    // Skip the initial non-header lines until the actual header row
    while line.starts_with("#") {
        // Clear line for next iteration
        line.clear();
        if buf_reader.read_line(&mut line)? == 0 {
            break;
        }
    }

    // Create a CSV reader and read the file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(buf_reader);

    // Find the correct population column name (this can change by year)
    let headers = rdr.headers()?;
    let mut population_column = None;
    // Search for the column with a name containing the string "Bevölkerung" followed by a year
    for (i, header) in headers.iter().enumerate() {
        if header.starts_with("Bevölkerung am") && header.ends_with("Deutsche (in 1000)") {
            population_column = Some(i);
            break;
        }
    }
    let population_column =
        population_column.context("no column containing the total population found")?;

    let mut gebiete = BTreeMap::new();
    // Process the CSV records
    for result in rdr.records() {
        let record = result?;

        // Extract Wahlkreis-Nr. and population

        // Wahlkreis-Nr. is the second column
        let gebietsnummer: GebietNr = record[1].parse()?;

        // Find population from variable column
        let population: f64 = record
            .get(population_column)
            .with_context(|| format!("unable to retrieve the population for {gebietsnummer} from column {population_column}"))?
            .replace(',', ".")
            .parse::<f64>()
            .with_context(|| format!("failed parsing the population for {gebietsnummer} from column {population_column}"))?
            * 1000.0;

        gebiete.insert(
            gebietsnummer,
            Gebiet::new(record[2].to_owned(), population, gebietsnummer),
        );
    }

    Ok(gebiete)
}
