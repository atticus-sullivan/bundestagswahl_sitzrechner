use anyhow::Result;
use core::str;
use std::str::FromStr;

use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

use crate::types::{GebietNr, GruppeNr};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Gesamtergebnis {
    #[serde(rename = "Gebietsergebnis")]
    pub gebietsergebnisse: Vec<Gebietsergebnis>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Gebietsergebnis {
    // AuswertungseinheitenErwartet
    // AuswertungseinheitenEingetroffen
    // ErgebnisVollstaendig
    // NrAenderung
    #[serde(rename = "@Gebietsnummer")]
    pub gebietsnummer: GebietNr,
    #[serde(rename = "@Gebietsart")]
    pub gebietsart: Gebietsart,
    #[serde(rename = "@UegGebietsart")]
    pub ueg_gebietsart: Option<Gebietsart>,
    #[serde(rename = "@UegGebietsnummer")]
    pub ueg_gebietsnummer: Option<GebietNr>,
    // AuswertungseinheitenGesamt
    #[serde(rename = "GebietText")]
    pub gebiet_text: String,
    #[serde(rename = "$value")]
    pub ergebnisse: Vec<Ergebnis>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Ergebnis {
    Gruppenergebnis(Gruppenergebnis),
    Direktergebnis(Direktergebnis),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Direktergebnis {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Gruppenergebnis {
    #[serde(rename = "@Gruppe")]
    pub gruppe: GruppeNr,
    #[serde(rename = "@Gruppenart")]
    pub gruppenart: Gruppenart,
    #[serde(rename = "@Name")]
    pub name: String,
    // direktkandidat
    #[serde(rename = "$value")]
    pub stimmergebnisse: Vec<Stimmergebnis>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Stimmergebnis {
    #[serde(rename = "@Stimmart")]
    pub stimmart: Stimmart,
    #[serde(rename = "@Anzahl")]
    pub anzahl: u64,
    #[serde(
        rename = "@Prozent",
        default,
        deserialize_with = "deserialize_maybe_nan"
    )]
    pub prozent: Option<f64>,
    // AnzahlVergleich
    // ProzentVergleich
    // Differenz
    // ProzentDifferenz
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Stimmart {
    DIREKT,
    LISTE,
    KEINE,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gebietsart {
    BUND,
    LAND,
    WAHLKREIS,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gruppenart {
    PARTEI,
    ALLGEMEIN, // waehlende / wahlberechtigte / gueltig / ungueltig
    UEBRIGE,
    EINZELKANDIDAT,
}

// custom deserializer function
fn deserialize_maybe_nan<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s == "n/a" {
        Ok(None)
    } else {
        match f64::from_str(&s) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(serde::de::Error::custom("Invalid number")),
        }
    }
}
