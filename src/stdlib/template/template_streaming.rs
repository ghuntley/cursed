use crate::error::CursedError;
/// Template Streaming Renderer - High-performance streaming template rendering
use std::collections::HashMap;
use std::io::{Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter as AsyncBufWriter};
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use futures::{Stream, StreamExt};
use tracing::{debug, error, info, instrument, span, warn, Level};

use crate::object::Object as CursedObject;
use super::template_core::{TemplateContext, TemplateConfig, TemplateLoader};
use super::template_syntax::{TemplateAst, TemplateNode, TemplateExpression, BlockNode};
use super::template_render::{TemplateRenderer, RenderContext, SecurityLevel, OutputFormat};
use super::template_filters::FilterRegistry;

/// Streaming render configuration
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// Buffer size for streaming output
    /// Chunk size for streaming data
    /// Enable async rendering
    /// Maximum concurrent operations
    /// Streaming timeout
    /// Enable compression
    /// Memory pressure threshold
    /// Enable progressive rendering
impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            memory_pressure_threshold: 50 * 1024 * 1024, // 50MB
        }
    }
/// Streaming render result
#[derive(Debug)]
pub struct StreamingResult {
    /// Total bytes written
    /// Rendering duration
    /// Number of chunks processed
    /// Memory high water mark
    /// Whether compression was used
    /// Stream completion status
/// Chunk types for streaming
#[derive(Debug, Clone)]
pub enum StreamChunk {
    /// Text content
    /// Raw HTML content
    /// JSON data
    /// Binary data
    /// Control chunk (flush, end, etc.)
/// Control commands for streaming
#[derive(Debug, Clone)]
pub enum ControlCommand {
    /// Flush output buffer
    /// End of stream
    /// CursedError occurred
    /// Progress update
/// Async streaming template renderer
pub struct StreamingTemplateRenderer {
    /// Base template renderer
    /// Streaming configuration
    /// Concurrency semaphore
    /// Performance metrics
/// Streaming performance metrics
#[derive(Debug, Clone)]
pub struct StreamingMetrics {
    /// Total streams processed
    /// Average streaming time
    /// Total bytes streamed
    /// Peak concurrent streams
    /// Stream errors
    /// Compression ratio (when enabled)
impl StreamingTemplateRenderer {
    /// Create a new streaming renderer
    pub fn new(
    ) -> Self {
        let base_renderer = TemplateRenderer::new(filters, loader, template_config);
        let semaphore = Arc::new(Semaphore::new(streaming_config.max_concurrent_operations));
        
        Self {
            metrics: Arc::new(Mutex::new(StreamingMetrics {
        }
    }
    
    /// Stream template to a writer
    #[instrument(skip(self, ast, context, writer))]
    pub async fn stream_to_writer<W: AsyncWrite + Unpin>(
    ) -> crate::error::Result<StreamingResult> {
        let start_time = Instant::now();
        info!("Starting streaming template render");
        
        // Acquire semaphore for concurrency control
        let _permit = self.semaphore.acquire().await
            .map_err(|e| CursedError::TemplateError {
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
                        })?;
                    bytes_written += text.len();
                    memory_usage += text.len();
                }
                StreamChunk::Html(html) => {
                    buf_writer.write_all(html.as_bytes()).await
                        .map_err(|e| CursedError::TemplateError {
                        })?;
                    bytes_written += html.len();
                    memory_usage += html.len();
                }
                StreamChunk::Json(json) => {
                    buf_writer.write_all(json.as_bytes()).await
                        .map_err(|e| CursedError::TemplateError {
                        })?;
                    bytes_written += json.len();
                    memory_usage += json.len();
                }
                StreamChunk::Binary(data) => {
                    buf_writer.write_all(&data).await
                        .map_err(|e| CursedError::TemplateError {
                        })?;
                    bytes_written += data.len();
                    memory_usage += data.len();
                }
                StreamChunk::Control(ControlCommand::Flush) => {
                    buf_writer.flush().await
                        .map_err(|e| CursedError::TemplateError {
                        })?;
                }
                StreamChunk::Control(ControlCommand::End) => {
                    break;
                }
                StreamChunk::Control(ControlCommand::CursedError(error_msg)) => {
                    return Err(CursedError::TemplateError {
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
                    })?;
                memory_usage = 0;
            }
        }
        
        // Final flush
        buf_writer.flush().await
            .map_err(|e| CursedError::TemplateError {
            })?;
        
        // Wait for background processing to complete
        let processing_completed = processing_handle.await
            .map_err(|e| CursedError::TemplateError {
            })??;
        
        let render_time = start_time.elapsed();
        
        // Update metrics
        self.update_streaming_metrics(bytes_written, render_time, chunks_processed);
        
        info!(
            "Streaming template render completed"
        );
        
        Ok(StreamingResult {
        })
    /// Start background template processing
    async fn start_background_processing(
    ) -> crate::error::crate::error::Result<tokio::task::JoinHandle<Result<bool>>> {
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
                        let _ = chunk_sender.send(StreamChunk::Control(ControlCommand::CursedError(e.to_string()))).await;
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
    /// Process a template node into chunks
    async fn process_node_to_chunks(
    ) -> crate::error::Result<Vec<StreamChunk>> {
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
            TemplateNode::Variable { expression, filters, .. } => {
                // Resolve variable and apply filters  
                let name = match expression {
                if let Some(value) = context.get(&name) {
                    let processed_value = Self::apply_filters_to_value(&value, &filters.iter().map(|f| f.name.clone()).collect::<Vec<_>>(), context).await?;
                    let text_value = Self::object_to_string(&processed_value)?;
                    
                    // Apply security escaping based on output format
                    let escaped_value = Self::apply_security_escaping(&text_value, context)?;
                    
                    chunks.push(match context.output_format {
                    });
                } else {
                    // Variable not found - emit empty chunk or error based on strict mode
                    chunks.push(StreamChunk::Text(String::new()));
                }
            }
            TemplateNode::Block { block, .. } => {
                // Process block content recursively  
                let content_nodes = match block {
                if !content_nodes.is_empty() {
                    for content_node in content_nodes {
                        let mut node_chunks = Self::process_node_to_chunks(&content_node, context, config).await?;
                        chunks.append(&mut node_chunks);
                    }
                }
            }
            TemplateNode::Comment { .. } => {
                // Comments produce no output
            }
            TemplateNode::Include { .. } => {
                // Include templates - for streaming we'll skip this for now
                chunks.push(StreamChunk::Text("<!-- Include not supported in streaming mode -->".to_string()));
            }
            TemplateNode::Extends { .. } => {
                // Template inheritance - for streaming we'll skip this for now  
                chunks.push(StreamChunk::Text("<!-- Extends not supported in streaming mode -->".to_string()));
            }
            TemplateNode::BlockDef { .. } => {
                // Block definitions - for streaming we'll skip this for now
                chunks.push(StreamChunk::Text("<!-- BlockDef not supported in streaming mode -->".to_string()));
            }
            TemplateNode::Set { .. } => {
                // Variable assignment - for streaming we'll skip this for now
            }
            TemplateNode::Raw { .. } => {
                // Raw content - for streaming we'll skip this for now
                chunks.push(StreamChunk::Text("<!-- Raw not supported in streaming mode -->".to_string()));
            }
            TemplateNode::Filter { .. } => {
                // Filter blocks - for streaming we'll skip this for now
                chunks.push(StreamChunk::Text("<!-- Filter blocks not supported in streaming mode -->".to_string()));
            }
            TemplateNode::Macro { .. } => {
                // Macro definitions - for streaming we'll skip this for now
                chunks.push(StreamChunk::Text("<!-- Macros not supported in streaming mode -->".to_string()));
            }
        }
        
        Ok(chunks)
    /// Apply filters to a value (simplified version for streaming)
    async fn apply_filters_to_value(
    ) -> crate::error::Result<CursedObject> {
        // For streaming, we'll use a simplified filter application
        // In a full implementation, this would use the FilterRegistry
        Ok(value.clone())
    /// Apply security escaping
    fn apply_security_escaping(text: &str, context: &RenderContext) -> crate::error::Result<String> {
        match context.security_level {
            SecurityLevel::Strict | SecurityLevel::Moderate => {
                match context.output_format {
                }
            }
        }
    }
    
    /// Convert object to string
    fn object_to_string(obj: &CursedObject) -> crate::error::Result<String> {
        match obj {
            CursedObject::Array(arr) => {
                let items: Vec<String> = arr.iter()
                    .map(|item| Self::object_to_string(item))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(format!("[{}]", items.join(", ")))
            }
            CursedObject::Map(map) => {
                let items: crate::error::Result<()> = map.iter()
                    .map(|(k, v)| Ok(format!("{}: {}", k, Self::object_to_string(v)?)))
                    .collect();
                Ok(format!("{{{}}}", items?.join(", ")))
            }
        }
    /// Escape HTML characters
    fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
    /// Escape XML characters
    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&apos;")
    /// Escape JSON characters
    fn escape_json(s: &str) -> String {
        s.replace('\\', "\\\\")
         .replace('"', "\\\"")
         .replace('\n', "\\n")
         .replace('\r', "\\r")
         .replace('\t', "\\t")
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
    /// Stream template to a string (for testing)
    pub async fn stream_to_string(
    ) -> crate::error::Result<()> {
        let mut output = Vec::new();
        let result = self.stream_to_writer(ast, context, &mut output).await?;
        
        let output_string = String::from_utf8(output)
            .map_err(|e| CursedError::TemplateError {
            })?;
        
        Ok((output_string, result))
    }
}

/// Async template stream - provides a Stream interface for template rendering
pub struct TemplateStream {
    /// Chunk receiver
    /// Background processing handle
impl TemplateStream {
    /// Create a new template stream
    pub async fn new(
    ) -> crate::error::Result<Self> {
        let (chunk_sender, chunk_receiver) = mpsc::channel::<StreamChunk>(renderer.config.max_concurrent_operations);
        let processing_handle = renderer.start_background_processing(ast, context, chunk_sender).await?;
        
        Ok(Self {
        })
    }
}

impl Stream for TemplateStream {
    type Item = crate::error::Result<StreamChunk>;
    
    fn poll_next(
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.chunk_receiver.poll_recv(cx) {
        }
    }
