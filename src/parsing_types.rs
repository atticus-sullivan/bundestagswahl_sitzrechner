// SPDX-FileCopyrightText: 2025 Lukas Heindl
//
// SPDX-License-Identifier: MIT

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Ergebnis {
    Gruppenergebnis(Gruppenergebnis),
    Direktergebnis(Direktergebnis),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Direktergebnis {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Stimmergebnis {
    #[serde(rename = "@Stimmart")]
    pub stimmart: Stimmart,
    #[serde(
        rename = "@Anzahl",
        default,
        deserialize_with = "deserialize_maybe_nan"
    )]
    pub anzahl: Option<u64>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Stimmart {
    #[serde(rename="DIREKT")]
    Direkt,
    #[serde(rename="LISTE")]
    Liste,
    #[serde(rename="KEINE")]
    Keine,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gebietsart {
    #[serde(rename="BUND")]
    Bund,
    #[serde(rename="LAND")]
    Land,
    #[serde(rename="WAHLKREIS")]
    Wahlkreis,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Gruppenart {
    #[serde(rename="PARTEI")]
    Partei,
    #[serde(rename="ALLGEMEIN")]
    Allgemein, // waehlende / wahlberechtigte / gueltig / ungueltig
    #[serde(rename="UEBRIGE")]
    Uebrige,
    #[serde(rename="EINZELKANDIDAT")]
    Einzelkandidat,
}

// custom deserializer function
fn deserialize_maybe_nan<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s == "n/a" {
        Ok(None)
    } else {
        match T::from_str(&s) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(serde::de::Error::custom("Invalid number")),
        }
    }
}
