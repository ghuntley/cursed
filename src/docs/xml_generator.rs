//! XML Documentation Generator
//! 
//! Generates structured XML documentation for integration with
//! documentation tools, IDEs, and automated processing systems.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, SearchIndexEntry};
use crate::error::Error;
use std::path::Path;
use std::fs;

pub struct XmlGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> XmlGenerator<'a> {
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive XML documentation
    pub fn generate_documentation(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let doc_path = output_dir.join("documentation.xml");
        
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<?xml-stylesheet type=\"text/xsl\" href=\"cursed-docs.xsl\"?>\n");
        xml.push_str("<cursed-documentation xmlns=\"https://cursed-lang.org/documentation/v1\" version=\"1.0\">\n");
        
        // Project metadata
        xml.push_str("  <project>\n");
        xml.push_str(&format!("    <name>{}</name>\n", self.escape_xml(&self.config.title)));
        if let Some(desc) = &self.config.description {
            xml.push_str(&format!("    <description>{}</description>\n", self.escape_xml(desc)));
        }
        if let Some(version) = &self.config.version {
            xml.push_str(&format!("    <version>{}</version>\n", self.escape_xml(version)));
        }
        xml.push_str("    <authors>\n");
        for author in &self.config.authors {
            xml.push_str(&format!("      <author>{}</author>\n", self.escape_xml(author)));
        }
        xml.push_str("    </authors>\n");
        xml.push_str(&format!("    <generated-at>{}</generated-at>\n", chrono::Utc::now().to_rfc3339()));
        xml.push_str("    <generator>CURSED Documentation Generator</generator>\n");
        xml.push_str("  </project>\n");

        // Configuration
        xml.push_str("  <configuration>\n");
        xml.push_str(&format!("    <include-private>{}</include-private>\n", self.config.include_private));
        xml.push_str(&format!("    <include-examples>{}</include-examples>\n", self.config.include_examples));
        xml.push_str(&format!("    <generate-cross-refs>{}</generate-cross-refs>\n", self.config.generate_cross_refs));
        xml.push_str("  </configuration>\n");

        // Statistics
        xml.push_str(&self.generate_statistics_xml(docs));

        // Gen Z Keywords mapping
        xml.push_str(&self.generate_keywords_xml());

        // Modules
        xml.push_str("  <modules>\n");
        for doc in docs {
            xml.push_str(&self.generate_module_xml(doc));
        }
        xml.push_str("  </modules>\n");

        xml.push_str("</cursed-documentation>\n");
        
        fs::write(doc_path, xml).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate search index XML
    pub fn generate_search_index(&self, index: &[SearchIndexEntry], output_dir: &Path) -> Result<(), Error> {
        let index_path = output_dir.join("search_index.xml");
        
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<search-index xmlns=\"https://cursed-lang.org/documentation/v1\">\n");
        xml.push_str("  <metadata>\n");
        xml.push_str(&format!("    <generated-at>{}</generated-at>\n", chrono::Utc::now().to_rfc3339()));
        xml.push_str(&format!("    <total-entries>{}</total-entries>\n", index.len()));
        xml.push_str("    <version>1.0.0</version>\n");
        xml.push_str("  </metadata>\n");
        
        xml.push_str("  <entries>\n");
        for entry in index {
            xml.push_str("    <entry>\n");
            xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&entry.name)));
            xml.push_str(&format!("      <kind>{}</kind>\n", entry.kind));
            xml.push_str(&format!("      <description>{}</description>\n", self.escape_xml(&entry.description)));
            xml.push_str(&format!("      <module>{}</module>\n", self.escape_xml(&entry.module)));
            xml.push_str(&format!("      <url>{}</url>\n", self.escape_xml(&entry.url)));
            xml.push_str("      <keywords>\n");
            for keyword in &entry.keywords {
                xml.push_str(&format!("        <keyword>{}</keyword>\n", self.escape_xml(keyword)));
            }
            xml.push_str("      </keywords>\n");
            xml.push_str("    </entry>\n");
        }
        xml.push_str("  </entries>\n");
        
        xml.push_str("</search-index>\n");
        
        fs::write(index_path, xml).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate statistics XML
    fn generate_statistics_xml(&self, docs: &[ExtractedDocumentation]) -> String {
        let mut xml = String::new();
        
        let total_items = docs.iter().map(|d| d.items.len()).sum::<usize>();
        let total_functions = docs.iter().map(|d| {
            d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count()
        }).sum::<usize>();
        let total_structs = docs.iter().map(|d| {
            d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count()
        }).sum::<usize>();
        let total_interfaces = docs.iter().map(|d| {
            d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Interface)).count()
        }).sum::<usize>();
        let total_lines = docs.iter().map(|d| d.source_info.line_count).sum::<usize>();
        
        xml.push_str("  <statistics>\n");
        xml.push_str(&format!("    <modules>{}</modules>\n", docs.len()));
        xml.push_str(&format!("    <total-items>{}</total-items>\n", total_items));
        xml.push_str(&format!("    <functions>{}</functions>\n", total_functions));
        xml.push_str(&format!("    <structs>{}</structs>\n", total_structs));
        xml.push_str(&format!("    <interfaces>{}</interfaces>\n", total_interfaces));
        xml.push_str(&format!("    <lines-of-code>{}</lines-of-code>\n", total_lines));
        
        if !docs.is_empty() {
            xml.push_str(&format!("    <average-items-per-module>{}</average-items-per-module>\n", total_items / docs.len()));
        }
        
        if let Some(largest) = docs.iter().max_by_key(|d| d.items.len()) {
            xml.push_str("    <largest-module>\n");
            xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&largest.module_name)));
            xml.push_str(&format!("      <items>{}</items>\n", largest.items.len()));
            xml.push_str("    </largest-module>\n");
        }
        
        xml.push_str("  </statistics>\n");
        xml
    }

    /// Generate Gen Z keywords mapping XML
    fn generate_keywords_xml(&self) -> String {
        let mut xml = String::new();
        
        xml.push_str("  <gen-z-keywords>\n");
        xml.push_str("    <description>CURSED uses Gen Z slang for keywords because traditional programming is cheugy</description>\n");
        xml.push_str("    <mappings>\n");
        
        let keywords = [
            ("slay", "fn, function", "Declares a function that absolutely slays"),
            ("sus", "let mut, var", "Declares a mutable variable (kinda sus if you ask me)"),
            ("facts", "let, const", "Declares a constant/immutable value (straight facts)"),
            ("lowkey", "if", "Conditional statement (lowkey checking this condition)"),
            ("highkey", "else", "Else clause (highkey the alternative)"),
            ("periodt", "while", "While loop (keeps going, periodt)"),
            ("bestie", "for", "For loop (going through this together, bestie)"),
            ("flex", "break", "Break statement (flexing out of this loop)"),
            ("squad", "struct, class", "Struct definition (organizing the squad)"),
            ("collab", "interface, trait", "Interface definition (collaborative vibes)"),
            ("stan", "async, spawn", "Spawn async operation/goroutine (we stan this concurrency)"),
            ("yolo", "yield, await", "Yield/await operation (yolo, just sending it)"),
        ];
        
        for (cursed, traditional, description) in keywords {
            xml.push_str("      <mapping>\n");
            xml.push_str(&format!("        <cursed>{}</cursed>\n", cursed));
            xml.push_str(&format!("        <traditional>{}</traditional>\n", self.escape_xml(traditional)));
            xml.push_str(&format!("        <description>{}</description>\n", self.escape_xml(description)));
            xml.push_str("      </mapping>\n");
        }
        
        xml.push_str("    </mappings>\n");
        xml.push_str("  </gen-z-keywords>\n");
        xml
    }

    /// Generate module XML
    fn generate_module_xml(&self, doc: &ExtractedDocumentation) -> String {
        let mut xml = String::new();
        
        xml.push_str("    <module>\n");
        xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&doc.module_name)));
        if let Some(package) = &doc.package_name {
            xml.push_str(&format!("      <package>{}</package>\n", self.escape_xml(package)));
        }
        xml.push_str(&format!("      <file-path>{}</file-path>\n", self.escape_xml(&doc.file_path.to_string_lossy())));
        
        // Source info
        xml.push_str("      <source-info>\n");
        xml.push_str(&format!("        <file-size>{}</file-size>\n", doc.source_info.file_size));
        xml.push_str(&format!("        <line-count>{}</line-count>\n", doc.source_info.line_count));
        xml.push_str(&format!("        <encoding>{}</encoding>\n", self.escape_xml(&doc.source_info.encoding)));
        if let Some(last_modified) = doc.source_info.last_modified {
            let timestamp = chrono::DateTime::<chrono::Utc>::from(last_modified).to_rfc3339();
            xml.push_str(&format!("        <last-modified>{}</last-modified>\n", timestamp));
        }
        xml.push_str("      </source-info>\n");
        
        // Imports
        if !doc.imports.is_empty() {
            xml.push_str("      <imports>\n");
            for import in &doc.imports {
                xml.push_str(&format!("        <import>{}</import>\n", self.escape_xml(import)));
            }
            xml.push_str("      </imports>\n");
        }
        
        // Items
        xml.push_str("      <items>\n");
        for item in &doc.items {
            xml.push_str(&self.generate_item_xml(item));
        }
        xml.push_str("      </items>\n");
        
        xml.push_str("    </module>\n");
        xml
    }

    /// Generate item XML
    fn generate_item_xml(&self, item: &crate::docs::generator::DocumentationItem) -> String {
        let mut xml = String::new();
        
        xml.push_str("        <item>\n");
        xml.push_str(&format!("          <name>{}</name>\n", self.escape_xml(&item.name)));
        xml.push_str(&format!("          <kind>{}</kind>\n", item.kind));
        xml.push_str(&format!("          <visibility>{:?}</visibility>\n", item.visibility));
        xml.push_str(&format!("          <module>{}</module>\n", self.escape_xml(&item.module)));
        xml.push_str(&format!("          <summary>{}</summary>\n", self.escape_xml(&item.summary)));
        xml.push_str(&format!("          <description><![CDATA[{}]]></description>\n", item.description));
        
        if let Some(signature) = &item.signature {
            xml.push_str(&format!("          <signature><![CDATA[{}]]></signature>\n", signature));
        }
        
        // Parameters
        if !item.parameters.is_empty() {
            xml.push_str("          <parameters>\n");
            for param in &item.parameters {
                xml.push_str("            <parameter>\n");
                xml.push_str(&format!("              <name>{}</name>\n", self.escape_xml(&param.name)));
                if let Some(type_name) = &param.type_name {
                    xml.push_str(&format!("              <type>{}</type>\n", self.escape_xml(type_name)));
                }
                xml.push_str(&format!("              <description>{}</description>\n", self.escape_xml(&param.description)));
                if let Some(default) = &param.default_value {
                    xml.push_str(&format!("              <default-value><![CDATA[{}]]></default-value>\n", default));
                }
                xml.push_str("            </parameter>\n");
            }
            xml.push_str("          </parameters>\n");
        }
        
        // Return type
        if let Some(return_type) = &item.return_type {
            xml.push_str(&format!("          <return-type>{}</return-type>\n", self.escape_xml(return_type)));
        }
        
        // Examples
        if !item.examples.is_empty() {
            xml.push_str("          <examples>\n");
            for example in &item.examples {
                xml.push_str("            <example>\n");
                if let Some(title) = &example.title {
                    xml.push_str(&format!("              <title>{}</title>\n", self.escape_xml(title)));
                }
                if let Some(description) = &example.description {
                    xml.push_str(&format!("              <description>{}</description>\n", self.escape_xml(description)));
                }
                xml.push_str(&format!("              <code><![CDATA[{}]]></code>\n", example.code));
                xml.push_str(&format!("              <language>{}</language>\n", self.escape_xml(&example.language)));
                if let Some(output) = &example.output {
                    xml.push_str(&format!("              <output><![CDATA[{}]]></output>\n", output));
                }
                xml.push_str("            </example>\n");
            }
            xml.push_str("          </examples>\n");
        }
        
        // Tags
        if !item.tags.is_empty() {
            xml.push_str("          <tags>\n");
            for (tag_name, tag_values) in &item.tags {
                xml.push_str(&format!("            <tag name=\"{}\">\n", self.escape_xml(tag_name)));
                for value in tag_values {
                    xml.push_str(&format!("              <value>{}</value>\n", self.escape_xml(value)));
                }
                xml.push_str("            </tag>\n");
            }
            xml.push_str("          </tags>\n");
        }
        
        // Location
        xml.push_str("          <location>\n");
        xml.push_str(&format!("            <line>{}</line>\n", item.location.line));
        xml.push_str(&format!("            <column>{}</column>\n", item.location.column));
        if let Some(file) = &item.location.file {
            xml.push_str(&format!("            <file>{}</file>\n", self.escape_xml(file)));
        }
        xml.push_str("          </location>\n");
        
        // Source code
        if self.config.include_examples {
            if let Some(source) = &item.source_code {
                xml.push_str(&format!("          <source-code><![CDATA[{}]]></source-code>\n", source));
            }
        }
        
        // CURSED-specific features
        xml.push_str(&self.generate_cursed_features_xml(item));
        
        xml.push_str("        </item>\n");
        xml
    }

    /// Generate CURSED-specific features XML
    fn generate_cursed_features_xml(&self, item: &crate::docs::generator::DocumentationItem) -> String {
        let mut xml = String::new();
        let mut features = Vec::new();
        
        if let Some(signature) = &item.signature {
            if signature.contains("slay") { features.push("uses-slay-keyword"); }
            if signature.contains("sus") { features.push("uses-sus-keyword"); }
            if signature.contains("facts") { features.push("uses-facts-keyword"); }
            if signature.contains("squad") { features.push("uses-squad-keyword"); }
            if signature.contains("collab") { features.push("uses-collab-keyword"); }
        }
        
        let name_lower = item.name.to_lowercase();
        if name_lower.contains("vibes") || name_lower.contains("energy") || name_lower.contains("mood") {
            features.push("gen-z-naming");
        }
        
        if item.description.chars().any(|c| c as u32 > 127) {
            features.push("uses-emojis");
        }
        
        if !features.is_empty() {
            xml.push_str("          <cursed-features>\n");
            for feature in &features {
                xml.push_str(&format!("            <feature>{}</feature>\n", feature));
            }
            xml.push_str(&format!("            <slang-level>{}</slang-level>\n", self.calculate_slang_level(item)));
            xml.push_str(&format!("            <gen-z-score>{}</gen-z-score>\n", features.len()));
            xml.push_str("          </cursed-features>\n");
        }
        
        xml
    }

    /// Calculate slang level
    fn calculate_slang_level(&self, item: &crate::docs::generator::DocumentationItem) -> String {
        let slang_keywords = ["slay", "sus", "facts", "lowkey", "highkey", "periodt", "bestie", "flex", "squad", "collab", "stan", "yolo"];
        let mut slang_count = 0;
        
        if let Some(signature) = &item.signature {
            for keyword in &slang_keywords {
                if signature.contains(keyword) {
                    slang_count += 1;
                }
            }
        }
        
        for keyword in &slang_keywords {
            if item.description.to_lowercase().contains(keyword) {
                slang_count += 1;
            }
        }
        
        match slang_count {
            0 => "basic",
            1..=2 => "lowkey",
            3..=4 => "highkey", 
            _ => "absolutely-iconic",
        }.to_string()
    }

    /// Escape XML special characters
    fn escape_xml(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}
