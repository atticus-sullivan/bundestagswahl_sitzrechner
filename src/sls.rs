use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
struct PartyCumulated {
    zweitstimmen: u64,
    mindestsitzzahl: u64,
    seats: u64,
}

#[derive(Clone)]
struct Party {
    zweitstimmen: u64,
    sitzkontingent_verteilung: u64,
    direktmandate: u64,
}

#[derive(Clone)]
struct State {
    population: u64,
    sitzkontingent: u64,
    parties: Vec<Party>,
}

#[derive(Clone)]
struct SLS {
    base_seats: u64,
    parties: Vec<PartyCumulated>,
    states: Vec<State>,
}

impl SLS {
    fn calc_sitzkontingent(self: &mut Self) {
        let dist = sls(
            self.states
                .iter()
                .enumerate()
                .map(|(i, j)| (i, j.population))
                .collect(),
            self.base_seats,
        );

        for (k, seats) in dist.iter() {
            self.states[*k].sitzkontingent = *seats;
        }
    }

    fn calc_unterverteilung(self: &mut Self) {
        for s in self.states.iter_mut() {
            let dist = sls(
                s.parties
                    .iter()
                    .enumerate()
                    .map(|(i, j)| (i, j.zweitstimmen))
                    .collect(),
                s.sitzkontingent,
            );

            for (k, seats) in dist.iter() {
                s.parties[*k].sitzkontingent_verteilung = *seats;
            }
        }
    }

    fn calc_mindestsitzzahlen(self: &mut Self) {
        // 2017
        for (i, p) in self.parties.iter_mut().enumerate() {
            p.mindestsitzzahl = self
                .states
                .iter()
                .map(|s| {
                    let pl = &s.parties[i];
                    pl.sitzkontingent_verteilung.max(pl.direktmandate) // mindestsitzzahl
                })
                .sum();
        }
    }

    fn calc_oberverteilung(self: &mut Self) {
        // 2017
        let div = self
            .parties
            .iter()
            .map(|p| {
                // TODO fixpoint?
                (p.zweitstimmen as f64) / (p.mindestsitzzahl as f64 - 0.5)
            })
            .fold(f64::INFINITY, |a, b| a.min(b));

        for p in self.parties.iter_mut() {
            // TODO fixpoint?
            p.seats = (p.zweitstimmen as f64 / div).round() as u64;
        }
    }
}

fn sls<T>(xs: HashMap<T, u64>, total: u64) -> HashMap<T, u64>
where
    T: Hash + Eq + Clone,
{
    let mut distribution: HashMap<T, u64> = HashMap::new();

    // TODO use https://docs.rs/fixed/latest/fixed/ here?
    let mut zut_div = xs.iter().map(|x| x.1).sum::<u64>() as f64 / total as f64;

    let mut s = distribution.iter().map(|x| x.1).sum::<u64>();
    while s != total {
        if s > total {
            zut_div -= 1.0;
        } else {
            zut_div += 1.0;
        }

        for (id, x) in xs.iter() {
            distribution.insert(id.clone(), (*x as f64 / zut_div).round() as u64);
        }

        s = distribution.iter().map(|x| x.1).sum::<u64>();
    }
    distribution
}
