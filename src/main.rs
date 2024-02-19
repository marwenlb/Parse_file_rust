mod log_parser; 
mod file_io; 
mod log_analysis; 
mod output; 
use clap::{App, Arg};
fn main() {
    let matches = App::new("Log Analyzer CLI")
        .version("1.0")
        .author("Your Name")
        .about("Analyzes log files")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input log file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FORMAT")
                .help("Sets the output format (plain text, CSV)")
                .takes_value(true)
                .possible_values(&["plain", "csv"])
                .default_value("plain"),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_format = matches.value_of("output").unwrap();
    let log_entries = match file_io::read_log_file(input_file) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading log file: {}", err);
            return;
        }
    };
    match output_format {
        "plain" => {
            if let Err(err) = output::output_plain_format(&log_entries) {
                eprintln!("Error while outputting plain format: {}", err);
            }
        }
        "csv" => {
            if let Err(err) = output::output_csv_format(&log_entries) {
                eprintln!("Error while outputting CSV format: {}", err);
            }
        }
        _ => {
            eprintln!("Invalid output format specified");
        }
    }
}
