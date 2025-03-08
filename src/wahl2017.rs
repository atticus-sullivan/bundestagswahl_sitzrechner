// SPDX-FileCopyrightText: 2025 Lukas Heindl
//
// SPDX-License-Identifier: MIT

use anyhow::{bail, Context, Result};
use log::debug;
use std::collections::{BTreeMap, HashSet};

use crate::types::{Bund, GruppeNr, Land, ParteiBund};

// reuse some functions from other schemes
use crate::wahl2021::sitzkontingent;
use crate::wahl2021::unterverteilung;

use crate::sls::sls;
use crate::wahl;

// used (legal) references:
// [1]: docs/2017_bundeswahlgesetz.pdf # TODO

// TODO
// references to the legal text are just copied from 2021 so they are probably wrong
pub fn calc(
    bund: Bund,
    parteinr_name: &BTreeMap<GruppeNr, String>,
) -> Result<(BTreeMap<GruppeNr, (u64, u64)>, u64, Bund)> {
    // [1] -> § 1 Abs.1 Satz 1
    let total_seats = 598;
    let direktmandate = wahl::wahlkreismandate(&bund);

    let total_dm = wahl::sum_total_wahlkreismandate(&direktmandate);

    // [1] -> § 6 Abs.1
    // TODO potential reduction of total seats (independant candidates)

    // [1] -> § 6 Abs.3
    // Parteien nationaler Minderheiten sind von der 5-Prozent-/3-Direktmandats-Huerde ausgenommen
    let keep = parteinr_name
        .iter()
        .filter_map(|(nr, name)| if name == "SSW" { Some(*nr) } else { None })
        .collect::<HashSet<_>>();

    // [1] -> § 6 Abs.3
    // Bei Verteilung der Sitze auf Landeslisten -> nur Parteien >= 3 Direktmandate oder >= 5%
    // Zweitstimmen
    let bund = wahl::huerde(bund, &total_dm, 3, 0.05, keep)?;

    // [1] -> § 6 Abs.2 Satz 1f
    // Gesamtzahl der Sitze werden auf die Länder anhand der Bevölkerung aufgeteilt
    let sk = sitzkontingent(&bund.laender, total_seats)?;
    debug!("1.Oberverteilung: {:#?}", sk);

    // [1] -> § 6 Abs.2 Satz 1f
    // Dem Land zugewiesene Sitze werden auf die Parteien anhand der Zweitstimmen in diesem Land
    // aufgeteilt
    let uv = unterverteilung(&bund.laender, &sk)?;
    debug!("1.Unterverteilung: {:#?}", uv);

    // [1] -> § 6 Abs.5 Satz 2
    // max aus direktmandaten und mittelwert aus direktmandaten und (1.)Unterverteilung => sum
    let msz = mindestsitzzahl(&bund.laender, &bund.parteien, &uv, &direktmandate)?;
    debug!("Mindestsitzzahlen: {:#?}", msz);

    // [1] -> § 6 Abs.5+6
    // Gesamtzahl der Sitze wird so lange erhöht, bis jede Partei (bei der Verteilung nach den
    // bundesweiten Zweitstimmen) mindestens so viele Sitze bekommt wie ihr nach Mindestsitzzahl
    // zusteht. Jedoch können dabei bis zu 3 Ueberhangsmandate unausgeglichen bleiben.
    let (fin, seats) = oberverteilung(&bund.parteien, &msz)?;

    // [1] -> § 6 Abs.7
    // TODO potential +x seats due to >50% votes but not >50% seats

    // include amount of wahlkreismandate
    let ret = fin
        .into_iter()
        .map(|(p, s)| (p, (*total_dm.get(&p).unwrap_or(&0), s)))
        .collect::<BTreeMap<_, _>>();

    Ok((ret, seats, bund))
}

/// Berechnet die *Mindestsitzzahl* für jede Partei `parteien_bund` auf Basis der vorangegangen
/// (Unter)verteilung `uv` und der `direktmandate` der Partei
///
/// returns `parteiNr -> Sitze`
fn mindestsitzzahl(
    laender: &[Land],
    parteien_bund: &BTreeMap<GruppeNr, ParteiBund>,
    uv: &BTreeMap<usize, BTreeMap<GruppeNr, u64>>,
    direktmandate: &[BTreeMap<GruppeNr, u64>],
) -> Result<BTreeMap<GruppeNr, u64>> {
    let mut msa = BTreeMap::new();

    for i in parteien_bund.keys() {
        let mut msz = 0;
        for (j, _) in laender.iter().enumerate() {
            // get -> unwrap_or makes sense as eg no entry for CSU in all laender except for Bayern
            // sitzkontingent der Partei im Land
            let skv_p = *uv[&j].get(i).unwrap_or(&0);
            // direktmandate der Partei im Land
            let dm_p = *direktmandate[j].get(i).unwrap_or(&0);

            // [1] -> § 6 Abs.5 Satz 2
            // "höhere Wert aus [...] der Zahl der Im Land [...] errungenen [Direktmandate] oder dem
            // [...] Mittelwert [(siehe oben)] [...]"
            msz += skv_p.max(dm_p);
        }
        // [1] -> § 6 Abs.5 Satz 3
        // "Jede Partei erhält mindestens die bei der ersten Verteilung [...] für ihre Landesliste
        // ermittelten ermittelten Sitze"
        msa.insert(*i, msz);
    }
    Ok(msa)
}

/// Berechnet die *(2.)Oberverteilung* für jede Partei `parteien_bund` auf Basis der bundesweiten
/// Zweitstimmen und der jeweiligen Mindestsitzzahl `msz`
///
/// returns `parteiNr -> Sitze`
fn oberverteilung(
    parteien_bund: &BTreeMap<GruppeNr, ParteiBund>,
    msz: &BTreeMap<GruppeNr, u64>,
) -> Result<(BTreeMap<GruppeNr, u64>, u64)> {
    let total_seats: u64 = msz.values().sum();
    // [1] -> § 6 Abs.5
    // "Die Zahl der [...] Sitze wird so lange erhöht, bis [...]"
    for total_seats in total_seats.. {
        // [1] -> § 6 Abs.5 Satz 1
        // "[...] bei der zweiten Verteilung der Sitze [...]"
        // [1] -> § 6 Abs.6 Satz 1
        // "[...] Sitze werden [...] bundesweit nach der Zahl der [...] Zweitstimmen [...] auf die
        // [...] Parteien verteilt"
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
        )?;

        let unausgeglichener_ueberhang = msz
            .iter()
            .map(|(i, a)| (*i, (*a as i64 - dist[i] as i64).max(0) as u64))
            .collect::<BTreeMap<_, _>>();
        let unausgeglichener_ueberhang_cnt = unausgeglichener_ueberhang.values().sum::<u64>();

        // [1] -> § 6 Abs.5 Satz 4
        // "Bei der Erhöhung bleiben in den Wahlkreisen errungene Sitze [...] bis zu einer Zahl von
        // drei unberücksichtigt"
        if unausgeglichener_ueberhang_cnt == 0 {
            return Ok((
                dist,
                // [1] -> § 6 Abs.5 Satz 5
                // "die Gesamtzahl der Sitze [...] erhöht sich um die Unterschiedszahl"
                total_seats,
            ));
        }
    }
    bail!("Bundestag grew over u64::max");
}
