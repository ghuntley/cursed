//! XML Documentation Generator
//! 
//! Generates structured XML documentation following DocBook and API documentation standards.
//! The XML output can be used with XML processing tools, XSLT transformations, and 
//! integrated into existing documentation toolchains.

use super::{DocGeneratorConfig, ExtractedDocumentation, DocumentationItem, SearchIndexEntry, Parameter, Example};
use crate::error::Error;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

/// XML documentation generator
pub struct XmlGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> XmlGenerator<'a> {
    /// Create a new XML generator
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive XML documentation
    pub fn generate_documentation(
        &self,
        docs: &[ExtractedDocumentation],
        output_dir: &Path,
    ) -> Result<(), Error> {
        // Generate main documentation file
        let main_xml = self.build_main_documentation(docs)?;
        let main_file = output_dir.join("documentation.xml");
        fs::write(&main_file, main_xml).map_err(Error::Io)?;

        // Generate individual module files
        for doc in docs {
            let module_xml = self.build_module_documentation(doc)?;
            let module_filename = format!("{}.xml", doc.module_name.replace("::", "_"));
            let module_file = output_dir.join(module_filename);
            fs::write(&module_file, module_xml).map_err(Error::Io)?;
        }

        // Generate API index for quick access
        let api_index_xml = self.build_api_index(docs)?;
        let api_file = output_dir.join("api_index.xml");
        fs::write(&api_file, api_index_xml).map_err(Error::Io)?;

        // Generate DTD file for validation
        let dtd_content = self.generate_dtd()?;
        let dtd_file = output_dir.join("cursed_docs.dtd");
        fs::write(&dtd_file, dtd_content).map_err(Error::Io)?;

        Ok(())
    }

    /// Generate search index in XML format
    pub fn generate_search_index(
        &self,
        search_index: &[SearchIndexEntry],
        output_dir: &Path,
    ) -> Result<(), Error> {
        let search_xml = self.build_search_index_xml(search_index)?;
        let search_file = output_dir.join("search_index.xml");
        fs::write(&search_file, search_xml).map_err(Error::Io)?;

        Ok(())
    }

    /// Build main documentation XML structure
    fn build_main_documentation(&self, docs: &[ExtractedDocumentation]) -> Result<String, Error> {
        let mut xml = String::new();
        
        // XML declaration and DTD reference
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE documentation SYSTEM \"cursed_docs.dtd\">\n\n");
        
        // Root element with metadata
        xml.push_str("<documentation>\n");
        xml.push_str("  <metadata>\n");
        xml.push_str(&format!("    <title>{}</title>\n", self.escape_xml(&self.config.title)));
        
        if let Some(description) = &self.config.description {
            xml.push_str(&format!("    <description>{}</description>\n", self.escape_xml(description)));
        }
        
        if let Some(version) = &self.config.version {
            xml.push_str(&format!("    <version>{}</version>\n", self.escape_xml(version)));
        }
        
        if !self.config.authors.is_empty() {
            xml.push_str("    <authors>\n");
            for author in &self.config.authors {
                xml.push_str(&format!("      <author>{}</author>\n", self.escape_xml(author)));
            }
            xml.push_str("    </authors>\n");
        }
        
        if let Some(base_url) = &self.config.base_url {
            xml.push_str(&format!("    <base_url>{}</base_url>\n", self.escape_xml(base_url)));
        }
        
        xml.push_str(&format!("    <generated_at>{}</generated_at>\n", 
            chrono::Utc::now().to_rfc3339()));
        xml.push_str("    <generator>CURSED Documentation Generator</generator>\n");
        xml.push_str("  </metadata>\n\n");

        // Project summary
        xml.push_str("  <summary>\n");
        xml.push_str(&format!("    <total_modules>{}</total_modules>\n", docs.len()));
        let total_items: usize = docs.iter().map(|d| d.items.len()).sum();
        xml.push_str(&format!("    <total_items>{}</total_items>\n", total_items));
        
        // Item statistics by type
        let mut item_counts = HashMap::new();
        for doc in docs {
            for item in &doc.items {
                *item_counts.entry(item.kind.to_string()).or_insert(0) += 1;
            }
        }
        
        xml.push_str("    <item_statistics>\n");
        for (kind, count) in item_counts {
            xml.push_str(&format!("      <item_type name=\"{}\" count=\"{}\"/>\n", kind, count));
        }
        xml.push_str("    </item_statistics>\n");
        xml.push_str("  </summary>\n\n");

        // Modules overview
        xml.push_str("  <modules>\n");
        for doc in docs {
            xml.push_str("    <module>\n");
            xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&doc.module_name)));
            xml.push_str(&format!("      <file_path>{}</file_path>\n", 
                self.escape_xml(&doc.file_path.display().to_string())));
            xml.push_str(&format!("      <item_count>{}</item_count>\n", doc.items.len()));
            
            // Module imports
            if !doc.imports.is_empty() {
                xml.push_str("      <imports>\n");
                for import in &doc.imports {
                    xml.push_str(&format!("        <import>{}</import>\n", self.escape_xml(import)));
                }
                xml.push_str("      </imports>\n");
            }
            
            // Module source info
            xml.push_str("      <source_info>\n");
            xml.push_str(&format!("        <file_size>{}</file_size>\n", doc.source_info.file_size));
            xml.push_str(&format!("        <line_count>{}</line_count>\n", doc.source_info.line_count));
            xml.push_str(&format!("        <encoding>{}</encoding>\n", 
                self.escape_xml(&doc.source_info.encoding)));
            if let Some(modified) = doc.source_info.last_modified {
                if let Ok(datetime) = modified.duration_since(std::time::UNIX_EPOCH) {
                    xml.push_str(&format!("        <last_modified>{}</last_modified>\n", 
                        datetime.as_secs()));
                }
            }
            xml.push_str("      </source_info>\n");
            
            xml.push_str(&format!("      <reference>file://{}.xml</reference>\n", 
                doc.module_name.replace("::", "_")));
            xml.push_str("    </module>\n");
        }
        xml.push_str("  </modules>\n");
        
        xml.push_str("</documentation>\n");
        
        Ok(xml)
    }

    /// Build module-specific XML documentation
    fn build_module_documentation(&self, doc: &ExtractedDocumentation) -> Result<String, Error> {
        let mut xml = String::new();
        
        // XML declaration and DTD reference
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE module SYSTEM \"cursed_docs.dtd\">\n\n");
        
        // Module root element
        xml.push_str("<module>\n");
        xml.push_str("  <header>\n");
        xml.push_str(&format!("    <name>{}</name>\n", self.escape_xml(&doc.module_name)));
        xml.push_str(&format!("    <file_path>{}</file_path>\n", 
            self.escape_xml(&doc.file_path.display().to_string())));
        
        if let Some(package_name) = &doc.package_name {
            xml.push_str(&format!("    <package>{}</package>\n", self.escape_xml(package_name)));
        }
        
        xml.push_str(&format!("    <generated_at>{}</generated_at>\n", 
            chrono::Utc::now().to_rfc3339()));
        xml.push_str("  </header>\n\n");

        // Module imports
        if !doc.imports.is_empty() {
            xml.push_str("  <imports>\n");
            for import in &doc.imports {
                xml.push_str(&format!("    <import>{}</import>\n", self.escape_xml(import)));
            }
            xml.push_str("  </imports>\n\n");
        }

        // Group items by type
        let mut grouped_items = HashMap::new();
        for item in &doc.items {
            let key = item.kind.to_string();
            grouped_items.entry(key).or_insert_with(Vec::new).push(item);
        }

        // Document each item type
        xml.push_str("  <items>\n");
        
        for (item_type, items) in grouped_items {
            xml.push_str(&format!("    <item_group type=\"{}\">\n", item_type));
            
            for item in items {
                xml.push_str("      <item>\n");
                xml.push_str(&format!("        <name>{}</name>\n", self.escape_xml(&item.name)));
                xml.push_str(&format!("        <kind>{}</kind>\n", item.kind.to_string()));
                xml.push_str(&format!("        <visibility>{:?}</visibility>\n", item.visibility));
                xml.push_str(&format!("        <module_path>{}</module_path>\n", 
                    self.escape_xml(&item.module)));
                
                xml.push_str(&format!("        <summary>{}</summary>\n", 
                    self.escape_xml(&item.summary)));
                
                if !item.description.is_empty() {
                    xml.push_str(&format!("        <description>{}</description>\n", 
                        self.escape_xml(&item.description)));
                }
                
                if let Some(signature) = &item.signature {
                    xml.push_str(&format!("        <signature>{}</signature>\n", 
                        self.escape_xml(signature)));
                }
                
                if let Some(return_type) = &item.return_type {
                    xml.push_str(&format!("        <return_type>{}</return_type>\n", 
                        self.escape_xml(return_type)));
                }
                
                // Parameters
                if !item.parameters.is_empty() {
                    xml.push_str("        <parameters>\n");
                    for param in &item.parameters {
                        xml.push_str(&self.format_parameter_xml(param, 10));
                    }
                    xml.push_str("        </parameters>\n");
                }
                
                // Examples
                if !item.examples.is_empty() {
                    xml.push_str("        <examples>\n");
                    for example in &item.examples {
                        xml.push_str(&self.format_example_xml(example, 10));
                    }
                    xml.push_str("        </examples>\n");
                }
                
                // Tags
                if !item.tags.is_empty() {
                    xml.push_str("        <tags>\n");
                    for (tag_name, values) in &item.tags {
                        for value in values {
                            xml.push_str(&format!("          <tag name=\"{}\">{}</tag>\n", 
                                self.escape_xml(tag_name), self.escape_xml(value)));
                        }
                    }
                    xml.push_str("        </tags>\n");
                }
                
                // Location information
                xml.push_str("        <location>\n");
                xml.push_str(&format!("          <line>{}</line>\n", item.location.line));
                xml.push_str(&format!("          <column>{}</column>\n", item.location.column));
                if let Some(file) = &item.location.file {
                    xml.push_str(&format!("          <file>{}</file>\n", self.escape_xml(file)));
                }
                xml.push_str("        </location>\n");
                
                // Source code if available and configured
                if self.config.include_examples && item.source_code.is_some() {
                    xml.push_str("        <source_code>\n");
                    xml.push_str(&format!("          <![CDATA[{}]]>\n", 
                        item.source_code.as_ref().unwrap()));
                    xml.push_str("        </source_code>\n");
                }
                
                xml.push_str("      </item>\n");
            }
            
            xml.push_str("    </item_group>\n");
        }
        
        xml.push_str("  </items>\n");
        xml.push_str("</module>\n");
        
        Ok(xml)
    }

    /// Build API index XML for quick navigation
    fn build_api_index(&self, docs: &[ExtractedDocumentation]) -> Result<String, Error> {
        let mut xml = String::new();
        
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE api_index SYSTEM \"cursed_docs.dtd\">\n\n");
        
        xml.push_str("<api_index>\n");
        xml.push_str("  <metadata>\n");
        xml.push_str(&format!("    <generated_at>{}</generated_at>\n", 
            chrono::Utc::now().to_rfc3339()));
        xml.push_str("    <description>Quick reference index for all API items</description>\n");
        xml.push_str("  </metadata>\n\n");

        // Build index by item type
        let mut all_items = Vec::new();
        for doc in docs {
            for item in &doc.items {
                all_items.push((doc, item));
            }
        }

        // Group by item kind
        let mut by_kind = HashMap::new();
        for (doc, item) in &all_items {
            by_kind.entry(item.kind.to_string()).or_insert_with(Vec::new).push((doc, *item));
        }

        xml.push_str("  <index_by_type>\n");
        for (kind, items) in by_kind {
            xml.push_str(&format!("    <type_group name=\"{}\">\n", kind));
            
            for (doc, item) in items {
                xml.push_str("      <item_ref>\n");
                xml.push_str(&format!("        <name>{}</name>\n", self.escape_xml(&item.name)));
                xml.push_str(&format!("        <module>{}</module>\n", 
                    self.escape_xml(&doc.module_name)));
                xml.push_str(&format!("        <summary>{}</summary>\n", 
                    self.escape_xml(&item.summary)));
                xml.push_str(&format!("        <file_ref>{}.xml#{}</file_ref>\n", 
                    doc.module_name.replace("::", "_"), item.name));
                xml.push_str("      </item_ref>\n");
            }
            
            xml.push_str("    </type_group>\n");
        }
        xml.push_str("  </index_by_type>\n");

        // Alphabetical index
        all_items.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        
        xml.push_str("  <alphabetical_index>\n");
        for (doc, item) in all_items {
            xml.push_str("    <item_ref>\n");
            xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&item.name)));
            xml.push_str(&format!("      <kind>{}</kind>\n", item.kind.to_string()));
            xml.push_str(&format!("      <module>{}</module>\n", 
                self.escape_xml(&doc.module_name)));
            xml.push_str(&format!("      <file_ref>{}.xml#{}</file_ref>\n", 
                doc.module_name.replace("::", "_"), item.name));
            xml.push_str("    </item_ref>\n");
        }
        xml.push_str("  </alphabetical_index>\n");
        
        xml.push_str("</api_index>\n");
        
        Ok(xml)
    }

    /// Build search index in XML format
    fn build_search_index_xml(&self, search_index: &[SearchIndexEntry]) -> Result<String, Error> {
        let mut xml = String::new();
        
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE search_index SYSTEM \"cursed_docs.dtd\">\n\n");
        
        xml.push_str("<search_index>\n");
        xml.push_str("  <metadata>\n");
        xml.push_str(&format!("    <entry_count>{}</entry_count>\n", search_index.len()));
        xml.push_str(&format!("    <generated_at>{}</generated_at>\n", 
            chrono::Utc::now().to_rfc3339()));
        xml.push_str("  </metadata>\n\n");
        
        xml.push_str("  <entries>\n");
        for entry in search_index {
            xml.push_str("    <entry>\n");
            xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&entry.name)));
            xml.push_str(&format!("      <kind>{}</kind>\n", entry.kind.to_string()));
            xml.push_str(&format!("      <description>{}</description>\n", 
                self.escape_xml(&entry.description)));
            xml.push_str(&format!("      <module>{}</module>\n", 
                self.escape_xml(&entry.module)));
            xml.push_str(&format!("      <url>{}</url>\n", self.escape_xml(&entry.url)));
            
            if !entry.keywords.is_empty() {
                xml.push_str("      <keywords>\n");
                for keyword in &entry.keywords {
                    xml.push_str(&format!("        <keyword>{}</keyword>\n", 
                        self.escape_xml(keyword)));
                }
                xml.push_str("      </keywords>\n");
            }
            
            xml.push_str("    </entry>\n");
        }
        xml.push_str("  </entries>\n");
        xml.push_str("</search_index>\n");
        
        Ok(xml)
    }

    /// Format parameter as XML
    fn format_parameter_xml(&self, param: &Parameter, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        let mut xml = String::new();
        
        xml.push_str(&format!("{}  <parameter>\n", indent_str));
        xml.push_str(&format!("{}    <name>{}</name>\n", indent_str, 
            self.escape_xml(&param.name)));
        
        if let Some(type_name) = &param.type_name {
            xml.push_str(&format!("{}    <type>{}</type>\n", indent_str, 
                self.escape_xml(type_name)));
        }
        
        xml.push_str(&format!("{}    <description>{}</description>\n", indent_str, 
            self.escape_xml(&param.description)));
        
        if let Some(default_value) = &param.default_value {
            xml.push_str(&format!("{}    <default_value>{}</default_value>\n", indent_str, 
                self.escape_xml(default_value)));
        }
        
        xml.push_str(&format!("{}  </parameter>\n", indent_str));
        
        xml
    }

    /// Format example as XML
    fn format_example_xml(&self, example: &Example, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        let mut xml = String::new();
        
        xml.push_str(&format!("{}  <example>\n", indent_str));
        
        if let Some(title) = &example.title {
            xml.push_str(&format!("{}    <title>{}</title>\n", indent_str, 
                self.escape_xml(title)));
        }
        
        if let Some(description) = &example.description {
            xml.push_str(&format!("{}    <description>{}</description>\n", indent_str, 
                self.escape_xml(description)));
        }
        
        xml.push_str(&format!("{}    <language>{}</language>\n", indent_str, 
            self.escape_xml(&example.language)));
        
        xml.push_str(&format!("{}    <code>\n", indent_str));
        xml.push_str(&format!("{}      <![CDATA[{}]]\n", indent_str, example.code));
        xml.push_str(&format!("{}    </code>\n", indent_str));
        
        if let Some(output) = &example.output {
            xml.push_str(&format!("{}    <output>\n", indent_str));
            xml.push_str(&format!("{}      <![CDATA[{}]]\n", indent_str, output));
            xml.push_str(&format!("{}    </output>\n", indent_str));
        }
        
        xml.push_str(&format!("{}  </example>\n", indent_str));
        
        xml
    }

    /// Generate DTD file for XML validation
    fn generate_dtd(&self) -> Result<String, Error> {
        let dtd = r#"<!-- CURSED Documentation DTD -->
<!-- This DTD defines the structure for CURSED documentation XML files -->

<!ELEMENT documentation (metadata, summary, modules)>

<!ELEMENT metadata (title, description?, version?, authors?, base_url?, generated_at, generator)>
<!ELEMENT title (#PCDATA)>
<!ELEMENT description (#PCDATA)>
<!ELEMENT version (#PCDATA)>
<!ELEMENT authors (author+)>
<!ELEMENT author (#PCDATA)>
<!ELEMENT base_url (#PCDATA)>
<!ELEMENT generated_at (#PCDATA)>
<!ELEMENT generator (#PCDATA)>

<!ELEMENT summary (total_modules, total_items, item_statistics)>
<!ELEMENT total_modules (#PCDATA)>
<!ELEMENT total_items (#PCDATA)>
<!ELEMENT item_statistics (item_type*)>
<!ELEMENT item_type EMPTY>
<!ATTLIST item_type name CDATA #REQUIRED count CDATA #REQUIRED>

<!ELEMENT modules (module*)>
<!ELEMENT module (header, imports?, items?)>

<!ELEMENT header (name, file_path, package?, generated_at)>
<!ELEMENT name (#PCDATA)>
<!ELEMENT file_path (#PCDATA)>
<!ELEMENT package (#PCDATA)>

<!ELEMENT imports (import*)>
<!ELEMENT import (#PCDATA)>

<!ELEMENT items (item_group*)>
<!ELEMENT item_group (item*)>
<!ATTLIST item_group type CDATA #REQUIRED>

<!ELEMENT item (name, kind, visibility, module_path, summary, description?, signature?, return_type?, parameters?, examples?, tags?, location, source_code?)>
<!ELEMENT kind (#PCDATA)>
<!ELEMENT visibility (#PCDATA)>
<!ELEMENT module_path (#PCDATA)>
<!ELEMENT summary (#PCDATA)>
<!ELEMENT signature (#PCDATA)>
<!ELEMENT return_type (#PCDATA)>

<!ELEMENT parameters (parameter*)>
<!ELEMENT parameter (name, type?, description, default_value?)>
<!ELEMENT type (#PCDATA)>
<!ELEMENT default_value (#PCDATA)>

<!ELEMENT examples (example*)>
<!ELEMENT example (title?, description?, language, code, output?)>
<!ELEMENT language (#PCDATA)>
<!ELEMENT code (#PCDATA)>
<!ELEMENT output (#PCDATA)>

<!ELEMENT tags (tag*)>
<!ELEMENT tag (#PCDATA)>
<!ATTLIST tag name CDATA #REQUIRED>

<!ELEMENT location (line, column, file?)>
<!ELEMENT line (#PCDATA)>
<!ELEMENT column (#PCDATA)>
<!ELEMENT file (#PCDATA)>

<!ELEMENT source_code (#PCDATA)>

<!-- API Index DTD -->
<!ELEMENT api_index (metadata, index_by_type, alphabetical_index)>
<!ELEMENT index_by_type (type_group*)>
<!ELEMENT type_group (item_ref*)>
<!ATTLIST type_group name CDATA #REQUIRED>

<!ELEMENT alphabetical_index (item_ref*)>
<!ELEMENT item_ref (name, kind?, module, summary?, file_ref)>
<!ELEMENT file_ref (#PCDATA)>

<!-- Search Index DTD -->
<!ELEMENT search_index (metadata, entries)>
<!ELEMENT entry_count (#PCDATA)>
<!ELEMENT entries (entry*)>
<!ELEMENT entry (name, kind, description, module, url, keywords?)>
<!ELEMENT url (#PCDATA)>
<!ELEMENT keywords (keyword*)>
<!ELEMENT keyword (#PCDATA)>
"#;
        
        Ok(dtd.to_string())
    }

    /// Escape XML special characters
    fn escape_xml(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

/// Reference implementation for external use
pub use XmlGenerator;
