use std::fs::File;
use std::io::{BufRead, BufReader};
use super::log_parser::parse_log_line;
#[derive(Debug)]
pub struct LogEntry {
    pub request_type: String,
    pub endpoint_url: String,
    pub status_code: u16,
    pub response_time_ms: u32,
}
// Reads a log file and parses its contents into a vector of LogEntry structs.
pub fn read_log_file(input_file: &str) -> Result<Vec<LogEntry>, String> {
    let mut log_entries: Vec<LogEntry> = Vec::new();

    let file = match File::open(input_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("failed to open file: {}", err)),
    };
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                match parse_log_line(&line) {
                    Ok(log) => log_entries.push(log),
                    Err(err) => {
                        eprintln!("error parsing line: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("error reading log file: {}", err);
                return Err(format!("error reading log file: {}", err));
            }
        }
    }
    Ok(log_entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use tempfile::TempDir;
    #[test]
    fn not_empty_log_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("log.txt");
        File::create(&file_path).unwrap();
        let result = read_log_file(file_path.to_str().unwrap());
        assert!(result.is_ok());
        let log_entries = result.unwrap();
        assert!(log_entries.is_empty());
    }
}