// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

const DEFAULT_PENPOT_URL: &str = "http://localhost:9001";

/// Resolve the Penpot URL from the environment, falling back to the default.
fn resolve_penpot_url() -> String {
    std::env::var("PENPOT_URL").unwrap_or_else(|_| DEFAULT_PENPOT_URL.to_string())
}

/// Validate that a URL string can be parsed into a valid URL.
#[cfg(test)]
fn validate_url(url: &str) -> Result<url::Url, url::ParseError> {
    url.parse::<url::Url>()
}

fn main() {
    let penpot_url = resolve_penpot_url();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app
                .get_webview_window("main")
                .expect("failed to get main window");

            let _ = window.navigate(penpot_url.parse().expect("invalid PENPOT_URL"));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_url_when_env_unset() {
        // Make sure PENPOT_URL is not set for this test
        std::env::remove_var("PENPOT_URL");
        assert_eq!(resolve_penpot_url(), DEFAULT_PENPOT_URL);
    }

    #[test]
    fn custom_url_from_env() {
        let custom = "http://my-penpot:8080";
        std::env::set_var("PENPOT_URL", custom);
        assert_eq!(resolve_penpot_url(), custom);
        std::env::remove_var("PENPOT_URL");
    }

    #[test]
    fn default_url_is_valid() {
        assert!(validate_url(DEFAULT_PENPOT_URL).is_ok());
    }

    #[test]
    fn valid_http_url() {
        assert!(validate_url("http://localhost:9001").is_ok());
    }

    #[test]
    fn valid_https_url() {
        assert!(validate_url("https://penpot.example.com").is_ok());
    }

    #[test]
    fn invalid_url_rejected() {
        assert!(validate_url("not a url").is_err());
    }

    #[test]
    fn empty_url_rejected() {
        assert!(validate_url("").is_err());
    }

    #[test]
    fn url_with_path_is_valid() {
        assert!(validate_url("http://localhost:9001/some/path").is_ok());
    }
}
