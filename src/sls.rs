use std::{collections::BTreeMap, fmt::Display};

pub fn sls<T>(xs: BTreeMap<T, f64>, total: u64) -> BTreeMap<T, u64>
where
    T: Ord + Clone + Display,
{
    let mut distribution: BTreeMap<T, u64> = BTreeMap::new();

    let mut zut_div: f64 = xs.iter().map(|x| x.1).sum::<f64>() / total as f64;
    // println!(
    //     "sls start: {zut_div} | {total} | {}",
    //     xs.iter().map(|x| x.1).sum::<f64>()
    // );

    // find a working divisor
    let mut s = distribution.iter().map(|x| x.1).sum();
    let mut diff = 1.0;
    let mut last = (0.0, 0.0);
    while s != total {
        if s < total {
            zut_div -= diff;
        } else {
            zut_div += diff;
        }
        if (last.1 - zut_div).abs() < 10e-10_f64 {
            diff *= 0.5;
        }
        last = (zut_div, last.0);

        for (id, x) in xs.iter() {
            distribution.insert(id.clone(), (*x / zut_div).round() as u64);
        }

        s = distribution.iter().map(|x| x.1).sum::<u64>();
        // println!();
    }

    // println!("sls res: {zut_div}");
    distribution
}
