/// Template Streaming Renderer - High-performance streaming template rendering
use std::collections::HashMap;
use std::io::{Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter as AsyncBufWriter};
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use futures::{Stream, StreamExt};
use tracing::{debug, error, info, instrument, span, Level};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use super::template_core::{TemplateContext, TemplateConfig, TemplateLoader};
use super::template_syntax::{TemplateAst, TemplateNode, TemplateExpression};
use super::template_render::{TemplateRenderer, RenderContext, SecurityLevel, OutputFormat};
use super::template_filters::FilterRegistry;

/// Streaming render configuration
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// Buffer size for streaming output
    pub buffer_size: usize,
    /// Chunk size for streaming data
    pub chunk_size: usize,
    /// Enable async rendering
    pub enable_async: bool,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Streaming timeout
    pub stream_timeout: Duration,
    /// Enable compression
    pub enable_compression: bool,
    /// Memory pressure threshold
    pub memory_pressure_threshold: usize,
    /// Enable progressive rendering
    pub enable_progressive_rendering: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            chunk_size: 4096,
            enable_async: true,
            max_concurrent_operations: 8,
            stream_timeout: Duration::from_secs(30),
            enable_compression: false,
            memory_pressure_threshold: 50 * 1024 * 1024, // 50MB
            enable_progressive_rendering: true,
        }
    }
}

/// Streaming render result
#[derive(Debug)]
pub struct StreamingResult {
    /// Total bytes written
    pub bytes_written: usize,
    /// Rendering duration
    pub render_time: Duration,
    /// Number of chunks processed
    pub chunks_processed: usize,
    /// Memory high water mark
    pub memory_high_water_mark: usize,
    /// Whether compression was used
    pub compression_used: bool,
    /// Stream completion status
    pub completed_successfully: bool,
}

/// Chunk types for streaming
#[derive(Debug, Clone)]
pub enum StreamChunk {
    /// Text content
    Text(String),
    /// Raw HTML content
    Html(String),
    /// JSON data
    Json(String),
    /// Binary data
    Binary(Vec<u8>),
    /// Control chunk (flush, end, etc.)
    Control(ControlCommand),
}

/// Control commands for streaming
#[derive(Debug, Clone)]
pub enum ControlCommand {
    /// Flush output buffer
    Flush,
    /// End of stream
    End,
    /// Error occurred
    Error(String),
    /// Progress update
    Progress(f32),
}

/// Async streaming template renderer
pub struct StreamingTemplateRenderer {
    /// Base template renderer
    base_renderer: TemplateRenderer,
    /// Streaming configuration
    config: StreamingConfig,
    /// Concurrency semaphore
    semaphore: Arc<Semaphore>,
    /// Performance metrics
    metrics: Arc<Mutex<StreamingMetrics>>,
}

/// Streaming performance metrics
#[derive(Debug, Clone)]
pub struct StreamingMetrics {
    /// Total streams processed
    pub total_streams: u64,
    /// Average streaming time
    pub average_stream_time: Duration,
    /// Total bytes streamed
    pub total_bytes_streamed: u64,
    /// Peak concurrent streams
    pub peak_concurrent_streams: usize,
    /// Stream errors
    pub stream_errors: u64,
    /// Compression ratio (when enabled)
    pub compression_ratio: f64,
}

impl StreamingTemplateRenderer {
    /// Create a new streaming renderer
    pub fn new(
        filters: Arc<FilterRegistry>,
        loader: Arc<dyn TemplateLoader>,
        template_config: &TemplateConfig,
        streaming_config: StreamingConfig,
    ) -> Self {
        let base_renderer = TemplateRenderer::new(filters, loader, template_config);
        let semaphore = Arc::new(Semaphore::new(streaming_config.max_concurrent_operations));
        
        Self {
            base_renderer,
            config: streaming_config,
            semaphore,
            metrics: Arc::new(Mutex::new(StreamingMetrics {
                total_streams: 0,
                average_stream_time: Duration::from_secs(0),
                total_bytes_streamed: 0,
                peak_concurrent_streams: 0,
                stream_errors: 0,
                compression_ratio: 1.0,
            })),
        }
    }
    
