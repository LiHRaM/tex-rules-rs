use anyhow::Result;
use rayon::prelude::*;

mod lib;
fn main() -> Result<()> {
    let errored = parse_from_current_path();

    if !errored {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Errors were detected."))
    }
}

fn parse_from_current_path() -> bool {
    lib::find_tex_files()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|path| lib::parse_from_path(&path).is_err())
        .reduce(|| false, |acc, elem| acc || elem)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reg_error_found() {
        let errored = parse_from_current_path();
        assert_eq!(errored, true);
    }
}