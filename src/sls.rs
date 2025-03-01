use std::{collections::BTreeMap, fmt::Display};

pub fn sls<T>(xs: BTreeMap<T, f64>, total: u64) -> BTreeMap<T, u64>
where
    T: Ord + Clone + Display,
{
    let mut distribution: BTreeMap<T, u64> = BTreeMap::new();

    // TODO use https://docs.rs/fixed/latest/fixed/ here?
    let mut zut_div: f64 = xs.iter().map(|x| x.1).sum::<f64>() / total as f64;
    println!(
        "sls start: {zut_div} | {total} | {}",
        xs.iter().map(|x| x.1).sum::<f64>()
    );

    let mut s = distribution.iter().map(|x| x.1).sum();
    while s != total {
        if s < total {
            zut_div -= 1.0;
        } else {
            zut_div += 1.0;
        }

        for (id, x) in xs.iter() {
            // println!("sls: {id} -> {}", *x/zut_div);
            distribution.insert(id.clone(), (*x / zut_div).round() as u64);
        }

        s = distribution.iter().map(|x| x.1).sum::<u64>();
        // println!();
    }
    println!("sls res: {zut_div}");
    distribution
}
