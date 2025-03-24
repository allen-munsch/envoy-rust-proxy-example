use std::time::SystemTime;

// src/lib.rs
use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
struct FilterConfig {
    header_name: String,
    header_value: String,
    upstream_timeout_ms: u64,
}

// Implement a custom filter root context
#[derive(Default)]
struct FilterRoot {
    config: Option<FilterConfig>,
}

impl Context for FilterRoot {}

impl RootContext for FilterRoot {
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            match serde_json::from_slice::<FilterConfig>(&config_bytes) {
                Ok(config) => {
                    info!("Loaded configuration: {:?}", config);
                    self.config = Some(config);
                    true
                }
                Err(e) => {
                    info!("Failed to parse configuration: {}", e);
                    false
                }
            }
        } else {
            info!("No configuration provided, using defaults");
            self.config = Some(FilterConfig {
                header_name: "x-wasm-filter".to_string(),
                header_value: "envoy-rust".to_string(),
                upstream_timeout_ms: 5000,
            });
            true
        }
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        let config = self.config.clone()?;
        Some(Box::new(HttpFilter {
            context_id,
            config,
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

// HTTP filter that handles HTTP requests and responses
struct HttpFilter {
    context_id: u32,
    config: FilterConfig,
}

#[derive(Serialize)]
struct Metadata {
    request_path: String,
    filter_version: String,
    timestamp: SystemTime,
}

impl Context for HttpFilter {}

impl HttpContext for HttpFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        // Get and log request headers
        let headers = self.get_http_request_headers();
        info!("Request headers: {:?}", headers);

        // Set custom header
        self.set_http_request_header(&self.config.header_name, Some(&self.config.header_value));

        // Extract path for later use
        let path = self
            .get_http_request_header(":path")
            .unwrap_or_default();

        // Store path in context
        self.set_property(vec!["request_path"], Some(path.as_bytes()));

        // Set upstream timeout
        self.set_http_request_header("x-envoy-upstream-rq-timeout-ms", 
                                    Some(&self.config.upstream_timeout_ms.to_string()));

        // Continue processing
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        // Get response headers
        let headers = self.get_http_response_headers();
        info!("Response headers: {:?}", headers);

        // Add a custom response header
        self.set_http_response_header("x-wasm-response", Some("processed"));

        // Get stored request path
        let path_bytes = self.get_property(vec!["request_path"]).unwrap_or_default();
        let request_path = String::from_utf8(path_bytes).unwrap_or_default();

        // Create response metadata
        let metadata = Metadata {
            request_path,
            filter_version: "0.1.0".to_string(),
            timestamp: self.get_current_time(),
        };

        // Serialize and add as response header
        if let Ok(metadata_json) = serde_json::to_string(&metadata) {
            self.set_http_response_header("x-response-metadata", Some(&metadata_json));
        }

        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if !end_of_stream {
            // Wait for the entire body
            return Action::Pause;
        }

        if body_size > 0 {
            // Get and potentially modify the response body
            if let Some(body_bytes) = self.get_http_response_body(0, body_size) {
                info!("Response body size: {}", body_bytes.len());

                // For demonstration, we'll just log the size but not modify the body
                // You could modify the body here if needed
            }
        }

        Action::Continue
    }

    // Handle log phase
    fn on_log(&mut self) {
        info!("HTTP request/response completed for context: {}", self.context_id);
    }
}

// Register the root context factory
#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(FilterRoot::default())
    });
}