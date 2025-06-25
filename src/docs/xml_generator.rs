// XML Documentation Generator
// 
// Generates comprehensive XML documentation compatible with various tools and IDEs.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, DocumentationItem, SearchIndexEntry};
use crate::error::CursedError;
use std::fs;
use std::path::Path;

/// XML documentation generator
pub struct XmlGenerator {
    config: DocGeneratorConfig,
}

impl XmlGenerator {
    pub fn new(config: &DocGeneratorConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Generate comprehensive XML documentation
    pub fn generate_documentation(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> crate::error::Result<()> {
        let xml_path = output_dir.join("documentation.xml");
        
        let mut content = String::new();
        content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        content.push_str("<documentation>\n");
        
        // Metadata section
        content.push_str("  <metadata>\n");
        content.push_str(&format!("    <title>{}</title>\n", self.xml_escape(&self.config.title)));
        if let Some(description) = &self.config.description {
            content.push_str(&format!("    <description>{}</description>\n", self.xml_escape(description)));
        }
        if let Some(version) = &self.config.version {
            content.push_str(&format!("    <version>{}</version>\n", self.xml_escape(version)));
        }
        if !self.config.authors.is_empty() {
            content.push_str("    <authors>\n");
            for author in &self.config.authors {
                content.push_str(&format!("      <author>{}</author>\n", self.xml_escape(author)));
            }
            content.push_str("    </authors>\n");
        }
        content.push_str(&format!("    <generated_at>{}</generated_at>\n", chrono::Utc::now().to_rfc3339()));
        content.push_str("    <generator>CURSED Documentation System</generator>\n");
        content.push_str("    <format_version>1.0.0</format_version>\n");
        content.push_str("  </metadata>\n");
        
        // Configuration section
        content.push_str("  <configuration>\n");
        content.push_str(&format!("    <include_examples>{}</include_examples>\n", self.config.include_examples));
        content.push_str(&format!("    <include_private>{}</include_private>\n", self.config.include_private));
        content.push_str(&format!("    <generate_cross_refs>{}</generate_cross_refs>\n", self.config.generate_cross_refs));
        if let Some(base_url) = &self.config.base_url {
            content.push_str(&format!("    <base_url>{}</base_url>\n", self.xml_escape(base_url)));
        }
        content.push_str("  </configuration>\n");
        
        // Statistics section
        let total_items: usize = docs.iter().map(|d| d.items.len()).sum();
        let total_functions = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count()).sum::<usize>();
        let total_structs = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count()).sum::<usize>();
        
        content.push_str("  <statistics>\n");
        content.push_str(&format!("    <total_modules>{}</total_modules>\n", docs.len()));
        content.push_str(&format!("    <total_items>{}</total_items>\n", total_items));
        content.push_str(&format!("    <total_functions>{}</total_functions>\n", total_functions));
        content.push_str(&format!("    <total_structs>{}</total_structs>\n", total_structs));
        content.push_str("  </statistics>\n");
        
        // Modules section
        content.push_str("  <modules>\n");
        for doc in docs {
            content.push_str(&self.generate_module_xml(doc)?);
        }
        content.push_str("  </modules>\n");
        
        content.push_str("</documentation>\n");
        
        fs::write(xml_path, content).map_err(CursedError::Io)?;
        Ok(())
    }

    /// Generate XML for a single module
    fn generate_module_xml(&self, doc: &ExtractedDocumentation) -> crate::error::Result<()> {
        let mut content = String::new();
        
        content.push_str(&format!("    <module name=\"{}\">\n", self.xml_escape(&doc.module_name)));
        
        // Module information
        if let Some(package) = &doc.package_name {
            content.push_str(&format!("      <package>{}</package>\n", self.xml_escape(package)));
        }
        content.push_str(&format!("      <file_path>{}</file_path>\n", self.xml_escape(&doc.file_path.display().to_string())));
        
        // Source information
        content.push_str("      <source_info>\n");
        content.push_str(&format!("        <file_size>{}</file_size>\n", doc.source_info.file_size));
        content.push_str(&format!("        <line_count>{}</line_count>\n", doc.source_info.line_count));
        content.push_str(&format!("        <encoding>{}</encoding>\n", self.xml_escape(&doc.source_info.encoding)));
        if let Some(last_modified) = doc.source_info.last_modified {
            if let Ok(datetime) = last_modified.duration_since(std::time::UNIX_EPOCH) {
                content.push_str(&format!("        <last_modified>{}</last_modified>\n", datetime.as_secs()));
            }
        }
        content.push_str("      </source_info>\n");
        
        // Imports
        if !doc.imports.is_empty() {
            content.push_str("      <imports>\n");
            for import in &doc.imports {
                content.push_str(&format!("        <import>{}</import>\n", self.xml_escape(import)));
            }
            content.push_str("      </imports>\n");
        }
        
        // Items
        content.push_str("      <items>\n");
        for item in &doc.items {
            content.push_str(&self.generate_item_xml(item)?);
        }
        content.push_str("      </items>\n");
        
        content.push_str("    </module>\n");
        Ok(content)
    }

