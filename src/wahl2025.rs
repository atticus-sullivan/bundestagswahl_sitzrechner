// SPDX-FileCopyrightText: 2025 Lukas Heindl
//
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
// use log::debug;
use std::collections::{BTreeMap, HashSet};

use crate::types::{Bund, GruppeNr, ParteiBund};

use crate::sls::sls;
use crate::wahl;

// used (legal) references:
// [1]: docs/2025_bundeswahlgesetz.pdf

pub fn calc(
    bund: Bund,
    parteinr_name: &BTreeMap<GruppeNr, String>,
) -> Result<(BTreeMap<GruppeNr, u64>, u64, Bund)> {
    // [1] -> § 1 Abs.1 Satz 1
    let total_seats = 630;
    let wahlkreismandate = wahl::wahlkreismandate(&bund);

    // [1] -> § 4 Abs.3
    // TODO potential reduction of total seats (independant candidates)

    // [1] -> § 4 Abs.2 Satz 3
    // Parteien nationaler Minderheiten sind von der 5-Prozent-/3-Direktmandats-Huerde ausgenommen
    let keep = parteinr_name
        .iter()
        .filter_map(|(nr, name)| if name == "SSW" { Some(*nr) } else { None })
        .collect::<HashSet<_>>();

    // [1] -> § 4 Abs.2 Satz 2 2.
    // Bei Verteilung der Sitze auf Landeslisten -> nur Parteien >= 3 Direktmandate oder >= 5%
    // Zweitstimmen
    let bund = wahl::huerde(bund, &wahlkreismandate, 3, 0.05, keep)?;

    // [1] -> § 4 Abs.2 Satz 1
    // Verteilung der Sitze auf die Parteien anhand Zweitstimmen
    // Bemerkung: "Wahlgebiet" meint hier das Bundesgebiet
    let ov = oberverteilung(&bund.parteien, total_seats)?;
    // debug!("Oberverteilung: {:#?}", ov);

    // nicht relevant sofern nur die Sitzverteilung von Interesse ist //
    // let uv = unterverteilung(&bund.parteien, &bund.laender, &ov)?;
    // debug!("Unterverteilung: {:#?}", uv);

    // nicht relevant sofern nur die Sitzverteilung von Interesse ist //
    // let zsd = zweitstimmendeckung()?;

    // [1] -> § 4 Abs.4
    // TODO potential +x seats due to >50% votes but not >50% seats

    Ok((ov, total_seats, bund))
}

/// Berechnet die *Oberverteilung* für die `parteien_bund` auf Basis der `base_seats` Sitze und der
/// bundesweit erzielten Zweitstimmen
///
/// returns `parteiNr -> Sitze`
fn oberverteilung(
    parteien_bund: &BTreeMap<GruppeNr, ParteiBund>,
    base_seats: u64,
) -> Result<BTreeMap<GruppeNr, u64>> {
    // [1] -> § 5 Abs.1 Satz 1
    // "die Zahl der [...] Zweitstimmen im Wahlgebiet [(Deutschland) wird] durch den [...]
    // Zuteilungsdivisor geteilt und das [Ergebnis] [...] gerundet"
    let dist = sls(
        parteien_bund
            .iter()
            .map(|(i, p)| -> Result<(GruppeNr, f64)> {
                Ok((
                    *i,
                    p.zweitstimmen
                        .with_context(|| format!("no zweitstimmen set for partei {i}"))?
                        as f64,
                ))
            })
            .collect::<Result<_>>()?,
        base_seats,
    );
    Ok(dist)
}

// /// Berechnet die *Unterverteilung* für jede Partei `parteien_bund` auf Basis der Oberverteilung
// /// `ov` für die jeweilien `laender`
// ///
// /// nicht relevant sofern nur die Sitzverteilung von Interesse ist
// ///
// /// returns `parteiNr -> landIdx -> Sitze`
// fn unterverteilung(
//     parteien_bund: &BTreeMap<GruppeNr, ParteiBund>,
//     laender: &Vec<Land>,
//     ov: &BTreeMap<GruppeNr, u64>,
// ) -> Result<BTreeMap<GruppeNr, BTreeMap<usize, u64>>> {
//     let mut uv: BTreeMap<GruppeNr, BTreeMap<usize, u64>> = Default::default();
//
//     for i in parteien_bund.keys() {
//         // [1] -> § 5 Abs.1 Satz 2
//         // "für jede Partei die Zahl der auf ihre Landesliste entfallenen Zweitstimmen durch den
//         // [...] Zuteilungsdivisor geteilt und das [Ergebnis] [...] gerundet"
//         let dist = sls(
//             laender
//                 .iter()
//                 .enumerate()
//                 .map(|(j, l)| -> Result<(usize, f64)> {
//                     Ok((
//                         j,
//                         l.parteien
//                             .get(i)
//                             .map_or(Some(0), |p| p.zweitstimmen)
//                             .with_context(|| {
//                                 format!("no zweitstimmen set for partei {i} in land {j}")
//                             })? as f64,
//                     ))
//                 })
//                 .collect::<Result<_>>()?,
//             ov[i],
//         );
//
//         uv.insert(*i, dist);
//     }
//
//     Ok(uv)
// }

// /// Berechnet die *Zweitstimmendeckung* ...
// ///
// /// nicht relevant sofern nur die Sitzverteilung von Interesse ist
// ///
// /// returns ``
// fn zweitstimmendeckung() -> Result<()> {
//     // [1] -> § 6 Abs.1
//     Ok(())
// }