    /// Stream template to a writer
    #[instrument(skip(self, ast, context, writer))]
    pub async fn stream_to_writer<W: AsyncWrite + Unpin>(
        &self,
        ast: &TemplateAst,
        context: RenderContext,
        writer: W,
    ) -> Result<StreamingResult, CursedError> {
        let start_time = Instant::now();
        info!("Starting streaming template render");
        
        // Acquire semaphore for concurrency control
        let _permit = self.semaphore.acquire().await
            .map_err(|e| CursedError::TemplateError {
                message: format!("Failed to acquire streaming permit: {}", e),
                source_location: None,
            })?;
        
        let mut buf_writer = AsyncBufWriter::with_capacity(self.config.buffer_size, writer);
        let mut chunks_processed = 0;
        let mut bytes_written = 0;
        let mut memory_usage = 0;
        
        // Create streaming context
        let (chunk_sender, mut chunk_receiver) = mpsc::channel::<StreamChunk>(self.config.max_concurrent_operations);
        
        // Start background template processing
        let processing_handle = self.start_background_processing(ast.clone(), context, chunk_sender).await?;
        
        // Process chunks and write to output
        while let Some(chunk) = chunk_receiver.recv().await {
            match chunk {
                StreamChunk::Text(text) => {
                    buf_writer.write_all(text.as_bytes()).await
                        .map_err(|e| CursedError::TemplateError {
                            message: format!("Failed to write text chunk: {}", e),
                            source_location: None,
                        })?;
                    bytes_written += text.len();
                    memory_usage += text.len();
                }
                StreamChunk::Html(html) => {
                    buf_writer.write_all(html.as_bytes()).await
                        .map_err(|e| CursedError::TemplateError {
                            message: format!("Failed to write HTML chunk: {}", e),
                            source_location: None,
                        })?;
                    bytes_written += html.len();
                    memory_usage += html.len();
                }
                StreamChunk::Json(json) => {
                    buf_writer.write_all(json.as_bytes()).await
                        .map_err(|e| CursedError::TemplateError {
                            message: format!("Failed to write JSON chunk: {}", e),
                            source_location: None,
                        })?;
                    bytes_written += json.len();
                    memory_usage += json.len();
                }
                StreamChunk::Binary(data) => {
                    buf_writer.write_all(&data).await
                        .map_err(|e| CursedError::TemplateError {
                            message: format!("Failed to write binary chunk: {}", e),
                            source_location: None,
                        })?;
                    bytes_written += data.len();
                    memory_usage += data.len();
                }
                StreamChunk::Control(ControlCommand::Flush) => {
                    buf_writer.flush().await
                        .map_err(|e| CursedError::TemplateError {
                            message: format!("Failed to flush output: {}", e),
                            source_location: None,
                        })?;
                }
                StreamChunk::Control(ControlCommand::End) => {
                    break;
                }
                StreamChunk::Control(ControlCommand::Error(error_msg)) => {
                    return Err(CursedError::TemplateError {
                        message: format!("Streaming error: {}", error_msg),
                        source_location: None,
                    });
                }
                StreamChunk::Control(ControlCommand::Progress(progress)) => {
                    debug!(progress = progress, "Streaming progress update");
                }
            }
            
            chunks_processed += 1;
            
            // Check memory pressure
            if memory_usage > self.config.memory_pressure_threshold {
                buf_writer.flush().await
                    .map_err(|e| CursedError::TemplateError {
                        message: format!("Failed to flush under memory pressure: {}", e),
                        source_location: None,
                    })?;
                memory_usage = 0;
            }
        }
        
        // Final flush
        buf_writer.flush().await
            .map_err(|e| CursedError::TemplateError {
                message: format!("Failed to flush final output: {}", e),
                source_location: None,
            })?;
        
        // Wait for background processing to complete
        let processing_completed = processing_handle.await
            .map_err(|e| CursedError::TemplateError {
                message: format!("Background processing failed: {}", e),
                source_location: None,
            })??;
        
        let render_time = start_time.elapsed();
        
        // Update metrics
        self.update_streaming_metrics(bytes_written, render_time, chunks_processed);
        
        info!(
            bytes_written = bytes_written,
            chunks_processed = chunks_processed,
            render_time_ms = render_time.as_millis(),
            "Streaming template render completed"
        );
        
        Ok(StreamingResult {
            bytes_written,
            render_time,
            chunks_processed,
            memory_high_water_mark: memory_usage,
            compression_used: self.config.enable_compression,
            completed_successfully: processing_completed,
        })
    }
    
