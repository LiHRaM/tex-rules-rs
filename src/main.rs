use anyhow::Result;
use rayon::prelude::*;

mod lib;
fn main() -> Result<()> {
    let errored = lib::find_tex_files()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|path| lib::parse_from_path(&path).is_err())
        .reduce(|| false, |acc, elem| acc && elem);

    if !errored {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Errors were detected."))
    }
}
