use crate::stdlib::embed_that::error::{EmbedError, EmbedResult};
use crate::stdlib::io::{IoResult, IoError};
use std::collections::HashMap;
use std::io::{Read, Cursor};
use std::time::SystemTime;

/// Core type aliases for CURSED language compatibility
pub type tea = String;  // String type in CURSED
pub type normie = i32;  // Integer type in CURSED  
pub type lit = bool;    // Boolean type in CURSED

/// Represents a single embedded file with metadata and content
#[derive(Debug, Clone)]
pub struct ThatFile {
    name: tea,
    content: Vec<u8>,
    size: i64,
    mod_time: SystemTime,
    mime_type: Option<tea>,
}

impl ThatFile {
    /// Create a new ThatFile instance
    pub fn new(name: tea, content: Vec<u8>) -> Self {
        let size = content.len() as i64;
        let mod_time = SystemTime::now();
        
        Self {
            name,
            content,
            size,
            mod_time,
            mime_type: None,
        }
    }
    
    /// Create a ThatFile with explicit metadata
    pub fn with_metadata(name: tea, content: Vec<u8>, mod_time: SystemTime) -> Self {
        let size = content.len() as i64;
        
        Self {
            name,
            content,
            size,
            mod_time,
            mime_type: None,
        }
    }
    
    /// Get the file name
    pub fn name(&self) -> tea {
        self.name.clone()
    }
    
    /// Get the file size in bytes
    pub fn size(&self) -> i64 {
        self.size
    }
    
    /// Get the modification time
    pub fn mod_time(&self) -> SystemTime {
        self.mod_time
    }
    
    /// Get the file content as bytes
    pub fn content(&self) -> Vec<u8> {
        self.content.clone()
    }
    
    /// Get the file content as a string
    pub fn content_string(&self) -> EmbedResult<tea> {
        String::from_utf8(self.content.clone())
            .map_err(|e| EmbedError::Utf8Error { reason: e.to_string() })
    }
    