    /// Start background template processing
    async fn start_background_processing(
        &self,
        ast: TemplateAst,
        context: RenderContext,
        chunk_sender: mpsc::Sender<StreamChunk>,
    ) -> Result<JoinHandle<Result<bool, CursedError>>, CursedError> {
        let config = self.config.clone();
        
        let handle = tokio::spawn(async move {
            let processing_span = span!(Level::DEBUG, "background_processing");
            let _enter = processing_span.enter();
            
            debug!("Starting background template processing");
            
            // Process template nodes into chunks
            for (index, node) in ast.nodes.iter().enumerate() {
                match Self::process_node_to_chunks(node, &context, &config).await {
                    Ok(chunks) => {
                        for chunk in chunks {
                            if chunk_sender.send(chunk).await.is_err() {
                                warn!("Failed to send chunk - receiver may be closed");
                                return Ok(false);
                            }
                        }
                    }
                    Err(e) => {
                        error!(error = ?e, node_index = index, "Failed to process template node");
                        let _ = chunk_sender.send(StreamChunk::Control(ControlCommand::Error(e.to_string()))).await;
                        return Err(e);
                    }
                }
                
                // Send progress updates
                if config.enable_progressive_rendering {
                    let progress = (index + 1) as f32 / ast.nodes.len() as f32;
                    let _ = chunk_sender.send(StreamChunk::Control(ControlCommand::Progress(progress))).await;
                }
            }
            
            // Send end marker
            let _ = chunk_sender.send(StreamChunk::Control(ControlCommand::End)).await;
            
            debug!("Background template processing completed");
            Ok(true)
        });
        
        Ok(handle)
    }
    
    /// Process a template node into chunks
    async fn process_node_to_chunks(
        node: &TemplateNode,
        context: &RenderContext,
        config: &StreamingConfig,
    ) -> Result<Vec<StreamChunk>, CursedError> {
        let mut chunks = Vec::new();
        
        match node {
            TemplateNode::Text(text) => {
                // Split large text into chunks
                if text.len() > config.chunk_size {
                    for chunk in text.as_bytes().chunks(config.chunk_size) {
                        let chunk_text = String::from_utf8_lossy(chunk).to_string();
                        chunks.push(StreamChunk::Text(chunk_text));
                    }
                } else {
                    chunks.push(StreamChunk::Text(text.clone()));
                }
            }
            TemplateNode::Variable { name, filters } => {
                // Resolve variable and apply filters
                if let Some(value) = context.get(name) {
                    let processed_value = Self::apply_filters_to_value(&value, filters, context).await?;
                    let text_value = Self::object_to_string(&processed_value)?;
                    
                    // Apply security escaping based on output format
                    let escaped_value = Self::apply_security_escaping(&text_value, context)?;
                    
                    chunks.push(match context.output_format {
                        OutputFormat::Html => StreamChunk::Html(escaped_value),
                        OutputFormat::Json => StreamChunk::Json(escaped_value),
                        _ => StreamChunk::Text(escaped_value),
                    });
                } else {
                    // Variable not found - emit empty chunk or error based on strict mode
                    chunks.push(StreamChunk::Text(String::new()));
                }
            }
            TemplateNode::Block { block_type, content, .. } => {
                // Process block content recursively
                if let Some(content_nodes) = content {
                    for content_node in content_nodes {
                        let mut node_chunks = Self::process_node_to_chunks(content_node, context, config).await?;
                        chunks.append(&mut node_chunks);
                    }
                }
            }
            TemplateNode::Comment(_) => {
                // Comments produce no output
            }
        }
        
        Ok(chunks)
    }
    
    /// Apply filters to a value (simplified version for streaming)
    async fn apply_filters_to_value(
        value: &CursedObject,
        _filters: &[String],
        _context: &RenderContext,
    ) -> Result<CursedObject, CursedError> {
        // For streaming, we'll use a simplified filter application
        // In a full implementation, this would use the FilterRegistry
        Ok(value.clone())
    }
    
