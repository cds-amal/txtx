use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static REQUEST_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct LspTestClient {
    process: Child,
    request_id: u64,
}

impl LspTestClient {
    /// Start a new LSP server process
    pub fn start(working_dir: &str) -> Result<Self, String> {
        // Try to use cargo test binary, fallback to system txtx
        let txtx_binary = std::env::var("CARGO_BIN_EXE_txtx")
            .unwrap_or_else(|_| "txtx".to_string());
        
        let mut process = Command::new(txtx_binary)
            .arg("lsp")
            .current_dir(working_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start LSP server: {}", e))?;

        Ok(LspTestClient {
            process,
            request_id: REQUEST_ID.fetch_add(1, Ordering::SeqCst),
        })
    }

    /// Send a request and wait for response
    pub fn send_request<T: Serialize>(&mut self, method: &str, params: T) -> Result<Value, String> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params
        });

        self.request_id += 1;
        
        self.send_message(&request)?;
        self.read_response()
    }

    /// Send a notification (no response expected)
    pub fn send_notification<T: Serialize>(&mut self, method: &str, params: T) -> Result<(), String> {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });

        self.send_message(&notification)
    }

    /// Send a message over stdin
    fn send_message(&mut self, message: &Value) -> Result<(), String> {
        let content = message.to_string();
        let header = format!("Content-Length: {}\r\n\r\n", content.len());
        
        let stdin = self.process.stdin.as_mut()
            .ok_or("LSP process stdin not available")?;
        
        stdin.write_all(header.as_bytes())
            .map_err(|e| format!("Failed to write header: {}", e))?;
        stdin.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write content: {}", e))?;
        stdin.flush()
            .map_err(|e| format!("Failed to flush stdin: {}", e))?;
        
        Ok(())
    }

    /// Read a response from stdout
    fn read_response(&mut self) -> Result<Value, String> {
        let stdout = self.process.stdout.as_mut()
            .ok_or("LSP process stdout not available")?;
        
        let mut reader = BufReader::new(stdout);
        let mut header = String::new();
        
        // Read headers
        loop {
            reader.read_line(&mut header)
                .map_err(|e| format!("Failed to read header: {}", e))?;
            
            if header.ends_with("\r\n\r\n") {
                break;
            }
        }
        
        // Parse Content-Length
        let content_length = header.lines()
            .find(|line| line.starts_with("Content-Length:"))
            .and_then(|line| line.split(':').nth(1))
            .and_then(|len| len.trim().parse::<usize>().ok())
            .ok_or("Failed to parse Content-Length")?;
        
        // Read content
        let mut content = vec![0u8; content_length];
        reader.read_exact(&mut content)
            .map_err(|e| format!("Failed to read content: {}", e))?;
        
        let response: Value = serde_json::from_slice(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        Ok(response)
    }

    /// Read any pending notifications (non-blocking)
    pub fn read_notifications(&mut self, timeout_ms: u64) -> Vec<Value> {
        let mut notifications = Vec::new();
        let start = std::time::Instant::now();
        
        while start.elapsed().as_millis() < timeout_ms as u128 {
            // Try to read a message
            match self.try_read_message() {
                Ok(Some(msg)) => {
                    if msg.get("method").is_some() && msg.get("id").is_none() {
                        notifications.push(msg);
                    }
                }
                _ => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
        
        notifications
    }

    /// Try to read a message without blocking
    fn try_read_message(&mut self) -> Result<Option<Value>, String> {
        // This is a simplified version - in production you'd want proper async handling
        // For now, we'll just try to read with a short timeout
        Ok(None)
    }

    /// Initialize the LSP server
    pub fn initialize(&mut self, root_uri: &str) -> Result<Value, String> {
        let params = serde_json::json!({
            "processId": std::process::id(),
            "rootUri": root_uri,
            "capabilities": {
                "textDocument": {
                    "definition": {
                        "dynamicRegistration": true,
                        "linkSupport": true
                    },
                    "hover": {
                        "dynamicRegistration": true,
                        "contentFormat": ["plaintext", "markdown"]
                    },
                    "completion": {
                        "dynamicRegistration": true,
                        "completionItem": {
                            "snippetSupport": true
                        }
                    }
                }
            }
        });

        let response = self.send_request("initialize", params)?;
        
        // Send initialized notification
        self.send_notification("initialized", serde_json::json!({}))?;
        
        Ok(response)
    }

    /// Open a text document
    pub fn open_document(&mut self, uri: &str, language_id: &str, text: &str) -> Result<(), String> {
        self.send_notification("textDocument/didOpen", serde_json::json!({
            "textDocument": {
                "uri": uri,
                "languageId": language_id,
                "version": 1,
                "text": text
            }
        }))
    }

    /// Request go-to-definition
    pub fn goto_definition(&mut self, uri: &str, line: u32, character: u32) -> Result<Value, String> {
        self.send_request("textDocument/definition", serde_json::json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": line,
                "character": character
            }
        }))
    }

    /// Request hover information
    pub fn hover(&mut self, uri: &str, line: u32, character: u32) -> Result<Value, String> {
        self.send_request("textDocument/hover", serde_json::json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": line,
                "character": character
            }
        }))
    }

    /// Request completions
    pub fn completion(&mut self, uri: &str, line: u32, character: u32) -> Result<Value, String> {
        self.send_request("textDocument/completion", serde_json::json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": line,
                "character": character
            }
        }))
    }

    /// Shutdown the LSP server
    pub fn shutdown(mut self) -> Result<(), String> {
        self.send_request("shutdown", serde_json::json!({}))?;
        self.send_notification("exit", serde_json::json!({}))?;
        
        // Wait for process to exit
        let _ = self.process.wait();
        Ok(())
    }
}

impl Drop for LspTestClient {
    fn drop(&mut self) {
        // Ensure process is killed if test fails
        let _ = self.process.kill();
    }
}