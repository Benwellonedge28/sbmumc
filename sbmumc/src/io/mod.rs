//! IO Module - Input/Output Handling for SBMUMC
//!
//! This module handles all input and output operations including:
//! - Text input/output
//! - Voice input/output
//! - File handling
//! - Multi-modal processing

use crate::core::{SbmumcError, Result, Input, Output, FileFormat, FileInput, FileOutput, EntityId};
use std::path::PathBuf;
use parking_lot::RwLock;
use std::collections::VecDeque;
use tracing::{debug, info};

/// Input Handler - Manages all input processing
pub struct InputHandler {
    /// Text input buffer
    text_buffer: RwLock<VecDeque<BufferedInput>>,

    /// Voice input processor
    voice_processor: VoiceProcessor,

    /// File input handler
    file_handler: FileInputHandler,

    /// Maximum buffer size
    max_buffer_size: usize,
}

/// Voice processor for audio input
pub struct VoiceProcessor {
    /// Supported audio formats
    supported_formats: Vec<String>,

    /// Sample rate
    sample_rate: u32,

    /// Enable voice recognition
    enable_recognition: bool,
}

/// File input handler
pub struct FileInputHandler {
    /// Supported file formats
    supported_formats: Vec<FileFormat>,

    /// Maximum file size
    max_file_size: usize,

    /// File watching enabled
    enable_watching: bool,
}

/// Buffered input
#[derive(Debug, Clone)]
pub struct BufferedInput {
    pub id: EntityId,
    pub input: Input,
    pub timestamp: crate::core::Timestamp,
    pub processed: bool,
}

/// Output Handler - Manages all output generation
pub struct OutputHandler {
    /// Text output buffer
    text_buffer: RwLock<VecDeque<BufferedOutput>>,

    /// Voice output processor
    voice_processor: VoiceOutputProcessor,

    /// File output handler
    file_handler: FileOutputHandler,

    /// Output history
    history: RwLock<Vec<OutputRecord>>,

    /// Maximum history size
    max_history_size: usize,
}

/// Voice output processor
pub struct VoiceOutputProcessor {
    /// Supported audio formats
    supported_formats: Vec<String>,

    /// Default voice
    default_voice: String,

    /// Enable voice synthesis
    enable_synthesis: bool,
}

/// File output handler
pub struct FileOutputHandler {
    /// Output directory
    output_dir: PathBuf,

    /// Enable automatic naming
    auto_naming: bool,
}

/// Buffered output
#[derive(Debug, Clone)]
pub struct BufferedOutput {
    pub id: EntityId,
    pub output: Output,
    pub timestamp: crate::core::Timestamp,
    pub delivered: bool,
}

/// Output record for history
#[derive(Debug, Clone)]
pub struct OutputRecord {
    pub id: EntityId,
    pub output: Output,
    pub timestamp: crate::core::Timestamp,
    pub format: String,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Result<Self> {
        info!("Initializing Input Handler");

        Ok(Self {
            text_buffer: RwLock::new(VecDeque::new()),
            voice_processor: VoiceProcessor::new()?,
            file_handler: FileInputHandler::new()?,
            max_buffer_size: 1000,
        })
    }

    /// Handle text input
    pub fn handle_text(&self, text: String) -> Result<EntityId> {
        debug!("Handling text input: {} chars", text.len());

        let id = EntityId::new();
        let input = Input::Text(text);

        let buffered = BufferedInput {
            id,
            input,
            timestamp: crate::core::Timestamp::now(),
            processed: false,
        };

        // Add to buffer
        let mut buffer = self.text_buffer.write();
        buffer.push_back(buffered);

        // Trim if too large
        while buffer.len() > self.max_buffer_size {
            buffer.pop_front();
        }

        Ok(id)
    }

    /// Handle voice input
    pub fn handle_voice(&self, audio_data: Vec<u8>, format: &str) -> Result<EntityId> {
        debug!("Handling voice input: {} bytes, format: {}", audio_data.len(), format);

        if !self.voice_processor.supports_format(format) {
            return Err(SbmumcError::Input(format!("Unsupported audio format: {}", format)));
        }

        let id = EntityId::new();
        let input = Input::Voice(crate::core::VoiceInput {
            audio_data,
            format: format.to_string(),
            language: None,
        });

        let buffered = BufferedInput {
            id,
            input,
            timestamp: crate::core::Timestamp::now(),
            processed: false,
        };

        let mut buffer = self.text_buffer.write();
        buffer.push_back(buffered);

        Ok(id)
    }

