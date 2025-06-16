use crate::stdlib::embed_that::core::{ThatFile, tea, FileSystemVibe};
use crate::stdlib::embed_that::error::{EmbedError, EmbedResult};
use std::io::Read;

/// Specific resource loaders for various file types
pub struct SpecificLoaders;

impl SpecificLoaders {
    /// Load an image from an embedded file
    pub fn load_image(path: &tea) -> EmbedResult<ImageData> {
        let file = super::resource_loader::load_that_file(path)?;
        
        if !file.is_image() {
            return Err(EmbedError::InvalidFormat { 
                file: path.clone(), 
                reason: "File is not an image".to_string() 
            });
        }
        
        let content = file.content();
        Self::decode_image_data(&content, &file.mime_type())
    }
    
    /// Load an image from an embedded filesystem
    pub fn load_image_fs(fs: &dyn FileSystemVibe, path: &tea) -> EmbedResult<ImageData> {
        let content = fs.read_file(path)?;
        
        // Detect image type from content or file extension
        let mime_type = detect_image_mime_type(&content, path);
        Self::decode_image_data(&content, &mime_type)
    }
    
    /// Load and parse JSON from an embedded file
    pub fn load_json<T>(path: &tea, target: &mut T) -> EmbedResult<()>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content_string()?;
        
        let parsed: T = serde_json::from_str(&content)
            .map_err(|e| EmbedError::JsonParsingError { reason: e.to_string() })?;
        
