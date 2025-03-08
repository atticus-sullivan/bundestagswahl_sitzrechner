// SPDX-FileCopyrightText: 2025 Lukas Heindl
//
// SPDX-License-Identifier: MIT

use anyhow::Result;
use std::collections::BTreeMap;

use crate::types::{Bund, GruppeNr};

use crate::wahl;

pub fn calc(
    bund: Bund,
    _parteinr_name: &BTreeMap<GruppeNr, String>,
) -> Result<(BTreeMap<GruppeNr, (u64, u64)>, u64, Bund)> {
    // [1] -> § 1 Abs.1 Satz 1
    let direktmandate = wahl::wahlkreismandate(&bund);

    let total_dm = wahl::sum_total_wahlkreismandate(&direktmandate);

    // include amount of wahlkreismandate
    let ret = total_dm
        .into_iter()
        .map(|(p, s)| (p, (s, s)))
        .collect::<BTreeMap<_, _>>();

    let seats = ret.values().map(|(s, _)| s).sum::<u64>();

    Ok((ret, seats, bund))
}
