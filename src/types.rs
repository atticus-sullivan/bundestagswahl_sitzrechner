use anyhow::{Context, Result};
use std::collections::BTreeMap;

use crate::parsing_types::{
    Ergebnis, Gebietsart, Gesamtergebnis, Gruppenart, Gruppenergebnis, Stimmart,
};

pub type GebietNr = u16;
pub type GruppeNr = i16;

#[derive(Debug, PartialEq)]
pub struct Gebiet {
    name: String,
    einwohner: f64,
    gebietsnummer: GebietNr,
}
impl Gebiet {
    pub fn new(name: String, einwohner: f64, gebietsnummer: GebietNr) -> Gebiet {
        Gebiet {
            name,
            einwohner,
            gebietsnummer,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParteiWahlkreis {
    pub erststimmen: Option<u64>,
    pub zweitstimmen: Option<u64>,
}
impl From<&Gruppenergebnis> for ParteiWahlkreis {
    fn from(value: &Gruppenergebnis) -> Self {
        let erststimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::Direkt)
            .map(|x| x.anzahl);
        let zweitstimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::Liste)
            .map(|x| x.anzahl);
        Self {
            erststimmen: erststimmen.unwrap_or(Some(0)),
            zweitstimmen: zweitstimmen.unwrap_or(Some(0)),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParteiLand {
    pub erststimmen: Option<u64>,
    pub zweitstimmen: Option<u64>,
}
impl From<&Gruppenergebnis> for ParteiLand {
    fn from(value: &Gruppenergebnis) -> Self {
        let erststimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::Direkt)
            .map(|x| x.anzahl);
        let zweitstimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::Liste)
            .map(|x| x.anzahl);
        Self {
            erststimmen: erststimmen.unwrap_or(Some(0)),
            zweitstimmen: zweitstimmen.unwrap_or(Some(0)),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParteiBund {
    pub erststimmen: Option<u64>,
    pub zweitstimmen: Option<u64>,
}
impl From<&Gruppenergebnis> for ParteiBund {
    fn from(value: &Gruppenergebnis) -> Self {
        let erststimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::Direkt)
            .map(|x| x.anzahl);
        let zweitstimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::Liste)
            .map(|x| x.anzahl);
        Self {
            erststimmen: erststimmen.unwrap_or(Some(0)),
            zweitstimmen: zweitstimmen.unwrap_or(Some(0)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bund {
    pub laender: Vec<Land>,
    pub parteien: BTreeMap<GruppeNr, ParteiBund>,
}
impl Bund {
    fn new(
        laender: Vec<(String, GebietNr)>,
        mut laender_wahlkreise: BTreeMap<GebietNr, Vec<(String, GebietNr)>>,
        mut wahlkreise_parteien: BTreeMap<
            GebietNr,
            BTreeMap<GebietNr, BTreeMap<GruppeNr, ParteiWahlkreis>>,
        >,
        mut laender_parteien: BTreeMap<GebietNr, BTreeMap<GruppeNr, ParteiLand>>,
        parteien: BTreeMap<GruppeNr, ParteiBund>,
        struktur: &BTreeMap<GebietNr, Gebiet>,
    ) -> Result<Self> {
        let laender = laender
            .iter()
            .map(|i| {
                let nr = &i.1;
                Land::new(
                    i.0.clone(),
                    *nr,
                    laender_wahlkreise
                        .remove(nr)
                        .with_context(|| format!("land {nr} not found in laender_wahlkreise"))?,
                    wahlkreise_parteien
                        .remove(nr)
                        .with_context(|| format!("land {nr} not found in wahlkreise_parteien"))?,
                    laender_parteien
                        .remove(nr)
                        .with_context(|| format!("land {nr} not found in laender_parteien"))?,
                    struktur,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { laender, parteien })
    }
}

#[derive(Debug, Clone)]
pub struct Land {
    pub name: String,
    pub einwohner: f64,
    pub parteien: BTreeMap<GruppeNr, ParteiLand>,
    pub wahlkreise: Vec<Wahlkreis>,
}
impl Land {
    fn new(
        name: String,
        gebietsnummer: GebietNr,
        wahlkreise: Vec<(String, GebietNr)>,
        mut wahlkreise_parteien: BTreeMap<GebietNr, BTreeMap<GruppeNr, ParteiWahlkreis>>,
        parteien: BTreeMap<GruppeNr, ParteiLand>,
        struktur: &BTreeMap<GebietNr, Gebiet>,
    ) -> Result<Self> {
        let wahlkreise = wahlkreise
            .iter()
            .map(|i| {
                let nr = &i.1;
                Wahlkreis::new(
                    i.0.clone(),
                    *nr,
                    wahlkreise_parteien
                        .remove(nr)
                        .with_context(|| format!("wahlkreis {nr} not found"))?,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            wahlkreise,
            parteien,
            einwohner: struktur
                .get(&(gebietsnummer + 900))
                .with_context(|| {
                    format!(
                        "population not found for Land {}/{}",
                        gebietsnummer,
                        gebietsnummer + 900
                    )
                })?
                .einwohner,
            name,
        })
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Wahlkreis {
    pub name: String,
    pub parteien: BTreeMap<GruppeNr, ParteiWahlkreis>,
}

impl Wahlkreis {
    fn new(
        name: String,
        _gebietsnummer: GebietNr,
        parteien: BTreeMap<GruppeNr, ParteiWahlkreis>,
    ) -> Result<Self> {
        Ok(Self { parteien, name })
    }
}

pub fn convert_data(
    stimmen: Gesamtergebnis,
    struktur: &BTreeMap<GebietNr, Gebiet>,
) -> Result<(Bund, BTreeMap<GruppeNr, String>)> {
    let mut wahlkreise_parteien: BTreeMap<
        GebietNr,
        BTreeMap<GebietNr, BTreeMap<GruppeNr, ParteiWahlkreis>>,
    > = Default::default();
    let mut laender_parteien: BTreeMap<GebietNr, BTreeMap<GruppeNr, ParteiLand>> =
        Default::default();
    let mut bund_parteien: BTreeMap<GruppeNr, ParteiBund> = Default::default();

    let mut bund_laender: Vec<(String, GebietNr)> = Default::default();
    let mut laender_wahlkreise: BTreeMap<GebietNr, Vec<(String, GebietNr)>> = Default::default();

    let mut parteinr_name: BTreeMap<GruppeNr, String> = Default::default();

    for ge in stimmen.gebietsergebnisse.iter() {
        match ge.gebietsart {
            Gebietsart::Bund => {
                // collect parties in vector
                bund_parteien = ge
                    .ergebnisse
                    .iter()
                    .filter_map(|i| match i {
                        Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                            if let Gruppenart::Partei = gruppenergebnis.gruppenart {
                                Some(gruppenergebnis)
                            } else {
                                None
                            }
                        }
                        Ergebnis::Direktergebnis(_) => None,
                    })
                    .map(|i| (i.gruppe, ParteiBund::from(i)))
                    .collect::<BTreeMap<_, _>>();
            }
            Gebietsart::Land => {
                // register in parent structure (bund)
                bund_laender.push((ge.gebiet_text.to_owned(), ge.gebietsnummer));
                // collect parties in vector
                laender_parteien.insert(
                    ge.gebietsnummer,
                    ge.ergebnisse
                        .iter()
                        .filter_map(|i| match i {
                            Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                                if let Gruppenart::Partei = gruppenergebnis.gruppenart {
                                    Some(gruppenergebnis)
                                } else {
                                    None
                                }
                            }
                            Ergebnis::Direktergebnis(_) => None,
                        })
                        .map(|i| (i.gruppe, ParteiLand::from(i)))
                        .collect::<BTreeMap<_, _>>(),
                );
            }
            Gebietsart::Wahlkreis => {
                // register in parent structure (land)
                laender_wahlkreise
                    .entry(
                        ge.ueg_gebietsnummer.with_context(|| {
                            format!("{} no ueg_gebietsnummer", ge.gebietsnummer)
                        })?,
                    )
                    .or_default()
                    .push((ge.gebiet_text.to_owned(), ge.gebietsnummer));
                // collect parties in vector
                wahlkreise_parteien
                    .entry(ge.ueg_gebietsnummer.with_context(|| {
                        format!("no ueg_gebietsnummer set for {}", ge.gebiet_text)
                    })?)
                    .or_default()
                    .insert(
                        ge.gebietsnummer,
                        ge.ergebnisse
                            .iter()
                            .filter_map(|i| match i {
                                Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                                    if let Gruppenart::Partei = gruppenergebnis.gruppenart {
                                        Some(gruppenergebnis)
                                    } else {
                                        None
                                    }
                                }
                                Ergebnis::Direktergebnis(_) => None,
                            })
                            .map(|i| (i.gruppe, ParteiWahlkreis::from(i)))
                            .collect::<BTreeMap<_, _>>(),
                    );
            }
        }

        parteinr_name.extend(ge.ergebnisse.iter().filter_map(|i| match i {
            Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                if let Gruppenart::Partei = gruppenergebnis.gruppenart {
                    Some((gruppenergebnis.gruppe, gruppenergebnis.name.to_owned()))
                } else {
                    None
                }
            }
            Ergebnis::Direktergebnis(_) => None,
        }));
    }

    Ok((
        Bund::new(
            bund_laender,
            laender_wahlkreise,
            wahlkreise_parteien,
            laender_parteien,
            bund_parteien,
            struktur,
        )?,
        parteinr_name,
    ))
}