        *target = parsed;
        Ok(())
    }
    
    /// Load and parse YAML from an embedded file
    pub fn load_yaml<T>(path: &tea, target: &mut T) -> EmbedResult<()>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content_string()?;
        
        let parsed: T = serde_yaml::from_str(&content)
            .map_err(|e| EmbedError::YamlParsingError { reason: e.to_string() })?;
        
        *target = parsed;
        Ok(())
    }
    
    /// Load and parse TOML from an embedded file
    pub fn load_toml<T>(path: &tea, target: &mut T) -> EmbedResult<()>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content_string()?;
        
        let parsed: T = toml::from_str(&content)
            .map_err(|e| EmbedError::TomlParsingError { reason: e.to_string() })?;
        
        *target = parsed;
        Ok(())
    }
    
    /// Load a configuration file with auto-detection of format
    pub fn load_config<T>(path: &tea, target: &mut T) -> EmbedResult<()>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content_string()?;
        let extension = file.extension().to_lowercase();
        
        match extension.as_str() {
            "json" => {
                let parsed: T = serde_json::from_str(&content)
                    .map_err(|e| EmbedError::JsonParsingError { reason: e.to_string() })?;
                *target = parsed;
            },
            "yaml" | "yml" => {
                let parsed: T = serde_yaml::from_str(&content)
                    .map_err(|e| EmbedError::YamlParsingError { reason: e.to_string() })?;
                *target = parsed;
            },
            "toml" => {
                let parsed: T = toml::from_str(&content)
                    .map_err(|e| EmbedError::TomlParsingError { reason: e.to_string() })?;
                *target = parsed;
            },
            _ => {
                // Try each format in order
                if let Ok(parsed) = serde_json::from_str::<T>(&content) {
                    *target = parsed;
                } else if let Ok(parsed) = serde_yaml::from_str::<T>(&content) {
                    *target = parsed;
                } else if let Ok(parsed) = toml::from_str::<T>(&content) {
                    *target = parsed;
                } else {
                    return Err(EmbedError::ConfigParsingError { 
                        reason: "Unable to parse as JSON, YAML, or TOML".to_string() 
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Load a text file with specific encoding handling
    pub fn load_text_file(path: &tea) -> EmbedResult<tea> {
        let file = super::resource_loader::load_that_file(path)?;
        
        if !file.is_text() {
            return Err(EmbedError::InvalidFormat { 
                file: path.clone(), 
                reason: "File is not a text file".to_string() 
            });
        }
        
        file.content_string()
    }
    
    /// Load a binary file as raw bytes
    pub fn load_binary_file(path: &tea) -> EmbedResult<Vec<u8>> {
        let file = super::resource_loader::load_that_file(path)?;
        Ok(file.content())
    }
    
    /// Load a CSS file with basic validation
    pub fn load_css(path: &tea) -> EmbedResult<CssData> {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content_string()?;
        
        // Basic CSS validation (check for basic CSS syntax)
        if !content.contains('{') || !content.contains('}') {
            return Err(EmbedError::InvalidFormat { 
                file: path.clone(), 
                reason: "Invalid CSS format".to_string() 
            });
        }
        
        Ok(CssData {
            content,
            minified: false,
            source_map: None,
        })
    }
    
    /// Load a JavaScript file with basic validation
    pub fn load_javascript(path: &tea) -> EmbedResult<JavaScriptData> {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content_string()?;
        
        // Basic JavaScript validation (very simple check)
        let has_js_keywords = content.contains("function") || 
                             content.contains("var") || 
                             content.contains("let") || 
                             content.contains("const") ||
                             content.contains("=>");
        
        Ok(JavaScriptData {
            content,
            minified: false,
            source_map: None,
            is_module: has_js_keywords,
        })
    }
    
    /// Load an HTML file with basic validation
    pub fn load_html(path: &tea) -> EmbedResult<HtmlData> {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content_string()?;
        
        // Basic HTML validation
        let has_html_tags = content.contains('<') && content.contains('>');
        
        Ok(HtmlData {
            content,
            title: extract_html_title(&content),
            has_doctype: content.trim_start().starts_with("<!DOCTYPE") || 
                        content.trim_start().starts_with("<!doctype"),
            is_valid: has_html_tags,
        })
    }
    
    /// Load a font file
    pub fn load_font(path: &tea) -> EmbedResult<FontData> {
        let file = super::resource_loader::load_that_file(path)?;
        let content = file.content();
        let extension = file.extension().to_lowercase();
        
        let font_type = match extension.as_str() {
            "ttf" => FontType::TrueType,
            "otf" => FontType::OpenType,
            "woff" => FontType::Woff,
            "woff2" => FontType::Woff2,
            "eot" => FontType::Eot,
            _ => return Err(EmbedError::InvalidFormat { 
                file: path.clone(), 
                reason: "Unsupported font format".to_string() 
            }),
        };
        
        Ok(FontData {
            content,
            font_type,
            size: file.size() as usize,
        })
    }
    
    /// Load an audio file
    pub fn load_audio(path: &tea) -> EmbedResult<AudioData> {
        let file = super::resource_loader::load_that_file(path)?;
        
        if !file.is_audio() {
            return Err(EmbedError::InvalidFormat { 
                file: path.clone(), 
                reason: "File is not an audio file".to_string() 
            });
        }
        
        let content = file.content();
        let extension = file.extension().to_lowercase();
        
        let audio_type = match extension.as_str() {
            "mp3" => AudioType::Mp3,
            "wav" => AudioType::Wav,
            "ogg" => AudioType::Ogg,
            "flac" => AudioType::Flac,
            "aac" => AudioType::Aac,
            _ => AudioType::Unknown,
        };
        
        Ok(AudioData {
            content,
            audio_type,
            size: file.size() as usize,
            duration: None, // Would require audio parsing library
        })
    }
    
    /// Load a video file
    pub fn load_video(path: &tea) -> EmbedResult<VideoData> {
        let file = super::resource_loader::load_that_file(path)?;
        
        if !file.is_video() {
            return Err(EmbedError::InvalidFormat { 
                file: path.clone(), 
                reason: "File is not a video file".to_string() 
            });
        }
        
        let content = file.content();
        let extension = file.extension().to_lowercase();
        
        let video_type = match extension.as_str() {
            "mp4" => VideoType::Mp4,
            "avi" => VideoType::Avi,
            "mov" => VideoType::Mov,
            "wmv" => VideoType::Wmv,
            "flv" => VideoType::Flv,
            "webm" => VideoType::WebM,
            _ => VideoType::Unknown,
        };
        
        Ok(VideoData {
            content,
            video_type,
            size: file.size() as usize,
            duration: None, // Would require video parsing library
            resolution: None,
        })
    }
    
    /// Decode image data from raw bytes
    fn decode_image_data(content: &[u8], mime_type: &str) -> EmbedResult<ImageData> {
        // For now, return basic image data
        // In a full implementation, this would use an image processing library
        let image_type = match mime_type {
            "image/png" => ImageType::Png,
            "image/jpeg" => ImageType::Jpeg,
            "image/gif" => ImageType::Gif,
            "image/svg+xml" => ImageType::Svg,
            "image/webp" => ImageType::WebP,
            _ => ImageType::Unknown,
        };
        
        Ok(ImageData {
            content: content.to_vec(),
            image_type,
            width: None,   // Would require image processing
            height: None,  // Would require image processing
            size: content.len(),
        })
    }
}

/// Image data structure
#[derive(Debug, Clone)]
pub struct ImageData {
    pub content: Vec<u8>,
    pub image_type: ImageType,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Png,
    Jpeg,
    Gif,
    Svg,
    WebP,
    Unknown,
}

/// CSS data structure
#[derive(Debug, Clone)]
pub struct CssData {
    pub content: tea,
    pub minified: bool,
    pub source_map: Option<tea>,
}

/// JavaScript data structure
#[derive(Debug, Clone)]
pub struct JavaScriptData {
    pub content: tea,
    pub minified: bool,
    pub source_map: Option<tea>,
    pub is_module: bool,
}

/// HTML data structure
#[derive(Debug, Clone)]
pub struct HtmlData {
    pub content: tea,
    pub title: Option<tea>,
    pub has_doctype: bool,
    pub is_valid: bool,
}

/// Font data structure
#[derive(Debug, Clone)]
pub struct FontData {
    pub content: Vec<u8>,
    pub font_type: FontType,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub enum FontType {
    TrueType,
    OpenType,
    Woff,
    Woff2,
    Eot,
}

/// Audio data structure
#[derive(Debug, Clone)]
pub struct AudioData {
    pub content: Vec<u8>,
    pub audio_type: AudioType,
    pub size: usize,
    pub duration: Option<f64>, // in seconds
}

#[derive(Debug, Clone)]
pub enum AudioType {
    Mp3,
    Wav,
    Ogg,
    Flac,
    Aac,
    Unknown,
}

/// Video data structure
#[derive(Debug, Clone)]
pub struct VideoData {
    pub content: Vec<u8>,
    pub video_type: VideoType,
    pub size: usize,
    pub duration: Option<f64>, // in seconds
    pub resolution: Option<(u32, u32)>, // width, height
}

#[derive(Debug, Clone)]
pub enum VideoType {
    Mp4,
    Avi,
    Mov,
    Wmv,
    Flv,
    WebM,
    Unknown,
}

/// Detect image MIME type from content or extension
fn detect_image_mime_type(content: &[u8], path: &str) -> tea {
    // Check magic bytes first
    if content.len() >= 8 {
        // PNG signature
        if &content[0..8] == b"\x89PNG\r\n\x1a\n" {
            return "image/png".to_string();
        }
        
        // JPEG signature
        if content.len() >= 2 && &content[0..2] == b"\xff\xd8" {
            return "image/jpeg".to_string();
        }
        
        // GIF signature
        if content.len() >= 6 && (&content[0..6] == b"GIF87a" || &content[0..6] == b"GIF89a") {
            return "image/gif".to_string();
        }
        
        // WebP signature
        if content.len() >= 12 && &content[0..4] == b"RIFF" && &content[8..12] == b"WEBP" {
            return "image/webp".to_string();
        }
    }
    
    // Fall back to extension-based detection
    if let Some(pos) = path.rfind('.') {
        let extension = &path[pos + 1..].to_lowercase();
        match extension {
            "png" => "image/png".to_string(),
            "jpg" | "jpeg" => "image/jpeg".to_string(),
            "gif" => "image/gif".to_string(),
            "svg" => "image/svg+xml".to_string(),
            "webp" => "image/webp".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    } else {
        "application/octet-stream".to_string()
    }
}

/// Extract title from HTML content
fn extract_html_title(content: &str) -> Option<tea> {
    // Simple regex-free title extraction
    let content_lower = content.to_lowercase();
    if let Some(start) = content_lower.find("<title>") {
        if let Some(end) = content_lower[start + 7..].find("</title>") {
            let title_start = start + 7;
            let title_end = title_start + end;
            if title_end <= content.len() {
                return Some(content[title_start..title_end].trim().to_string());
            }
        }
    }
    None
}

/// Public API functions for specific loaders
pub fn load_image(path: &tea) -> EmbedResult<ImageData> {
    SpecificLoaders::load_image(path)
}

pub fn load_image_fs(fs: &dyn FileSystemVibe, path: &tea) -> EmbedResult<ImageData> {
    SpecificLoaders::load_image_fs(fs, path)
}

pub fn load_json<T>(path: &tea, target: &mut T) -> EmbedResult<()>
where
    T: serde::de::DeserializeOwned + Clone,
{
    SpecificLoaders::load_json(path, target)
}

pub fn load_yaml<T>(path: &tea, target: &mut T) -> EmbedResult<()>
where
    T: serde::de::DeserializeOwned + Clone,
{
    SpecificLoaders::load_yaml(path, target)
}

pub fn load_toml<T>(path: &tea, target: &mut T) -> EmbedResult<()>
where
    T: serde::de::DeserializeOwned + Clone,
{
    SpecificLoaders::load_toml(path, target)
}

pub fn load_config<T>(path: &tea, target: &mut T) -> EmbedResult<()>
where
    T: serde::de::DeserializeOwned + Clone,
{
    SpecificLoaders::load_config(path, target)
}

pub fn load_text_file(path: &tea) -> EmbedResult<tea> {
    SpecificLoaders::load_text_file(path)
}

pub fn load_binary_file(path: &tea) -> EmbedResult<Vec<u8>> {
    SpecificLoaders::load_binary_file(path)
}

pub fn load_css(path: &tea) -> EmbedResult<CssData> {
    SpecificLoaders::load_css(path)
}

pub fn load_javascript(path: &tea) -> EmbedResult<JavaScriptData> {
    SpecificLoaders::load_javascript(path)
}

pub fn load_html(path: &tea) -> EmbedResult<HtmlData> {
    SpecificLoaders::load_html(path)
}

pub fn load_font(path: &tea) -> EmbedResult<FontData> {
    SpecificLoaders::load_font(path)
}

pub fn load_audio(path: &tea) -> EmbedResult<AudioData> {
    SpecificLoaders::load_audio(path)
}

pub fn load_video(path: &tea) -> EmbedResult<VideoData> {
    SpecificLoaders::load_video(path)
}
