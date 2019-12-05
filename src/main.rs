use anyhow::Result;

mod lib;
fn main() -> Result<()>  {
    let mut succ = true;
    for path in lib::find_tex_files() {
        if lib::parse_from_path(&path).is_err() {
            succ = false;
        }
    }
    match succ {
        true => Ok(()),
        false => Err(anyhow::anyhow!("Errors were detected.")),
    }
}