    /// Get a hash of the file content
    pub fn hash(&self) -> tea {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&self.content);
        format!("{:x}", hasher.finalize())
    }
    
    /// Get the MIME type of the file
    pub fn mime_type(&self) -> tea {
        if let Some(ref mime_type) = self.mime_type {
            return mime_type.clone();
        }
        
        // Simple MIME type detection based on file extension
        let extension = self.extension().to_lowercase();
        match extension.as_str() {
            "html" | "htm" => "text/html".to_string(),
            "css" => "text/css".to_string(),
            "js" => "application/javascript".to_string(),
            "json" => "application/json".to_string(),
            "xml" => "application/xml".to_string(),
            "txt" => "text/plain".to_string(),
            "md" => "text/markdown".to_string(),
            "png" => "image/png".to_string(),
            "jpg" | "jpeg" => "image/jpeg".to_string(),
            "gif" => "image/gif".to_string(),
            "svg" => "image/svg+xml".to_string(),
            "pdf" => "application/pdf".to_string(),
            "zip" => "application/zip".to_string(),
            "tar" => "application/x-tar".to_string(),
            "gz" => "application/gzip".to_string(),
            "mp3" => "audio/mpeg".to_string(),
            "wav" => "audio/wav".to_string(),
            "mp4" => "video/mp4".to_string(),
            "avi" => "video/x-msvideo".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
    
    /// Get the file extension
    pub fn extension(&self) -> tea {
        if let Some(pos) = self.name.rfind('.') {
            self.name[pos + 1..].to_string()
        } else {
            String::new()
        }
    }
    
    /// Check if the file is likely a text file
    pub fn is_text(&self) -> lit {
        let mime_type = self.mime_type();
        mime_type.starts_with("text/") || 
        mime_type == "application/json" ||
        mime_type == "application/xml" ||
        mime_type == "application/javascript"
    }
    
    /// Check if the file is an image
    pub fn is_image(&self) -> lit {
        self.mime_type().starts_with("image/")
    }
    
    /// Check if the file is audio
    pub fn is_audio(&self) -> lit {
        self.mime_type().starts_with("audio/")
    }
    
    /// Check if the file is video
    pub fn is_video(&self) -> lit {
        self.mime_type().starts_with("video/")
    }
    
    /// Get a reader for the file content
    pub fn reader(&self) -> Box<dyn Read> {
        Box::new(Cursor::new(self.content.clone()))
    }
    
    /// Set the MIME type explicitly
    pub fn set_mime_type(&mut self, mime_type: tea) {
        self.mime_type = Some(mime_type);
    }
    
    /// Decode the file content based on its format
    pub fn decode<T>(&self) -> EmbedResult<T> 
    where 
        T: serde::de::DeserializeOwned 
    {
        let mime_type = self.mime_type();
        let content_str = self.content_string()?;
        
        match mime_type.as_str() {
            "application/json" => {
                serde_json::from_str(&content_str)
                    .map_err(|e| EmbedError::JsonParsingError { reason: e.to_string() })
            },
            "application/x-yaml" | "text/yaml" => {
                serde_yaml::from_str(&content_str)
                    .map_err(|e| EmbedError::YamlParsingError { reason: e.to_string() })
            },
            "application/toml" | "text/toml" => {
                toml::from_str(&content_str)
                    .map_err(|e| EmbedError::TomlParsingError { reason: e.to_string() })
            },
            _ => {
                // Try to detect format by extension
                let extension = self.extension().to_lowercase();
                match extension.as_str() {
                    "json" => {
                        serde_json::from_str(&content_str)
                            .map_err(|e| EmbedError::JsonParsingError { reason: e.to_string() })
                    },
                    "yaml" | "yml" => {
                        serde_yaml::from_str(&content_str)
                            .map_err(|e| EmbedError::YamlParsingError { reason: e.to_string() })
                    },
                    "toml" => {
                        toml::from_str(&content_str)
                            .map_err(|e| EmbedError::TomlParsingError { reason: e.to_string() })
                    },
                    _ => Err(EmbedError::InvalidFormat { 
                        file: self.name.clone(), 
                        reason: format!("Unsupported format for decoding: {}", mime_type) 
                    })
                }
            }
        }
    }
}

/// Represents a collection of embedded files
#[derive(Debug, Clone)]
pub struct ThatFiles {
    files: HashMap<tea, ThatFile>,
}

impl ThatFiles {
    /// Create a new empty ThatFiles collection
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
    
    /// Create a ThatFiles collection from a vector of files
    pub fn from_files(files: Vec<ThatFile>) -> Self {
        let mut file_map = HashMap::new();
        for file in files {
            file_map.insert(file.name(), file);
        }
        
        Self {
            files: file_map,
        }
    }
    
    /// Add a file to the collection
    pub fn add_file(&mut self, file: ThatFile) {
        self.files.insert(file.name(), file);
    }
    
    /// Get a file by name
    pub fn get(&self, name: &tea) -> (ThatFile, lit) {
        match self.files.get(name) {
            Some(file) => (file.clone(), true),
            None => (ThatFile::new(String::new(), vec![]), false),
        }
    }
    
    /// Get a file by name, panic if not found (similar to Go's "must" pattern)
    pub fn get_must(&self, name: &tea) -> EmbedResult<ThatFile> {
        self.files.get(name)
            .cloned()
            .ok_or_else(|| EmbedError::FileNotFound { file: name.clone() })
    }
    
    /// Get all file names
    pub fn names(&self) -> Vec<tea> {
        self.files.keys().cloned().collect()
    }
    
    /// Get all files as a list
    pub fn list(&self) -> Vec<ThatFile> {
        self.files.values().cloned().collect()
    }
    
    /// Get the number of files
    pub fn count(&self) -> normie {
        self.files.len() as normie
    }
    
    /// Get the total size of all files
    pub fn total_size(&self) -> i64 {
        self.files.values().map(|f| f.size()).sum()
    }
    
    /// Filter files by pattern (simple glob matching)
    pub fn filter(&self, pattern: &tea) -> ThatFiles {
        let regex_pattern = glob_to_regex(pattern);
        let mut filtered = ThatFiles::new();
        
        for (name, file) in &self.files {
            if regex_pattern.is_match(name) {
                filtered.add_file(file.clone());
            }
        }
        
        filtered
    }
    
    /// Filter files by extension
    pub fn filter_by_ext(&self, ext: &tea) -> ThatFiles {
        let mut filtered = ThatFiles::new();
        let ext_lower = ext.to_lowercase();
        
        for (_, file) in &self.files {
            if file.extension().to_lowercase() == ext_lower {
                filtered.add_file(file.clone());
            }
        }
        
        filtered
    }
    
    /// Filter files by MIME type
    pub fn filter_by_mime(&self, mime_type: &tea) -> ThatFiles {
        let mut filtered = ThatFiles::new();
        
        for (_, file) in &self.files {
            if file.mime_type() == *mime_type {
                filtered.add_file(file.clone());
            }
        }
        
        filtered
    }
    
    /// Create a FileSystemVibe implementation from this collection
    pub fn make_fs(&self) -> Box<dyn FileSystemVibe> {
        Box::new(EmbeddedFileSystem::new(self.clone()))
    }
}

impl Default for ThatFiles {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a string loaded from an embedded file
#[derive(Debug, Clone)]
pub struct ThatString {
    content: tea,
}

impl ThatString {
    /// Create a new ThatString
    pub fn new(content: tea) -> Self {
        Self { content }
    }
    
    /// Get the string content
    pub fn string(&self) -> tea {
        self.content.clone()
    }
    
    /// Get the content as bytes
    pub fn bytes(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }
    
    /// Get a reader for the content
    pub fn reader(&self) -> Box<dyn Read> {
        Box::new(Cursor::new(self.content.as_bytes().to_vec()))
    }
    
    /// Split the content into lines
    pub fn lines(&self) -> Vec<tea> {
        self.content.lines().map(|s| s.to_string()).collect()
    }
    
    /// Split the content by a separator
    pub fn split(&self, sep: &tea) -> Vec<tea> {
        self.content.split(sep).map(|s| s.to_string()).collect()
    }
}

/// Represents a byte slice loaded from an embedded file
#[derive(Debug, Clone)]
pub struct ThatBytes {
    content: Vec<u8>,
}

impl ThatBytes {
    /// Create a new ThatBytes
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
    
    /// Get the content as a string
    pub fn string(&self) -> EmbedResult<tea> {
        String::from_utf8(self.content.clone())
            .map_err(|e| EmbedError::Utf8Error { reason: e.to_string() })
    }
    
    /// Get a reader for the content
    pub fn reader(&self) -> Box<dyn Read> {
        Box::new(Cursor::new(self.content.clone()))
    }
    
    /// Write the content to a writer
    pub fn write_to<W: std::io::Write>(&self, mut writer: W) -> IoResult<i64> {
        match writer.write_all(&self.content) {
            Ok(()) => Ok(self.content.len() as i64),
            Err(e) => Err(IoError::General { message: e.to_string() }),
        }
    }
    
    /// Auto-detect format and decode the content
    pub fn decode<T>(&self) -> EmbedResult<T>
    where
        T: serde::de::DeserializeOwned
    {
        let content_str = self.string()?;
        
        // Try JSON first
        if let Ok(result) = serde_json::from_str::<T>(&content_str) {
            return Ok(result);
        }
        
        // Try YAML
        if let Ok(result) = serde_yaml::from_str::<T>(&content_str) {
            return Ok(result);
        }
        
        // Try TOML
        if let Ok(result) = toml::from_str::<T>(&content_str) {
            return Ok(result);
        }
        
        Err(EmbedError::ConfigParsingError { 
            reason: "Unable to parse as JSON, YAML, or TOML".to_string() 
        })
    }
}

/// FileSystemVibe trait for embedded file systems
pub trait FileSystemVibe: Send + Sync {
    /// Open a file
    fn open(&self, name: &tea) -> EmbedResult<Box<dyn Read>>;
    
    /// Read a file completely
    fn read_file(&self, name: &tea) -> EmbedResult<Vec<u8>>;
    
    /// Read directory entries
    fn read_dir(&self, name: &tea) -> EmbedResult<Vec<DirEntry>>;
    
    /// Get file statistics
    fn stat(&self, name: &tea) -> EmbedResult<FileInfo>;
    
    /// Create a subdirectory view
    fn sub(&self, dir: &tea) -> EmbedResult<Box<dyn FileSystemVibe>>;
    
    /// Find files matching a pattern
    fn glob(&self, pattern: &tea) -> EmbedResult<Vec<tea>>;
    
    /// Walk the file tree
    fn walk(&self, root: &tea, callback: Box<dyn Fn(&tea, &FileInfo) -> EmbedResult<()>>) -> EmbedResult<()>;
}

/// Directory entry information
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: tea,
    pub is_dir: lit,
    pub size: i64,
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: tea,
    pub size: i64,
    pub mod_time: SystemTime,
    pub is_dir: lit,
}

/// Embedded file system implementation
#[derive(Debug, Clone)]
pub struct EmbeddedFileSystem {
    files: ThatFiles,
}

impl EmbeddedFileSystem {
    pub fn new(files: ThatFiles) -> Self {
        Self { files }
    }
}

impl FileSystemVibe for EmbeddedFileSystem {
    fn open(&self, name: &tea) -> EmbedResult<Box<dyn Read>> {
        let (file, found) = self.files.get(name);
        if !found {
            return Err(EmbedError::FileNotFound { file: name.clone() });
        }
        Ok(file.reader())
    }
    
    fn read_file(&self, name: &tea) -> EmbedResult<Vec<u8>> {
        let (file, found) = self.files.get(name);
        if !found {
            return Err(EmbedError::FileNotFound { file: name.clone() });
        }
        Ok(file.content())
    }
    
    fn read_dir(&self, name: &tea) -> EmbedResult<Vec<DirEntry>> {
        let mut entries = Vec::new();
        let prefix = if name.is_empty() { String::new() } else { format!("{}/", name) };
        
        for file_name in self.files.names() {
            if file_name.starts_with(&prefix) {
                let relative_name = &file_name[prefix.len()..];
                if !relative_name.contains('/') {
                    let (file, _) = self.files.get(&file_name);
                    entries.push(DirEntry {
                        name: relative_name.to_string(),
                        is_dir: false,
                        size: file.size(),
                    });
                }
            }
        }
        
        Ok(entries)
    }
    
    fn stat(&self, name: &tea) -> EmbedResult<FileInfo> {
        let (file, found) = self.files.get(name);
        if !found {
            return Err(EmbedError::FileNotFound { file: name.clone() });
        }
        
        Ok(FileInfo {
            name: file.name(),
            size: file.size(),
            mod_time: file.mod_time(),
            is_dir: false,
        })
    }
    
    fn sub(&self, dir: &tea) -> EmbedResult<Box<dyn FileSystemVibe>> {
        let filtered = self.files.filter(&format!("{}/*", dir));
        Ok(Box::new(EmbeddedFileSystem::new(filtered)))
    }
    
    fn glob(&self, pattern: &tea) -> EmbedResult<Vec<tea>> {
        let filtered = self.files.filter(pattern);
        Ok(filtered.names())
    }
    
    fn walk(&self, root: &tea, callback: Box<dyn Fn(&tea, &FileInfo) -> EmbedResult<()>>) -> EmbedResult<()> {
        for name in self.files.names() {
            if name.starts_with(root) {
                let (file, _) = self.files.get(&name);
                let info = FileInfo {
                    name: file.name(),
                    size: file.size(),
                    mod_time: file.mod_time(),
                    is_dir: false,
                };
                callback(&name, &info)?;
            }
        }
        Ok(())
    }
}

/// Convert a glob pattern to a regex
fn glob_to_regex(pattern: &str) -> regex::Regex {
    let mut regex_pattern = String::new();
    let mut chars = pattern.chars().peekable();
    
    regex_pattern.push('^');
    
    while let Some(ch) = chars.next() {
        match ch {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // consume the second *
                    if chars.peek() == Some(&'/') {
                        chars.next(); // consume the /
                        regex_pattern.push_str("(?:.*/)?");
                    } else {
                        regex_pattern.push_str(".*");
                    }
                } else {
                    regex_pattern.push_str("[^/]*");
                }
            },
            '?' => regex_pattern.push_str("[^/]"),
            '[' => {
                regex_pattern.push('[');
                if chars.peek() == Some(&'!') {
                    chars.next();
                    regex_pattern.push('^');
                }
                while let Some(bracket_char) = chars.next() {
                    if bracket_char == ']' {
                        regex_pattern.push(']');
                        break;
                    }
                    regex_pattern.push(bracket_char);
                }
            },
            '.' | '+' | '^' | '$' | '(' | ')' | '{' | '}' | '|' | '\\' => {
                regex_pattern.push('\\');
                regex_pattern.push(ch);
            },
            _ => regex_pattern.push(ch),
        }
    }
    
    regex_pattern.push('$');
    
    regex::Regex::new(&regex_pattern).unwrap_or_else(|_| {
        // Fallback to a simple pattern if regex compilation fails
        regex::Regex::new(".*").unwrap()
    })
}
