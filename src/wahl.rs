use std::collections::BTreeMap;

use anyhow::{Context, Result};

use crate::types::{Bund, GruppeNr, Land, ParteiWahlkreis};

fn count_keys(vec: Vec<Option<(&i16, &ParteiWahlkreis)>>) -> BTreeMap<i16, u64> {
    let mut map: BTreeMap<i16, u64> = BTreeMap::new();

    for item in vec {
        if let Some((key, _)) = item {
            *map.entry(*key).or_insert(0) += 1;
        }
    }

    map
}

pub fn wahlkreismandate(bund: &Bund) -> Vec<BTreeMap<GruppeNr, u64>> {
    bund.laender
        .iter()
        .map(|l| {
            let wkm = l
                .wahlkreise
                .iter()
                .map(|wk| wk.parteien.iter().max_by_key(|(_, p)| p.erststimmen))
                .collect::<Vec<_>>();
            count_keys(wkm)
        })
        .collect()
}

pub fn huerde(
    bund: Bund,
    wkm: &Vec<BTreeMap<GruppeNr, u64>>,
    wkm_huerde: u64,
    prozent_huerde: f64,
) -> Result<Bund> {
    let total_votes_bund = bund
        .parteien
        .iter()
        .filter_map(|(_, p)| p.zweitstimmen)
        .sum::<u64>() as f64;

    let total_wkm =
        wkm.iter()
            .flat_map(|wkm| wkm.iter())
            .fold(BTreeMap::new(), |mut acc, (&i, &wkm)| {
                *acc.entry(i).or_insert(0) += wkm;
                acc
            });

    let parteien_bund: BTreeMap<_, _> = bund
        .parteien
        .into_iter()
        .filter(|(i, p)| match p.zweitstimmen {
            Some(z) => {
                z as f64 / total_votes_bund >= prozent_huerde
                    || total_wkm.get(i).unwrap_or(&0) >= &wkm_huerde
            }
            None => false,
        })
        .collect();

    let laender_bund = bund
        .laender
        .into_iter()
        .map(|l| {
            Land {
                name: l.name,
                einwohner: l.einwohner,
                parteien: l
                    .parteien
                    .into_iter()
                    .filter(|(i, _)| parteien_bund.contains_key(i))
                    .collect(),
                wahlkreise: l.wahlkreise, // FIXME ATTENTION: The parteien in the wahlkreise are not filtered (yet)
            }
        })
        .collect();

    Ok(Bund {
        parteien: parteien_bund,
        laender: laender_bund,
    })
}