    /// Apply security escaping
    fn apply_security_escaping(text: &str, context: &RenderContext) -> Result<String, CursedError> {
        match context.security_level {
            SecurityLevel::Strict | SecurityLevel::Moderate => {
                match context.output_format {
                    OutputFormat::Html => Ok(Self::escape_html(text)),
                    OutputFormat::Xml => Ok(Self::escape_xml(text)),
                    OutputFormat::Json => Ok(Self::escape_json(text)),
                    _ => Ok(text.to_string()),
                }
            }
            SecurityLevel::Relaxed => Ok(text.to_string()),
        }
    }
    
    /// Convert object to string
    fn object_to_string(obj: &CursedObject) -> Result<String, CursedError> {
        match obj {
            CursedObject::String(s) => Ok(s.clone()),
            CursedObject::Integer(n) => Ok(n.to_string()),
            CursedObject::Float(n) => Ok(n.to_string()),
            CursedObject::Boolean(b) => Ok(b.to_string()),
            CursedObject::Char(c) => Ok(c.to_string()),
            CursedObject::Nil => Ok(String::new()),
            CursedObject::Array(arr) => {
                let items: Vec<String> = arr.iter()
                    .map(|item| Self::object_to_string(item))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(format!("[{}]", items.join(", ")))
            }
            CursedObject::Map(map) => {
                let items: Result<Vec<String>, CursedError> = map.iter()
                    .map(|(k, v)| Ok(format!("{}: {}", k, Self::object_to_string(v)?)))
                    .collect();
                Ok(format!("{{{}}}", items?.join(", ")))
            }
        }
    }
    
    /// Escape HTML characters
    fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
    }
    
    /// Escape XML characters
    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&apos;")
    }
    
    /// Escape JSON characters
    fn escape_json(s: &str) -> String {
        s.replace('\\', "\\\\")
         .replace('"', "\\\"")
         .replace('\n', "\\n")
         .replace('\r', "\\r")
         .replace('\t', "\\t")
    }
    
    /// Update streaming metrics
    fn update_streaming_metrics(&self, bytes_written: usize, render_time: Duration, chunks_processed: usize) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.total_streams += 1;
            metrics.total_bytes_streamed += bytes_written as u64;
            
            // Update average stream time
            let total_time = (metrics.average_stream_time.as_nanos() * (metrics.total_streams - 1) as u128) + render_time.as_nanos();
            metrics.average_stream_time = Duration::from_nanos((total_time / metrics.total_streams as u128) as u64);
        }
    }
    
    /// Get streaming metrics
    pub fn get_metrics(&self) -> Option<StreamingMetrics> {
        self.metrics.lock().ok().map(|m| m.clone())
    }
    
    /// Stream template to a string (for testing)
    pub async fn stream_to_string(
        &self,
        ast: &TemplateAst,
        context: RenderContext,
    ) -> Result<(String, StreamingResult), CursedError> {
        let mut output = Vec::new();
        let result = self.stream_to_writer(ast, context, &mut output).await?;
        
        let output_string = String::from_utf8(output)
            .map_err(|e| CursedError::TemplateError {
                message: format!("Failed to convert output to UTF-8: {}", e),
                source_location: None,
            })?;
        
        Ok((output_string, result))
    }
}

/// Async template stream - provides a Stream interface for template rendering
pub struct TemplateStream {
    /// Chunk receiver
    chunk_receiver: mpsc::Receiver<StreamChunk>,
    /// Background processing handle
    _processing_handle: JoinHandle<Result<bool, CursedError>>,
}

impl TemplateStream {
    /// Create a new template stream
    pub async fn new(
        renderer: &StreamingTemplateRenderer,
        ast: TemplateAst,
        context: RenderContext,
    ) -> Result<Self, CursedError> {
        let (chunk_sender, chunk_receiver) = mpsc::channel::<StreamChunk>(renderer.config.max_concurrent_operations);
        let processing_handle = renderer.start_background_processing(ast, context, chunk_sender).await?;
        
        Ok(Self {
            chunk_receiver,
            _processing_handle: processing_handle,
        })
    }
}

