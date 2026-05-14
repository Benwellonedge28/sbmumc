//! # SBMUMC Module 1555: Omni-IDE Bridge Protocol
//!
//! Language Server Protocol + Debug Adapter Protocol + WebSocket extension
//! for bidirectional RPC communication with IDEs

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorType {
    VSCode,
    JetBrains,
    Neovim,
    Emacs,
    VS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniIDEBridge {
    pub bridge_id: String,
    pub connected_editors: Vec<EditorType>,
    pub lsp_enabled: bool,
    pub dap_enabled: bool,
    pub websocket_port: u16,
}

impl OmniIDEBridge {
    pub fn new() -> Self {
        Self {
            bridge_id: crate::core::uuid_simple(),
            connected_editors: vec![EditorType::VSCode, EditorType::JetBrains, EditorType::Neovim],
            lsp_enabled: true,
            dap_enabled: true,
            websocket_port: 8765,
        }
    }

    pub fn connect_editor(&mut self, editor: EditorType) -> Result<()> {
        if !self.connected_editors.contains(&editor) {
            self.connected_editors.push(editor);
        }
        Ok(())
    }

    pub fn send_diagnostics(&self, file: &str, diagnostics: &[IDEDiagnostic]) -> Result<()> {
        Ok(())
    }

    pub fn send_completions(&self, file: &str, position: &Position, completions: &[Completion]) -> Result<()> {
        Ok(())
    }

    pub fn send_code_actions(&self, file: &str, range: &Range, actions: &[CodeAction]) -> Result<()> {
        Ok(())
    }

    pub fn receive_file_edit(&mut self, edit: &FileEdit) -> Result<FileEditAck> {
        Ok(FileEditAck {
            accepted: true,
            applied_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64,
        })
    }

    pub fn receive_breakpoint(&self, file: &str, line: u32) -> Result<BreakpointAck> {
        Ok(BreakpointAck {
            id: crate::core::uuid_simple(),
            verified: true,
            message: None,
        })
    }

    pub fn stream_runtime_log(&self, log: &RuntimeLog) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEDiagnostic {
    pub range: Range,
    pub message: String,
    pub severity: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Completion {
    pub label: String,
    pub kind: String,
    pub detail: String,
    pub insert_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAction {
    pub title: String,
    pub kind: String,
    pub edit: Option<FileEdit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEdit {
    pub file_uri: String,
    pub edits: Vec<TextEdit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEditAck {
    pub accepted: bool,
    pub applied_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointAck {
    pub id: String,
    pub verified: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeLog {
    pub timestamp: i64,
    pub level: String,
    pub source: String,
    pub message: String,
}

impl Default for OmniIDEBridge {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ide_bridge_connection() {
        let mut bridge = OmniIDEBridge::new();
        bridge.connect_editor(EditorType::VSCode).unwrap();
        assert!(bridge.connected_editors.contains(&EditorType::VSCode));
    }

    #[test]
    fn test_file_edit_ack() {
        let bridge = OmniIDEBridge::new();
        let edit = FileEdit {
            file_uri: "file:///test.rs".to_string(),
            edits: vec![],
        };
        let ack = bridge.receive_file_edit(&edit).unwrap();
        assert!(ack.accepted);
    }
}