    /// Generate XML for a single documentation item
    fn generate_item_xml(&self, item: &DocumentationItem) -> crate::error::Result<()> {
        let mut content = String::new();
        
        let kind_str = match item.kind {
            crate::docs::generator::ItemKind::Function => "function",
            crate::docs::generator::ItemKind::Struct => "struct",
            crate::docs::generator::ItemKind::Interface => "interface",
            crate::docs::generator::ItemKind::Variable => "variable",
            crate::docs::generator::ItemKind::Constant => "constant",
            crate::docs::generator::ItemKind::Type => "type",
            crate::docs::generator::ItemKind::Module => "module",
        };
        
        let visibility_str = match item.visibility {
            crate::docs::generator::Visibility::Public => "public",
            crate::docs::generator::Visibility::Private => "private",
        };
        
        content.push_str(&format!("        <item name=\"{}\" kind=\"{}\" visibility=\"{}\">\n", 
            self.xml_escape(&item.name), kind_str, visibility_str));
        
        content.push_str(&format!("          <module>{}</module>\n", self.xml_escape(&item.module)));
        content.push_str(&format!("          <summary>{}</summary>\n", self.xml_escape(&item.summary)));
        content.push_str(&format!("          <description>{}</description>\n", self.xml_escape(&item.description)));
        
        // Signature
        if let Some(signature) = &item.signature {
            content.push_str(&format!("          <signature>{}</signature>\n", self.xml_escape(signature)));
        }
        
        // Parameters
        if !item.parameters.is_empty() {
            content.push_str("          <parameters>\n");
            for param in &item.parameters {
                content.push_str("            <parameter>\n");
                content.push_str(&format!("              <name>{}</name>\n", self.xml_escape(&param.name)));
                if let Some(type_name) = &param.type_name {
                    content.push_str(&format!("              <type>{}</type>\n", self.xml_escape(type_name)));
                }
                content.push_str(&format!("              <description>{}</description>\n", self.xml_escape(&param.description)));
                if let Some(default_value) = &param.default_value {
                    content.push_str(&format!("              <default_value>{}</default_value>\n", self.xml_escape(default_value)));
                }
                content.push_str("            </parameter>\n");
            }
            content.push_str("          </parameters>\n");
        }
        
        // Return type
        if let Some(return_type) = &item.return_type {
            content.push_str(&format!("          <return_type>{}</return_type>\n", self.xml_escape(return_type)));
        }
        
        // Examples
        if !item.examples.is_empty() {
            content.push_str("          <examples>\n");
            for example in &item.examples {
                content.push_str("            <example>\n");
                if let Some(title) = &example.title {
                    content.push_str(&format!("              <title>{}</title>\n", self.xml_escape(title)));
                }
                if let Some(description) = &example.description {
                    content.push_str(&format!("              <description>{}</description>\n", self.xml_escape(description)));
                }
                content.push_str(&format!("              <language>{}</language>\n", self.xml_escape(&example.language)));
                content.push_str(&format!("              <code><![CDATA[{}]]></code>\n", example.code));
                if let Some(output) = &example.output {
                    content.push_str(&format!("              <output><![CDATA[{}]]></output>\n", output));
                }
                content.push_str("            </example>\n");
            }
            content.push_str("          </examples>\n");
        }
        
        // Tags
        if !item.tags.is_empty() {
            content.push_str("          <tags>\n");
            for (tag_name, tag_values) in &item.tags {
                for tag_value in tag_values {
                    content.push_str(&format!("            <tag name=\"{}\">{}</tag>\n", 
                        self.xml_escape(tag_name), self.xml_escape(tag_value)));
                }
            }
            content.push_str("          </tags>\n");
        }
        
        // Location
        content.push_str("          <location>\n");
        content.push_str(&format!("            <line>{}</line>\n", item.location.line));
        content.push_str(&format!("            <column>{}</column>\n", item.location.column));
        if let Some(file) = &item.location.file {
            content.push_str(&format!("            <file>{}</file>\n", self.xml_escape(file)));
        }
        content.push_str("          </location>\n");
        
        // Source code
        if self.config.include_examples && item.source_code.is_some() {
            content.push_str(&format!("          <source_code><![CDATA[{}]]></source_code>\n", 
                item.source_code.as_ref().unwrap()));
        }
        
        content.push_str("        </item>\n");
        Ok(content)
    }