    /// Handle file input
    pub fn handle_file(&self, path: &str, format: FileFormat) -> Result<EntityId> {
        debug!("Handling file input: {}, format: {:?}", path, format);

        if !self.file_handler.supports_format(&format) {
            return Err(SbmumcError::Input(format!("Unsupported file format: {:?}", format)));
        }

        // Read file
        let content = self.file_handler.read_file(path)?;

        let id = EntityId::new();
        let input = Input::File(FileInput {
            path: path.to_string(),
            format,
            content: Some(content),
        });

        let buffered = BufferedInput {
            id,
            input,
            timestamp: crate::core::Timestamp::now(),
            processed: false,
        };

        let mut buffer = self.text_buffer.write();
        buffer.push_back(buffered);

        Ok(id)
    }

    /// Get next buffered input
    pub fn get_next_input(&self) -> Option<BufferedInput> {
        let mut buffer = self.text_buffer.write();
        buffer.pop_front()
    }

    /// Get buffered input by ID
    pub fn get_input(&self, id: EntityId) -> Option<BufferedInput> {
        let buffer = self.text_buffer.read();
        buffer.iter().find(|i| i.id == id).cloned()
    }

    /// Mark input as processed
    pub fn mark_processed(&self, id: EntityId) -> Result<()> {
        let mut buffer = self.text_buffer.write();
        if let Some(input) = buffer.iter_mut().find(|i| i.id == id) {
            input.processed = true;
            Ok(())
        } else {
            Err(SbmumcError::Input("Input not found".to_string()))
        }
    }

    /// Clear processed inputs
    pub fn clear_processed(&self) {
        let mut buffer = self.text_buffer.write();
        buffer.retain(|i| !i.processed);
    }

    /// Get buffer size
    pub fn buffer_size(&self) -> usize {
        self.text_buffer.read().len()
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new().expect("Failed to create InputHandler")
    }
}

impl VoiceProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            supported_formats: vec![
                "wav".to_string(),
                "mp3".to_string(),
                "ogg".to_string(),
                "flac".to_string(),
            ],
            sample_rate: 16000,
            enable_recognition: true,
        })
    }

    pub fn supports_format(&self, format: &str) -> bool {
        self.supported_formats.iter().any(|f| f == format)
    }

    pub fn process_audio(&self, audio_data: &[u8]) -> Result<String> {
        if !self.enable_recognition {
            return Err(SbmumcError::Input("Voice recognition disabled".to_string()));
        }

        // In a full implementation, this would use a speech recognition model
        Ok("[Processed audio content]".to_string())
    }
}

impl FileInputHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            supported_formats: vec![
                FileFormat::Text,
                FileFormat::SourceCode,
                FileFormat::Grammar,
                FileFormat::Document,
                FileFormat::Data,
            ],
            max_file_size: 100 * 1024 * 1024, // 100MB
            enable_watching: false,
        })
    }

    pub fn supports_format(&self, format: &FileFormat) -> bool {
        self.supported_formats.iter().any(|f| f == format)
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        let path = PathBuf::from(path);

        if !path.exists() {
            return Err(SbmumcError::FileNotFound(path.display().to_string()));
        }

        let metadata = std::fs::metadata(&path)?;
        if metadata.len() > self.max_file_size as u64 {
            return Err(SbmumcError::Input("File too large".to_string()));
        }

        std::fs::read(&path).map_err(|e| SbmumcError::Input(format!("Failed to read file: {}", e)))
    }

    pub fn detect_format(&self, path: &str) -> Option<FileFormat> {
        let extension = PathBuf::from(path)
            .extension()?
            .to_str()?
            .to_lowercase();

        match extension.as_str() {
            "txt" | "text" => Some(FileFormat::Text),
            "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" | "go" | "rust" => Some(FileFormat::SourceCode),
            "g4" | "grammar" | "ebnf" => Some(FileFormat::Grammar),
            "pdf" | "doc" | "docx" | "odt" => Some(FileFormat::Document),
            "json" | "xml" | "yaml" | "yml" | "csv" => Some(FileFormat::Data),
            _ => Some(FileFormat::Unknown),
        }
    }
}

impl OutputHandler {
    /// Create a new output handler
    pub fn new() -> Result<Self> {
        info!("Initializing Output Handler");

        Ok(Self {
            text_buffer: RwLock::new(VecDeque::new()),
            voice_processor: VoiceOutputProcessor::new()?,
            file_handler: FileOutputHandler::new()?,
            history: RwLock::new(Vec::new()),
            max_history_size: 1000,
        })
    }

    /// Handle text output
    pub fn handle_text(&self, text: String) -> Result<EntityId> {
        debug!("Handling text output: {} chars", text.len());

        let id = EntityId::new();
        let output = Output::Text(text);

        let buffered = BufferedOutput {
            id,
            output,
            timestamp: crate::core::Timestamp::now(),
            delivered: false,
        };

        // Add to buffer
        let mut buffer = self.text_buffer.write();
        buffer.push_back(buffered);

        // Trim if too large
        while buffer.len() > self.max_history_size {
            buffer.pop_front();
        }

        Ok(id)
    }

