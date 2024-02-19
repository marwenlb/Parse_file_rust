use crate::file_io::LogEntry;
pub fn parse_log_line(line: &str) -> Result<LogEntry, &'static str> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Check if the entry has enough fields
    if parts.len() < 6 {
        return Err("Missing fields in log entry");
    }

    // Extract fields
    let request_type = parts[2].to_string();
    let endpoint_url = parts[3].to_string();
    let status_code = parts[4].parse::<u16>().unwrap_or(0);
    let response_time_ms = parts[5].parse::<u32>().unwrap_or(0);

    // Return LogEntry 
    Ok(LogEntry {
        request_type,
        endpoint_url,
        status_code,
        response_time_ms,
    })
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_line_valid() {
        let logs = [
            "[2024-07-05 05:13:35] POST /user/session 404 54",
            "[2024-08-22 09:07:58] DELETE /user/profile 403 317",
            "[2024-09-19 10:37:55] GET /api/update 403 119",
            "[2024-07-01 02:21:47] PUT /contact 403 381",
        ];

        // Expected LogEntry instances
        let expected_entries = [
            LogEntry {
                request_type: "POST".to_string(),
                endpoint_url: "/user/session".to_string(),
                status_code: 404,
                response_time_ms: 54,
            },
            LogEntry {
                request_type: "DELETE".to_string(),
                endpoint_url: "/user/profile".to_string(),
                status_code: 403,
                response_time_ms: 317,
            },
            LogEntry {
                request_type: "GET".to_string(),
                endpoint_url: "/api/update".to_string(),
                status_code: 403,
                response_time_ms: 119,
            },
            LogEntry {
                request_type: "PUT".to_string(),
                endpoint_url: "/contact".to_string(),
                status_code: 403,
                response_time_ms: 381,
            },
        ];

        for (log, expected_entry) in logs.iter().zip(expected_entries.iter()) {
            let result = parse_log_line(log);
            assert!(result.is_ok());

            let log_entry = result.unwrap();
            assert_eq!(log_entry.request_type, expected_entry.request_type);
            assert_eq!(log_entry.endpoint_url, expected_entry.endpoint_url);
            assert_eq!(log_entry.status_code, expected_entry.status_code);
            assert_eq!(log_entry.response_time_ms, expected_entry.response_time_ms);
        }
    }
}