    /// Generate search index XML
    pub fn generate_search_index(&self, search_index: &[SearchIndexEntry], output_dir: &Path) -> crate::error::Result<()> {
        let search_path = output_dir.join("search_index.xml");
        
        let mut content = String::new();
        content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        content.push_str("<search_index>\n");
        
        content.push_str("  <metadata>\n");
        content.push_str(&format!("    <generated_at>{}</generated_at>\n", chrono::Utc::now().to_rfc3339()));
        content.push_str(&format!("    <total_entries>{}</total_entries>\n", search_index.len()));
        content.push_str("    <version>1.0.0</version>\n");
        content.push_str("  </metadata>\n");
        
        content.push_str("  <entries>\n");
        for entry in search_index {
            content.push_str("    <entry>\n");
            content.push_str(&format!("      <name>{}</name>\n", self.xml_escape(&entry.name)));
            content.push_str(&format!("      <kind>{}</kind>\n", entry.kind.to_string()));
            content.push_str(&format!("      <description>{}</description>\n", self.xml_escape(&entry.description)));
            content.push_str(&format!("      <module>{}</module>\n", self.xml_escape(&entry.module)));
            content.push_str(&format!("      <url>{}</url>\n", self.xml_escape(&entry.url)));
            
            if !entry.keywords.is_empty() {
                content.push_str("      <keywords>\n");
                for keyword in &entry.keywords {
                    content.push_str(&format!("        <keyword>{}</keyword>\n", self.xml_escape(keyword)));
                }
                content.push_str("      </keywords>\n");
            }
            
            content.push_str("    </entry>\n");
        }
        content.push_str("  </entries>\n");
        
        content.push_str("</search_index>\n");
        
        fs::write(search_path, content).map_err(CursedError::Io)?;
        Ok(())
    }