    /// Handle structured output
    pub fn handle_structured(&self, data: serde_json::Value) -> Result<EntityId> {
        debug!("Handling structured output");

        let id = EntityId::new();
        let output = Output::Structured(data);

        let buffered = BufferedOutput {
            id,
            output,
            timestamp: crate::core::Timestamp::now(),
            delivered: false,
        };

        let mut buffer = self.text_buffer.write();
        buffer.push_back(buffered);

        Ok(id)
    }

    /// Handle voice output
    pub fn handle_voice(&self, text: &str, format: &str) -> Result<EntityId> {
        debug!("Handling voice output: {} chars, format: {}", text.len(), format);

        if !self.voice_processor.supports_format(format) {
            return Err(SbmumcError::Output(format!("Unsupported audio format: {}", format)));
        }

        // Synthesize audio (simplified)
        let audio_data = self.voice_processor.synthesize(text)?;

        let id = EntityId::new();
        let output = Output::File(FileOutput {
            path: format!("voice_{}.{}", id, format),
            format: FileFormat::Audio,
            content: audio_data,
        });

        let buffered = BufferedOutput {
            id,
            output,
            timestamp: crate::core::Timestamp::now(),
            delivered: false,
        };

        let mut buffer = self.text_buffer.write();
        buffer.push_back(buffered);

        Ok(id)
    }

    /// Handle file output
    pub fn handle_file(&self, path: &str, content: Vec<u8>, format: FileFormat) -> Result<EntityId> {
        debug!("Handling file output: {}, format: {:?}", path, format);

        let id = EntityId::new();
        let output = Output::File(FileOutput {
            path: path.to_string(),
            format,
            content,
        });

        // Write file
        self.file_handler.write_file(path, &output.content)?;

        let buffered = BufferedOutput {
            id,
            output,
            timestamp: crate::core::Timestamp::now(),
            delivered: true, // Already written
        };

        let mut buffer = self.text_buffer.write();
        buffer.push_back(buffered);

        Ok(id)
    }

    /// Get next buffered output
    pub fn get_next_output(&self) -> Option<BufferedOutput> {
        let mut buffer = self.text_buffer.write();
        buffer.pop_front()
    }

    /// Get output by ID
    pub fn get_output(&self, id: EntityId) -> Option<BufferedOutput> {
        let buffer = self.text_buffer.read();
        buffer.iter().find(|o| o.id == id).cloned()
    }

    /// Mark output as delivered
    pub fn mark_delivered(&self, id: EntityId) -> Result<()> {
        let mut buffer = self.text_buffer.write();
        if let Some(output) = buffer.iter_mut().find(|o| o.id == id) {
            output.delivered = true;

            // Add to history
            let history = &mut self.history.write();
            history.push(OutputRecord {
                id,
                output: output.output.clone(),
                timestamp: output.timestamp,
                format: "text".to_string(),
            });

            Ok(())
        } else {
            Err(SbmumcError::Output("Output not found".to_string()))
        }
    }

    /// Get output history
    pub fn get_history(&self, limit: usize) -> Vec<OutputRecord> {
        let history = self.history.read();
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Get buffer size
    pub fn buffer_size(&self) -> usize {
        self.text_buffer.read().len()
    }
}

impl Default for OutputHandler {
    fn default() -> Self {
        Self::new().expect("Failed to create OutputHandler")
    }
}

impl VoiceOutputProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            supported_formats: vec![
                "wav".to_string(),
                "mp3".to_string(),
                "ogg".to_string(),
            ],
            default_voice: "default".to_string(),
            enable_synthesis: true,
        })
    }

    pub fn supports_format(&self, format: &str) -> bool {
        self.supported_formats.iter().any(|f| f == format)
    }

    pub fn synthesize(&self, text: &str) -> Result<Vec<u8>> {
        if !self.enable_synthesis {
            return Err(SbmumcError::Output("Voice synthesis disabled".to_string()));
        }

        // In a full implementation, this would use a text-to-speech engine
        // For now, return placeholder data
        Ok(text.as_bytes().to_vec())
    }
}

impl FileOutputHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            output_dir: PathBuf::from("output"),
            auto_naming: true,
        })
    }

    pub fn set_output_dir(&mut self, path: PathBuf) {
        self.output_dir = path;
    }

    pub fn write_file(&self, path: &str, content: &[u8]) -> Result<()> {
        let path = PathBuf::from(path);

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&path, content)
            .map_err(|e| SbmumcError::Output(format!("Failed to write file: {}", e)))
    }

    pub fn generate_filename(&self, prefix: &str, extension: &str) -> String {
        let timestamp = chrono::Utc::now().timestamp();
        format!("{}_{}.{}", prefix, timestamp, extension)
    }
}
