use std::collections::BTreeMap;

use anyhow::Result;

use crate::types::GruppeNr;

pub fn banzhaf(verteilung: &BTreeMap<GruppeNr, u64>) -> Result<BTreeMap<GruppeNr, f64>> {
    // how many votes need to win a vote
    let quorum = (verteilung.values().sum::<u64>() as f64 / 2 as f64).ceil() as u64;

    // all kinds of koalitionen
    let potenzmenge = generate_power_set(&verteilung.keys().cloned().collect::<Vec<GruppeNr>>());

    // calculate what koalitionen win the vote
    let winning_coalition = potenzmenge.iter().filter_map(|i| {
        let votes = i.iter().map(|j| verteilung[j]).sum::<u64>();
        if votes >= quorum {
            Some((votes, i))
        } else {
            None
        }
    });

    // calculate critical fraktionen
    let mut fraktion_banzhaf_macht: BTreeMap<GruppeNr, u64> = Default::default();
    for (votes, w) in winning_coalition {
        for f in w.iter() {
            if votes - verteilung[f] < quorum {
                *fraktion_banzhaf_macht.entry(*f).or_insert(0) += 1;
            }
        }
    }

    // calculate banzhaf
    let total = fraktion_banzhaf_macht.values().sum::<u64>() as f64;

    Ok(fraktion_banzhaf_macht
        .iter()
        .map(|(p, macht)| (*p, *macht as f64 / total))
        .collect())
}

// Function to generate the power set (all combinations of elements)
fn generate_power_set<T>(elements: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut power_set = Vec::new();
    let n = elements.len();

    // Iterate through all numbers from 0 to 2^n - 1 to generate all subsets
    for i in 0..(1 << n) {
        let mut subset = Vec::new();

        for j in 0..n {
            // Check if the j-th element should be included in the subset
            if i & (1 << j) != 0 {
                subset.push(elements[j].clone());
            }
        }

        power_set.push(subset);
    }

    power_set
}
