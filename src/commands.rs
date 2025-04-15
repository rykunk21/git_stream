use crate::git::GitHistory;
use polars::prelude::*;
use std::{error::Error, path::PathBuf};

pub fn init_repository(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // TODO: Initialize repository monitoring
    // - Create config file
    // - Set up initial database
    // - Validate git repository
    Ok(())
}

pub fn watch_repository(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // TODO: Implement repository watching
    // - Set up git hooks
    // - Start monitoring thread
    // - Process events
    Ok(())
}

pub fn export_repository(path: &PathBuf, format: &str) -> Result<(), Box<dyn Error>> {
    let git_history = GitHistory::new(path)?;
    let df = git_history.to_df(None, None)?;

    match format.to_lowercase().as_str() {
        "csv" => {
            let file = std::fs::File::create(path.join("git_history.csv"))?;
            // CsvWriter::new(&mut file).finish(&df)?;
            todo!();
        }
        "parquet" => {
            let file = std::fs::File::create(path.join("git_history.parquet"))?;
            // ParquetWriter::new(file).finish(&df)?;
            todo!();
        }
        "json" => {
            let file = std::fs::File::create(path.join("git_history.json"))?;
            // JsonWriter::new(&mut file).finish(&df)?;

            todo!();
        }
        _ => return Err("TODO!() -> Unsupported format. Use 'csv', 'parquet', or 'json'".into()),
    }

    Ok(())
}

pub fn query_repository(
    path: &PathBuf,
    since: Option<&str>,
    author: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let git_history = GitHistory::new(path)?;
    let df = git_history.to_df(since, author)?;

    // Print the DataFrame to stdout
    println!("{}", df);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_query_repository() {
        let path = PathBuf::from(".");
        let result = query_repository(&path, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_export_repository() {
        let path = PathBuf::from(".");
        let result = export_repository(&path, "csv");
        assert!(result.is_ok());
        let result = export_repository(&path, "parquet");
        assert!(result.is_ok());
        let result = export_repository(&path, "json");
        assert!(result.is_ok());
    }
}
