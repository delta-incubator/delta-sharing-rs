use std::fs::remove_file;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn read_lines(path: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(&path)?;
    Ok(BufReader::new(file).lines())
}

pub fn tempfile(content: &str) -> Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "{}", content)?;
    Ok(file)
}

pub fn persist<'a>(content: &'a str, path: &'a Path) -> Result<&'a Path> {
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    Ok(&path)
}

pub fn remove(path: &Path) -> Result<()> {
    for _try in 1..=4 {
        match remove_file(&path) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                if _try < 4 {
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}
