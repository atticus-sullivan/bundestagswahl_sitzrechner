mod parsing;
mod parsing_types;
mod sls;
mod types;

use anyhow::{anyhow, bail, ensure, Context, Result};

fn main() -> Result<()> {
    let stimmen = parsing::parse_xml()?;
    let struktur = parsing::parse_csv()?;

    let (bund, parteinr_name) = types::convert_data(stimmen, &struktur)?;

    println!("{:#?}", bund);
    // println!("{:#?}", bund_laender);
    println!("{:#?}", parteinr_name);
    // TODO eigentlichen objekte müssen noch gebaut werden (vektoren können gemoved werden)

    Ok(())
}
