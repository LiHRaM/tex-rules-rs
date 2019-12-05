use glob::glob;
use memmem::Searcher;
use memmem::TwoWaySearcher;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use anyhow::Result;

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
                    println!("{}:{} {:?}", file_name, i+1, &line)
                }
            }
        }
    }
    match succ {
        true => Ok(()),
        false => Err(anyhow::anyhow!("Errors in {:?}", &path))
    }
}

pub fn parse_line(line: &str) -> Result<(), ()> {
    let line = line.as_bytes();
    let search = TwoWaySearcher::new("%".as_bytes());
    let res = search.search_in(line);
    let line = match res {
        None => line,
        Some(i) => &line[..i],
    };
    let search = TwoWaySearcher::new(". ".as_bytes());
    let res = search.search_in(line);

    match res {
        None => Ok(()),
        Some(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bad_file() {
        let file = include_str!("files/bad.tex");
        let result = parse_line(file);
        assert!(result.is_err());
    }

    #[test]
    fn test_good_file() {
        let file = include_str!("files/good.tex");
        let result = parse_line(file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_comments_file() {
        let file = include_str!("files/comments.tex");
        let result = parse_line(file);
        assert!(result.is_ok());
    }
}
