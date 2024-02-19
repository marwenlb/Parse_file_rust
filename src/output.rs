use std::fs::File;
use std::io::{Write};
use super::file_io::LogEntry;
use super::log_analysis::{count_request_types, analyze_errors, calculate_avg_response_time};

pub fn output_plain_format(log_entries: &[LogEntry]) -> Result<(), String> {
    let mut file = match File::create("output.txt") {
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating text file: {}", err)),
    };
    // Write Request Summary
    writeln!(&mut file, "Request Summary:").map_err(|e| format!("Error writing to file: {}", e))?;
    for (request_type, count) in count_request_types(&log_entries) {
        writeln!(&mut file, "{}: {}", request_type, count).map_err(|e| format!("Error writing to file: {}", e))?;
    }

    // Write Error Analysis
    writeln!(&mut file, "\nError Analysis:").map_err(|e| format!("Error writing to file: {}", e))?;
    for ((endpoint_url, status_code), count) in analyze_errors(&log_entries) {
        writeln!(&mut file, "Endpoint: {}, Status Code: {}: {}", endpoint_url, status_code, count).map_err(|e| format!("Error writing to file: {}", e))?;
    }

    // Write Performance Metrics
    writeln!(&mut file, "\nPerformance Metrics:").map_err(|e| format!("Error writing to file: {}", e))?;
    for (endpoint_url, avg_response_time) in calculate_avg_response_time(&log_entries) {
        writeln!(&mut file, "Endpoint: {}, Avg. Response Time: {} ms", endpoint_url, avg_response_time).map_err(|e| format!("Error writing to file: {}", e))?;
    }

    Ok(())
}

pub fn output_csv_format(log_entries: &[LogEntry]) -> Result<(), String> {
    let mut file = match File::create("output.csv") {
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating CSV file: {}", err)),
    };
    // Write Request Summary
    if let Err(err) = writeln!(&mut file, "Request Summary") {
        return Err(format!("Error writing Request Summary section: {}", err));
    }
    for (request_type, count) in count_request_types(&log_entries).iter() {
        if let Err(err) = writeln!(&mut file, "{},{}", request_type, count) {
            return Err(format!("Error writing request summary data: {}", err));
        }
    }
    // Write Error Analysis
    if let Err(err) = writeln!(&mut file, "\nError Analysis") {
        return Err(format!("Error writing Error Analysis section: {}", err));
    }
    for ((endpoint_url, status_code), count) in analyze_errors(&log_entries).iter() {
        if let Err(err) = writeln!(&mut file, "{},{},{}", endpoint_url, status_code, count) {
            return Err(format!("Error writing error analysis data: {}", err));
        }
    }
    // Write Performance Metrics
    if let Err(err) = writeln!(&mut file, "\nPerformance Metrics") {
        return Err(format!("Error writing Performance Metrics section: {}", err));
    }
    for (endpoint_url, avg_response_time) in calculate_avg_response_time(&log_entries).iter() {
        if let Err(err) = writeln!(&mut file, "{},{}", endpoint_url, avg_response_time) {
            return Err(format!("Error writing performance metrics data: {}", err));
        }
    }

    Ok(())
}