    /// Generate XML schema definition
    pub fn generate_schema(&self, output_dir: &Path) -> crate::error::Result<()> {
        let schema_path = output_dir.join("documentation.xsd");
        
        let schema = r#"<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified">
  
  <!-- Root element -->
  <xs:element name="documentation">
    <xs:complexType>
      <xs:sequence>
        <xs:element name="metadata" type="MetadataType"/>
        <xs:element name="configuration" type="ConfigurationType"/>
        <xs:element name="statistics" type="StatisticsType"/>
        <xs:element name="modules" type="ModulesType"/>
      </xs:sequence>
    </xs:complexType>
  </xs:element>
  
  <!-- Metadata type -->
  <xs:complexType name="MetadataType">
    <xs:sequence>
      <xs:element name="title" type="xs:string"/>
      <xs:element name="description" type="xs:string" minOccurs="0"/>
      <xs:element name="version" type="xs:string" minOccurs="0"/>
      <xs:element name="authors" minOccurs="0">
        <xs:complexType>
          <xs:sequence>
            <xs:element name="author" type="xs:string" maxOccurs="unbounded"/>
          </xs:sequence>
        </xs:complexType>
      </xs:element>
      <xs:element name="generated_at" type="xs:dateTime"/>
      <xs:element name="generator" type="xs:string"/>
      <xs:element name="format_version" type="xs:string"/>
    </xs:sequence>
  </xs:complexType>
  
  <!-- Configuration type -->
  <xs:complexType name="ConfigurationType">
    <xs:sequence>
      <xs:element name="include_examples" type="xs:boolean"/>
      <xs:element name="include_private" type="xs:boolean"/>
      <xs:element name="generate_cross_refs" type="xs:boolean"/>
      <xs:element name="base_url" type="xs:string" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>
  
  <!-- Statistics type -->
  <xs:complexType name="StatisticsType">
    <xs:sequence>
      <xs:element name="total_modules" type="xs:int"/>
      <xs:element name="total_items" type="xs:int"/>
      <xs:element name="total_functions" type="xs:int"/>
      <xs:element name="total_structs" type="xs:int"/>
    </xs:sequence>
  </xs:complexType>
  
  <!-- Modules type -->
  <xs:complexType name="ModulesType">
    <xs:sequence>
      <xs:element name="module" type="ModuleType" maxOccurs="unbounded"/>
    </xs:sequence>
  </xs:complexType>
  
  <!-- Module type -->
  <xs:complexType name="ModuleType">
    <xs:sequence>
      <xs:element name="package" type="xs:string" minOccurs="0"/>
      <xs:element name="file_path" type="xs:string"/>
      <xs:element name="source_info" type="SourceInfoType"/>
      <xs:element name="imports" minOccurs="0">
        <xs:complexType>
          <xs:sequence>
            <xs:element name="import" type="xs:string" maxOccurs="unbounded"/>
          </xs:sequence>
        </xs:complexType>
      </xs:element>
      <xs:element name="items">
        <xs:complexType>
          <xs:sequence>
            <xs:element name="item" type="ItemType" maxOccurs="unbounded"/>
          </xs:sequence>
        </xs:complexType>
      </xs:element>
    </xs:sequence>
    <xs:attribute name="name" type="xs:string" use="required"/>
  </xs:complexType>
  
  <!-- Source info type -->
  <xs:complexType name="SourceInfoType">
    <xs:sequence>
      <xs:element name="file_size" type="xs:long"/>
      <xs:element name="line_count" type="xs:int"/>
      <xs:element name="encoding" type="xs:string"/>
      <xs:element name="last_modified" type="xs:long" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>
  
  <!-- Item type -->
  <xs:complexType name="ItemType">
    <xs:sequence>
      <xs:element name="module" type="xs:string"/>
      <xs:element name="summary" type="xs:string"/>
      <xs:element name="description" type="xs:string"/>
      <xs:element name="signature" type="xs:string" minOccurs="0"/>
      <xs:element name="parameters" minOccurs="0">
        <xs:complexType>
          <xs:sequence>
            <xs:element name="parameter" type="ParameterType" maxOccurs="unbounded"/>
          </xs:sequence>
        </xs:complexType>
      </xs:element>
      <xs:element name="return_type" type="xs:string" minOccurs="0"/>
      <xs:element name="examples" minOccurs="0">
        <xs:complexType>
          <xs:sequence>
            <xs:element name="example" type="ExampleType" maxOccurs="unbounded"/>
          </xs:sequence>
        </xs:complexType>
      </xs:element>
      <xs:element name="tags" minOccurs="0">
        <xs:complexType>
          <xs:sequence>
            <xs:element name="tag" maxOccurs="unbounded">
              <xs:complexType>
                <xs:simpleContent>
                  <xs:extension base="xs:string">
                    <xs:attribute name="name" type="xs:string" use="required"/>
                  </xs:extension>
                </xs:simpleContent>
              </xs:complexType>
            </xs:element>
          </xs:sequence>
        </xs:complexType>
      </xs:element>
      <xs:element name="location" type="LocationType"/>
      <xs:element name="source_code" type="xs:string" minOccurs="0"/>
    </xs:sequence>
    <xs:attribute name="name" type="xs:string" use="required"/>
    <xs:attribute name="kind" type="xs:string" use="required"/>
    <xs:attribute name="visibility" type="xs:string" use="required"/>
  </xs:complexType>
  
  <!-- Parameter type -->
  <xs:complexType name="ParameterType">
    <xs:sequence>
      <xs:element name="name" type="xs:string"/>
      <xs:element name="type" type="xs:string" minOccurs="0"/>
      <xs:element name="description" type="xs:string"/>
      <xs:element name="default_value" type="xs:string" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>
  
  <!-- Example type -->
  <xs:complexType name="ExampleType">
    <xs:sequence>
      <xs:element name="title" type="xs:string" minOccurs="0"/>
      <xs:element name="description" type="xs:string" minOccurs="0"/>
      <xs:element name="language" type="xs:string"/>
      <xs:element name="code" type="xs:string"/>
      <xs:element name="output" type="xs:string" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>
  
  <!-- Location type -->
  <xs:complexType name="LocationType">
    <xs:sequence>
      <xs:element name="line" type="xs:int"/>
      <xs:element name="column" type="xs:int"/>
      <xs:element name="file" type="xs:string" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>

</xs:schema>"#;
        
        fs::write(schema_path, schema).map_err(CursedError::Io)?;
        Ok(())
    }

    /// Escape XML special characters
    fn xml_escape(&self, text: &str) -> String {
        text.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&apos;")
    }
}
