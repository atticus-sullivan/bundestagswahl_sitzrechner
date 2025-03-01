use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, HashSet};
use log::debug;

use crate::types::{Bund, GruppeNr, Land, ParteiBund};

use crate::sls::sls;
use crate::wahl;

pub fn calc(bund: Bund, parteinr_name: &BTreeMap<GruppeNr, String>) -> Result<(BTreeMap<GruppeNr, u64>, u64)> {
    let total_seats = 598;
    let direktmandate = wahl::wahlkreismandate(&bund);

    let keep = parteinr_name.iter().filter_map(|(nr,name)| {
        if name == "SSW" {
            Some(*nr)
        } else {
            None
        }
    }).collect::<HashSet<_>>();

    let bund = wahl::huerde(bund, &direktmandate, 3, 0.05, keep)?;

    let sk = sitzkontingent(&bund.laender, total_seats)?;
    debug!("1.Oberverteilung: {:#?}", sk);

    let uv = unterverteilung(&bund.laender, &sk)?;
    debug!("1.Unterverteilung: {:#?}", uv);

    let msa = mindestsitzzahl(&bund.laender, &bund.parteien, &uv, &direktmandate)?;
    debug!("Mindestsitzzahlen: {:#?}", msa);

    let (fin, seats) = oberverteilung(&bund.parteien, &msa)?;

    Ok((fin, seats))
}

// also called 1.Oberverteilung
// returns land -> kontingent
fn sitzkontingent(laender: &Vec<Land>, base_seats: u64) -> Result<BTreeMap<usize, u64>> {
    let dist = sls(
        laender
            .iter()
            .enumerate()
            .map(|(i, j)| (i, j.einwohner))
            .collect(),
        base_seats,
    );

    Ok(dist)
}

fn unterverteilung(
    laender: &Vec<Land>,
    sk: &BTreeMap<usize, u64>,
) -> Result<BTreeMap<usize, BTreeMap<GruppeNr, u64>>> {
    let mut uv = BTreeMap::new();

    for (li, l) in laender.iter().enumerate() {
        let dist: BTreeMap<i16, u64> = sls(
            l.parteien
                .iter()
                .map(|(i, j)| -> Result<(i16, f64)> {
                    Ok((
                        *i,
                        j.zweitstimmen.with_context(|| {
                            format!("no zweitstimmen set for partei {i} in land {li}")
                        })? as f64,
                    ))
                })
                .collect::<Result<_>>()?,
            sk[&li],
        );
        uv.insert(li, dist);
    }
    Ok(uv)
}

fn mindestsitzzahl(
    laender: &Vec<Land>,
    parteien_bund: &BTreeMap<GruppeNr, ParteiBund>,
    uv: &BTreeMap<usize, BTreeMap<GruppeNr, u64>>,
    direktmandate: &Vec<BTreeMap<GruppeNr, u64>>,
) -> Result<BTreeMap<GruppeNr, u64>> {
    let mut msa = BTreeMap::new();

    for (i, _) in parteien_bund.iter() {
        let mut msz = 0;
        for (j, _) in laender.iter().enumerate() {
            // makes sense as eg no entry for CSU in all laender except for Bayern
            let skv_p = *uv[&j].get(i).unwrap_or(&0);
            let dm_p = *direktmandate[j].get(i).unwrap_or(&0);
            let mean = ((skv_p + dm_p) as f64 / 2.0).round() as u64;
            msz += mean.max(dm_p);
            // println!("Partei {i} | Land {j} | sk {skv_p} | dm {dm_p} | m {mean} | mx {} | cum {msz}", mean.max(dm_p))
        }
        // msa.insert(*i, msz.max(skv));
        msa.insert(*i, msz);
    }
    Ok(msa)
}

fn oberverteilung(
    parteien_bund: &BTreeMap<GruppeNr, ParteiBund>,
    msa: &BTreeMap<GruppeNr, u64>,
) -> Result<(BTreeMap<GruppeNr, u64>, u64)> {
    let total_seats: u64 = msa.iter().map(|(_, v)| v).sum();
    for total_seats in total_seats.. {
        let dist = sls(
            parteien_bund
                .iter()
                .map(|(k, p)| -> Result<(i16, f64)> {
                    Ok((
                        *k,
                        p.zweitstimmen
                            .with_context(|| format!("no zweitstimmen set for {k}"))?
                            as f64,
                    ))
                })
                .collect::<Result<_>>()?,
            total_seats,
        );
        let ueberhang = msa
            .iter()
            .map(|(i, a)| (*i, (*a as i64 - dist[i] as i64).max(0) as u64))
            .collect::<BTreeMap<_, _>>();
        let ueberhang_cnt = ueberhang.iter().map(|(_, u)| u).sum::<u64>();

        if ueberhang_cnt <= 3 {
            return Ok((
                dist.iter().map(|(i, s)| (*i, s + ueberhang[i])).collect(),
                total_seats + ueberhang_cnt,
            ));
        }
    }
    bail!("Bundestag grew over u64::max");
}