impl Stream for TemplateStream {
    type Item = Result<StreamChunk, CursedError>;
    
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.chunk_receiver.poll_recv(cx) {
            std::task::Poll::Ready(Some(chunk)) => std::task::Poll::Ready(Some(Ok(chunk))),
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::template::template_syntax::{TemplateLexer, TemplateParser};
    use crate::stdlib::template::template_core::{TemplateDelimiters, FileSystemLoader};
    use crate::stdlib::template::template_filters::FilterRegistry;
    use tokio::io::sink;
    
    #[tokio::test]
    async fn test_streaming_renderer_creation() {
        let filters = Arc::new(FilterRegistry::new());
        let loader = Arc::new(FileSystemLoader::new("templates"));
        let template_config = TemplateConfig::default();
        let streaming_config = StreamingConfig::default();
        
        let renderer = StreamingTemplateRenderer::new(filters, loader, &template_config, streaming_config);
        
        assert_eq!(renderer.config.buffer_size, 8192);
        assert!(renderer.config.enable_async);
    }
    
    #[tokio::test]
    async fn test_simple_streaming() {
        let filters = Arc::new(FilterRegistry::new());
        let loader = Arc::new(FileSystemLoader::new("templates"));
        let template_config = TemplateConfig::default();
        let streaming_config = StreamingConfig::default();
        
        let renderer = StreamingTemplateRenderer::new(filters, loader, &template_config, streaming_config);
        
        let template_source = "Hello, {{ name }}!";
        let delimiters = TemplateDelimiters {
            variable: ("{{".to_string(), "}}".to_string()),
            block: ("{%".to_string(), "%}".to_string()),
            comment: ("{#".to_string(), "#}".to_string()),
        };
        
        let mut lexer = TemplateLexer::new(template_source, &delimiters);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = RenderContext::new();
        context.set("name".to_string(), CursedObject::String("World".to_string())).unwrap();
        
        let (output, result) = renderer.stream_to_string(&ast, context).await.unwrap();
        
        assert_eq!(output, "Hello, World!");
        assert!(result.completed_successfully);
        assert!(result.bytes_written > 0);
    }
    
    #[tokio::test]
    async fn test_streaming_to_writer() {
        let filters = Arc::new(FilterRegistry::new());
        let loader = Arc::new(FileSystemLoader::new("templates"));
        let template_config = TemplateConfig::default();
        let streaming_config = StreamingConfig::default();
        
        let renderer = StreamingTemplateRenderer::new(filters, loader, &template_config, streaming_config);
        
        let template_source = "{{ message }}";
        let delimiters = TemplateDelimiters {
            variable: ("{{".to_string(), "}}".to_string()),
            block: ("{%".to_string(), "%}".to_string()),
            comment: ("{#".to_string(), "#}".to_string()),
        };
        
        let mut lexer = TemplateLexer::new(template_source, &delimiters);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = RenderContext::new();
        context.set("message".to_string(), CursedObject::String("Streaming works!".to_string())).unwrap();
        
        let mut output = Vec::new();
        let result = renderer.stream_to_writer(&ast, context, &mut output).await.unwrap();
        
        let output_string = String::from_utf8(output).unwrap();
        assert_eq!(output_string, "Streaming works!");
        assert!(result.completed_successfully);
    }
    
    #[tokio::test]
    async fn test_chunk_processing() {
        let config = StreamingConfig::default();
        let context = RenderContext::new();
        
        let text_node = TemplateNode::Text("Hello, World!".to_string());
        let chunks = StreamingTemplateRenderer::process_node_to_chunks(&text_node, &context, &config).await.unwrap();
        
        assert_eq!(chunks.len(), 1);
        match &chunks[0] {
            StreamChunk::Text(text) => assert_eq!(text, "Hello, World!"),
            _ => panic!("Expected text chunk"),
        }
    }
    
    #[test]
    fn test_security_escaping() {
        let context = RenderContext::new()
            .with_security_level(SecurityLevel::Strict)
            .with_output_format(OutputFormat::Html);
        
        let input = "<script>alert('xss')</script>";
        let escaped = StreamingTemplateRenderer::apply_security_escaping(input, &context).unwrap();
        
        assert!(escaped.contains("&lt;script&gt;"));
        assert!(escaped.contains("&lt;/script&gt;"));
    }
}
