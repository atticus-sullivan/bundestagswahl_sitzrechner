use anyhow::{Context, Result};
use std::collections::HashMap;

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

#[derive(Debug)]
pub struct ParteiWahlkreis {
    erststimmen: Option<u64>,
    zweitstimmen: Option<u64>,
}
impl From<&Gruppenergebnis> for ParteiWahlkreis {
    fn from(value: &Gruppenergebnis) -> Self {
        let erststimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::DIREKT)
            .map(|x| x.anzahl);
        let zweitstimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::LISTE)
            .map(|x| x.anzahl);
        Self {
            erststimmen,
            zweitstimmen,
        }
    }
}

#[derive(Debug)]
pub struct ParteiLand {
    erststimmen: Option<u64>,
    zweitstimmen: Option<u64>,
}
impl From<&Gruppenergebnis> for ParteiLand {
    fn from(value: &Gruppenergebnis) -> Self {
        let erststimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::DIREKT)
            .map(|x| x.anzahl);
        let zweitstimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::LISTE)
            .map(|x| x.anzahl);
        Self {
            erststimmen,
            zweitstimmen,
        }
    }
}

#[derive(Debug)]
pub struct ParteiBund {
    erststimmen: Option<u64>,
    zweitstimmen: Option<u64>,
}
impl From<&Gruppenergebnis> for ParteiBund {
    fn from(value: &Gruppenergebnis) -> Self {
        let erststimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::DIREKT)
            .map(|x| x.anzahl);
        let zweitstimmen = value
            .stimmergebnisse
            .iter()
            .find(|i| i.stimmart == Stimmart::LISTE)
            .map(|x| x.anzahl);
        Self {
            erststimmen,
            zweitstimmen,
        }
    }
}

#[derive(Debug)]
pub struct Bund {
    laender: Vec<Land>,
    parteien: HashMap<GruppeNr, ParteiBund>,
}
impl Bund {
    fn new(
        laender: Vec<(String, GebietNr)>,
        mut laender_wahlkreise: HashMap<GebietNr, Vec<(String, GebietNr)>>,
        mut wahlkreise_parteien: HashMap<
            GebietNr,
            HashMap<GebietNr, HashMap<GruppeNr, ParteiWahlkreis>>,
        >,
        mut laender_parteien: HashMap<GebietNr, HashMap<GruppeNr, ParteiLand>>,
        parteien: HashMap<GruppeNr, ParteiBund>,
        struktur: &HashMap<GebietNr, Gebiet>,
    ) -> Result<Self> {
        let laender = laender
            .iter()
            .map(|i| {
                let nr = &i.1;
                Land::new(
                    i.0.clone(),
                    *nr,
                    laender_wahlkreise.remove(nr).context("land not found")?,
                    wahlkreise_parteien.remove(nr).context("land not found")?,
                    laender_parteien.remove(nr).context("land not found")?,
                    struktur,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { laender, parteien })
    }
}

#[derive(Debug)]
pub struct Land {
    name: String,
    einwohner: f64,
    parteien: HashMap<GruppeNr, ParteiLand>,
    wahlkreise: Vec<Wahlkreis>,
}
impl Land {
    fn new(
        name: String,
        gebietsnummer: GebietNr,
        wahlkreise: Vec<(String, GebietNr)>,
        mut wahlkreise_parteien: HashMap<GebietNr, HashMap<GruppeNr, ParteiWahlkreis>>,
        parteien: HashMap<GruppeNr, ParteiLand>,
        struktur: &HashMap<GebietNr, Gebiet>,
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
                        .context("wahlkreis not found")?,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            wahlkreise,
            parteien,
            einwohner: struktur
                .get(&gebietsnummer)
                .context("population not found for Land")?
                .einwohner,
            name,
        })
    }
}

#[derive(Debug)]
pub struct Wahlkreis {
    name: String,
    parteien: HashMap<GruppeNr, ParteiWahlkreis>,
}

impl Wahlkreis {
    fn new(
        name: String,
        _gebietsnummer: GebietNr,
        parteien: HashMap<GruppeNr, ParteiWahlkreis>,
    ) -> Result<Self> {
        Ok(Self { parteien, name })
    }
}

pub fn convert_data(
    stimmen: Gesamtergebnis,
    struktur: &HashMap<GebietNr, Gebiet>,
) -> Result<(Bund, HashMap<GruppeNr, String>)> {
    let mut wahlkreise_parteien: HashMap<
        GebietNr,
        HashMap<GebietNr, HashMap<GruppeNr, ParteiWahlkreis>>,
    > = Default::default();
    let mut laender_parteien: HashMap<GebietNr, HashMap<GruppeNr, ParteiLand>> = Default::default();
    let mut bund_parteien: HashMap<GruppeNr, ParteiBund> = Default::default();

    let mut bund_laender: Vec<(String, GebietNr)> = Default::default();
    let mut laender_wahlkreise: HashMap<GebietNr, Vec<(String, GebietNr)>> = Default::default();

    let mut parteinr_name: HashMap<GruppeNr, String> = Default::default();

    for ge in stimmen.gebietsergebnisse.iter() {
        match ge.gebietsart {
            Gebietsart::BUND => {
                // collect parties in vector
                bund_parteien = ge
                    .ergebnisse
                    .iter()
                    .filter_map(|i| match i {
                        Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                            if let Gruppenart::PARTEI = gruppenergebnis.gruppenart {
                                Some(gruppenergebnis)
                            } else {
                                None
                            }
                        }
                        Ergebnis::Direktergebnis(_) => None,
                    })
                    .filter_map(|i| ParteiBund::try_from(i).ok().map(|x| (i.gruppe, x)))
                    .collect::<HashMap<_, _>>();
            }
            Gebietsart::LAND => {
                println!("{:#?}", ge);
                // register in parent structure (bund)
                bund_laender.push((ge.gebiet_text.to_owned(), ge.gebietsnummer));
                // collect parties in vector
                laender_parteien.insert(
                    ge.gebietsnummer,
                    ge.ergebnisse
                        .iter()
                        .filter_map(|i| match i {
                            Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                                if let Gruppenart::PARTEI = gruppenergebnis.gruppenart {
                                    Some(gruppenergebnis)
                                } else {
                                    None
                                }
                            }
                            Ergebnis::Direktergebnis(_) => None,
                        })
                        .filter_map(|i| ParteiLand::try_from(i).ok().map(|x| (i.gruppe, x)))
                        .collect::<HashMap<_, _>>(),
                );
            }
            Gebietsart::WAHLKREIS => {
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
                    .entry(ge.ueg_gebietsnummer.context("no ueg_gebietsnummer")?)
                    .or_default()
                    .insert(
                        ge.gebietsnummer,
                        ge.ergebnisse
                            .iter()
                            .filter_map(|i| match i {
                                Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                                    if let Gruppenart::PARTEI = gruppenergebnis.gruppenart {
                                        Some(gruppenergebnis)
                                    } else {
                                        None
                                    }
                                }
                                Ergebnis::Direktergebnis(_) => None,
                            })
                            .filter_map(|i| {
                                ParteiWahlkreis::try_from(i).ok().map(|x| (i.gruppe, x))
                            })
                            .collect::<HashMap<_, _>>(),
                    );
            }
        }

        parteinr_name.extend(ge.ergebnisse.iter().filter_map(|i| match i {
            Ergebnis::Gruppenergebnis(gruppenergebnis) => {
                if let Gruppenart::PARTEI = gruppenergebnis.gruppenart {
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
