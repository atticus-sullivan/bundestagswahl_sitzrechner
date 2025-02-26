use anyhow::{Context, Result};
use core::str;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parsing_types::Gesamtergebnis;
use crate::types::{Gebiet, GebietNr};

pub fn parse_xml() -> Result<Gesamtergebnis> {
    let file = File::open(
        "/media/daten/coding/bundestagswahl_sitzrechner/data/2021-gesamtergebnis_01.xml",
    )?;
    let buf_reader = BufReader::new(file);
    let x: Gesamtergebnis = quick_xml::de::from_reader(buf_reader)?;
    Ok(x)
}

pub fn parse_csv() -> Result<HashMap<GebietNr, Gebiet>> {
    let file = File::open(
        "/media/daten/coding/bundestagswahl_sitzrechner/data/2021-btw21_strukturdaten.csv",
    )?;
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
        if buf_reader.read_line(&mut line)? <= 0 {
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
        if header.starts_with("Bevölkerung am") && header.ends_with("Insgesamt (in 1000)") {
            population_column = Some(i);
            break;
        }
    }
    let population_column =
        population_column.context("no column containing the total population found")?;

    let mut gebiete = HashMap::new();
    // Process the CSV records
    for result in rdr.records() {
        let record = result?;

        // Extract Wahlkreis-Nr. and population

        // Wahlkreis-Nr. is the second column
        let gebietsnummer: GebietNr = record[1].parse()?;

        // Find population from variable column
        let population: f64 = record
            .get(population_column)
            .context("unable to retrieve the population")?
            .replace(',', ".")
            .parse::<f64>()
            .context("failed parsing the population")?;

        gebiete.insert(
            gebietsnummer,
            Gebiet::new(record[2].to_owned(), population, gebietsnummer),
        );
    }

    Ok(gebiete)
}
