use anyhow::Result;
use glob::glob;
use memmem::Searcher;
use memmem::TwoWaySearcher;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub fn find_tex_files() -> impl Iterator<Item = PathBuf> {
    glob("./**/*.tex")
        .expect("invalid glob pattern")
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
}

pub fn parse_from_path(path: &PathBuf) -> Result<()> {
    let file_name = format!("Error: {}", path.to_str().expect("Invalid UTF-8 in path"));
    let mut succ = true;
    let file = File::open(path).expect("Invalid path from glob");
    let buf_reader = BufReader::new(file);
    for (i, line) in buf_reader.lines().enumerate() {
        if let Ok(line) = line {
            match parse_line(&line) {
                Result::Ok(_) => (),
                Result::Err(_) => {
                    succ = false;
                    println!("{}:{} {:?}", file_name, i + 1, &line)
                }
            }
        }
    }
    if succ {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Errors in {:?}", &path))
    }
}

pub fn parse_line(line: &str) -> Result<(), ()> {
    let line = line.as_bytes();
    let search = TwoWaySearcher::new(b"%");
    let res = search.search_in(line);
    let line = match res {
        None => line,
        Some(i) => &line[..i],
    };
    let search = TwoWaySearcher::new(b". ");
    let res = search.search_in(line);

    match res {
        None => Ok(()),
        Some(i) => {
            let line = String::from_utf8(line[i + 1..].to_vec()).expect("invalid utf8");
            if line.trim_end().is_empty() {
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bad_file() {
        let line = "This document writes several sentences on one line. Like. This.";
        let result = parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_good_file() {
        let line = "This is a normal line.";
        let result = parse_line(line);
        assert!(result.is_ok());
    }

    #[test]
    fn test_comments_file() {
        let line = "% Nobody. Cares. About. Comments.";
        let result = parse_line(line);
        assert!(result.is_ok());
    }

    #[test]
    fn test_whitespace_line() {
        let line = "This should be fine. ";
        let result = parse_line(line);
        assert!(result.is_ok());
    }
}
