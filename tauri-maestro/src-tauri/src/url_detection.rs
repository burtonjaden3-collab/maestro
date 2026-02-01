use once_cell::sync::Lazy;
use regex::Regex;

/// Detected server URL with port
#[derive(Debug, Clone)]
pub struct DetectedServer {
    pub url: String,
    pub port: u16,
}

/// Patterns for detecting localhost URLs in terminal output
static URL_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // http://localhost:3000 or https://localhost:3000
        Regex::new(r"https?://localhost:(\d+)").unwrap(),
        // http://127.0.0.1:3000
        Regex::new(r"https?://127\.0\.0\.1:(\d+)").unwrap(),
        // http://0.0.0.0:3000
        Regex::new(r"https?://0\.0\.0\.0:(\d+)").unwrap(),
        // http://[::1]:3000 (IPv6 localhost)
        Regex::new(r"https?://\[::1\]:(\d+)").unwrap(),
        // "Local:" or "Local  :" followed by URL (Vite, Next.js style)
        Regex::new(r"Local:?\s+https?://[^\s]+:(\d+)").unwrap(),
        // "listening on port 3000" or "on port 3000"
        Regex::new(r"(?:listening|running|started|ready)\s+(?:on\s+)?port\s+(\d+)").unwrap(),
        // "Server running at http://localhost:3000"
        Regex::new(r"(?:Server|App|Application)\s+(?:running|listening|started)\s+(?:at|on)\s+https?://[^\s:]+:(\d+)").unwrap(),
    ]
});

/// Detect localhost URLs from terminal output
pub fn detect_server_url(output: &str) -> Option<DetectedServer> {
    for pattern in URL_PATTERNS.iter() {
        if let Some(captures) = pattern.captures(output) {
            // Get the port from capture group 1
            if let Some(port_match) = captures.get(1) {
                if let Ok(port) = port_match.as_str().parse::<u16>() {
                    // Build full URL
                    let url = format!("http://localhost:{}", port);
                    return Some(DetectedServer { url, port });
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_localhost() {
        let result = detect_server_url("Server started at http://localhost:3000");
        assert!(result.is_some());
        let server = result.unwrap();
        assert_eq!(server.port, 3000);
        assert_eq!(server.url, "http://localhost:3000");
    }

    #[test]
    fn test_detect_vite_style() {
        let result = detect_server_url("  Local:   http://localhost:5173/");
        assert!(result.is_some());
        assert_eq!(result.unwrap().port, 5173);
    }

    #[test]
    fn test_detect_127_0_0_1() {
        let result = detect_server_url("App running at http://127.0.0.1:8080");
        assert!(result.is_some());
        assert_eq!(result.unwrap().port, 8080);
    }

    #[test]
    fn test_detect_listening_on_port() {
        let result = detect_server_url("Express server listening on port 4000");
        assert!(result.is_some());
        assert_eq!(result.unwrap().port, 4000);
    }

    #[test]
    fn test_no_match() {
        let result = detect_server_url("Hello world");
        assert!(result.is_none());
    }
}
