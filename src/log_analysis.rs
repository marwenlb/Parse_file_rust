use std::collections::HashMap;
pub use super::file_io::LogEntry;
// counts the occurrences of each request type
pub fn count_request_types(log_entries: &[LogEntry]) -> HashMap<String, usize> {
    let mut request_summary: HashMap<String, usize> = HashMap::new();
    for log in log_entries {
        *request_summary.entry(log.request_type.clone()).or_insert(0) += 1;
    }
    request_summary
}
// analyzes the occurrences of errors (status codes >= 400) == errors
pub fn analyze_errors(log_entries: &[LogEntry]) -> HashMap<(String, u16), usize> {
    let mut error_analysis: HashMap<(String, u16), usize> = HashMap::new();
    for log in log_entries {
        if log.status_code >= 400 {
            let key = (log.endpoint_url.clone(), log.status_code);
            *error_analysis.entry(key).or_insert(0) += 1;
        }
    }
    error_analysis
}
// calculates the average response time for each endpoint URL
pub fn calculate_avg_response_time(log_entries: &[LogEntry]) -> HashMap<String, u32> {
    let mut avg_response_time: HashMap<String, (u32, usize)> = HashMap::new();
    for log in log_entries {
        let entry = avg_response_time.entry(log.endpoint_url.clone()).or_insert((0, 0));
        entry.0 += log.response_time_ms;
        entry.1 += 1;
    }
    avg_response_time
        .into_iter()
        .map(|(endpoint, (total_time, count))| {
            (endpoint, total_time / count as u32)
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_avg_response_time() {
        // Create some test entries
        let log_entries = vec![
            LogEntry {
                endpoint_url: "/endpoint_test".to_string(),
                response_time_ms: 100,
                request_type: "GET".to_string(),
                status_code: 200,
            },
            LogEntry {
                endpoint_url: "/endpoint_test".to_string(),
                response_time_ms: 200,
                request_type: "GET".to_string(),
                status_code: 200,
            },
            LogEntry {
                endpoint_url: "/endpoint2".to_string(),
                response_time_ms: 150,
                request_type: "GET".to_string(),
                status_code: 200,
            },
            LogEntry {
                endpoint_url: "/endpoint2".to_string(),
                response_time_ms: 300,
                request_type: "GET".to_string(),
                status_code: 200,
            },
        ];
        let result = calculate_avg_response_time(&log_entries);
        let mut expected_result = HashMap::new();
        expected_result.insert("/endpoint_test".to_string(), 150);
        expected_result.insert("/endpoint2".to_string(), 225);

        assert_eq!(result, expected_result);
    }
    #[test]
    fn test_analyze_errors() {
        let log_entries = vec![
            LogEntry {
                endpoint_url: "/login".to_string(),
                status_code: 404,
                request_type: "GET".to_string(),
                response_time_ms: 120,
            },
            LogEntry {
                endpoint_url: "/login".to_string(),
                status_code: 500,
                request_type: "GET".to_string(),
                response_time_ms: 120,
            },
            LogEntry {
                endpoint_url: "/login".to_string(),
                status_code: 404,
                request_type: "GET".to_string(),
                response_time_ms: 120,
            },
        ];
        let result = analyze_errors(&log_entries);
        let mut expected_result = HashMap::new();
        expected_result.insert(("/login".to_string(), 404), 2);
        expected_result.insert(("/login".to_string(), 500), 1);

        assert_eq!(result, expected_result);
    }
    #[test]
    fn test_count_request_types() {
        // Create some test entries
        let log_entries = vec![
            LogEntry {
                request_type: "GET".to_string(),
                endpoint_url: "/login".to_string(),
                status_code: 404,
                response_time_ms: 120,
            },
            LogEntry {
                request_type: "POST".to_string(),
                endpoint_url: "/login".to_string(),
                status_code: 404,
                response_time_ms: 120,
            },
            LogEntry {
                request_type: "GET".to_string(),
                endpoint_url: "/login".to_string(),
                status_code: 404,
                response_time_ms: 120,
            },
            LogEntry {
                request_type: "PUT".to_string(),
                endpoint_url: "/login".to_string(),
                status_code: 404,
                response_time_ms: 120,
            },
        ];
        let result = count_request_types(&log_entries);
        let mut expected_result = HashMap::new();
        expected_result.insert("GET".to_string(), 2);
        expected_result.insert("POST".to_string(), 1);
        expected_result.insert("PUT".to_string(), 1);
        assert_eq!(result, expected_result);
    }
}