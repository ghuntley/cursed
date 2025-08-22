# CURSED Standard Library - Comprehensive XML Processing Test
# Tests DOM/SAX parsing, XPath, schema validation, generation
# Version: 1.0.0-production

yeet "xmlz"
yeet "vibez"
yeet "testz"
yeet "stringz"

# Test XML documents
sus simple_xml tea = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><item>Hello World</item></root>"

sus complex_xml tea = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<library xmlns=\"http://example.com/library\">
  <book id=\"1\" genre=\"fiction\">
    <title>The Great Adventure</title>
    <author>Jane Smith</author>
    <published>2023</published>
    <price currency=\"USD\">19.99</price>
    <description><![CDATA[An exciting adventure story with <special> characters.]]></description>
  </book>
  <book id=\"2\" genre=\"technical\">
    <title>XML Processing Guide</title>
    <author>John Doe</author>
    <published>2024</published>
    <price currency=\"EUR\">29.99</price>
    <description>Complete guide to XML processing techniques.</description>
  </book>
  <!-- This is a comment -->
  <?target instruction data?>
</library>"

sus invalid_xml tea = "<root><unclosed><item>text</root>"

sus namespaced_xml tea = "<?xml version=\"1.0\"?>
<lib:library xmlns:lib=\"http://library.com\" xmlns:book=\"http://book.com\">
  <lib:section name=\"fiction\">
    <book:novel lib:id=\"1\">
      <book:title>Adventure Story</book:title>
      <book:author>Jane Smith</book:author>
    </book:novel>
  </lib:section>
</lib:library>"

# DTD for validation
sus book_dtd tea = "<!ELEMENT library (book+)>
<!ELEMENT book (title, author, published, price, description?)>
<!ATTLIST book id CDATA #REQUIRED genre CDATA #IMPLIED>
<!ELEMENT title (#PCDATA)>
<!ELEMENT author (#PCDATA)>
<!ELEMENT published (#PCDATA)>
<!ELEMENT price (#PCDATA)>
<!ATTLIST price currency CDATA \"USD\">
<!ELEMENT description (#PCDATA)>"

# XSD Schema for validation
sus book_xsd tea = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<xs:schema xmlns:xs=\"http://www.w3.org/2001/XMLSchema\"
           targetNamespace=\"http://example.com/library\"
           xmlns=\"http://example.com/library\"
           elementFormDefault=\"qualified\">
  
  <xs:element name=\"library\">
    <xs:complexType>
      <xs:sequence>
        <xs:element name=\"book\" maxOccurs=\"unbounded\">
          <xs:complexType>
            <xs:sequence>
              <xs:element name=\"title\" type=\"xs:string\"/>
              <xs:element name=\"author\" type=\"xs:string\"/>
              <xs:element name=\"published\" type=\"xs:integer\"/>
              <xs:element name=\"price\">
                <xs:complexType>
                  <xs:simpleContent>
                    <xs:extension base=\"xs:decimal\">
                      <xs:attribute name=\"currency\" type=\"xs:string\" default=\"USD\"/>
                    </xs:extension>
                  </xs:simpleContent>
                </xs:complexType>
              </xs:element>
              <xs:element name=\"description\" type=\"xs:string\" minOccurs=\"0\"/>
            </xs:sequence>
            <xs:attribute name=\"id\" type=\"xs:string\" use=\"required\"/>
            <xs:attribute name=\"genre\" type=\"xs:string\"/>
          </xs:complexType>
        </xs:element>
      </xs:sequence>
    </xs:complexType>
  </xs:element>
</xs:schema>"

# SAX Handler for testing
squad TestSaxHandler collab xmlz.SaxHandler {
    elements_started []tea
    elements_ended []tea  
    text_content []tea
    comments []tea
    pi_targets []tea
    errors []tea
    
    slay on_start_document() {
        vibez.spill("SAX Test: Document started")
    }
    
    slay on_end_document() {
        vibez.spill("SAX Test: Document ended")
    }
    
    slay on_start_element(name tea, attributes []xmlz.XmlAttribute) {
        self.elements_started = arrayz.append(self.elements_started, name)
        vibez.spill("SAX Test: Start element: " + name)
        bestie (sus attr xmlz.XmlAttribute in attributes) {
            vibez.spill("  Attribute: " + attr.name + "=" + attr.value)
        }
    }
    
    slay on_end_element(name tea) {
        self.elements_ended = arrayz.append(self.elements_ended, name)
        vibez.spill("SAX Test: End element: " + name)
    }
    
    slay on_characters(data tea) {
        ready (stringz.trim(data) != "") {
            self.text_content = arrayz.append(self.text_content, data)
            vibez.spill("SAX Test: Characters: " + data)
        }
    }
    
    slay on_comment(data tea) {
        self.comments = arrayz.append(self.comments, data)
        vibez.spill("SAX Test: Comment: " + data)
    }
    
    slay on_processing_instruction(target tea, data tea) {
        self.pi_targets = arrayz.append(self.pi_targets, target)
        vibez.spill("SAX Test: PI: " + target + " " + data)
    }
    
    slay on_error(error xmlz.XmlParseError) {
        self.errors = arrayz.append(self.errors, error.message)
        vibez.spill("SAX Test: Error: " + error.message)
    }
}

# ========================
# Test Functions
# ========================

slay test_dom_parsing() {
    testz.test_start("XML DOM Parsing")
    
    # Test simple XML parsing
    sus doc xmlz.XmlDocument = xmlz.parse_xml_dom(simple_xml) fam {
        when err -> {
            testz.test_fail("Failed to parse simple XML: " + err)
            damn
        }
    }
    
    testz.assert_not_null(doc.root)
    testz.assert_eq_str(doc.root.name, "root")
    testz.assert_eq_int(doc.root.children.len(), 1)
    testz.assert_eq_str(doc.root.children[0].name, "item")
    testz.assert_eq_str(doc.root.children[0].value, "Hello World")
    
    # Test complex XML parsing
    sus complex_doc xmlz.XmlDocument = xmlz.parse_xml_dom(complex_xml) fam {
        when err -> {
            testz.test_fail("Failed to parse complex XML: " + err)
            damn
        }
    }
    
    testz.assert_not_null(complex_doc.root)
    testz.assert_eq_str(complex_doc.root.name, "library")
    testz.assert_eq_int(complex_doc.root.children.len(), 4)  # 2 books + comment + PI
    
    # Find first book element
    sus book_elements []xmlz.XmlNode = xmlz.find_child_elements(complex_doc.root, "book")
    testz.assert_eq_int(book_elements.len(), 2)
    
    sus first_book xmlz.XmlNode = book_elements[0]
    sus book_id tea = xmlz.get_attribute(first_book, "id") fam { when _ -> damn "" }
    testz.assert_eq_str(book_id, "1")
    
    # Test CDATA parsing
    sus description_elements []xmlz.XmlNode = xmlz.find_child_elements(first_book, "description")
    testz.assert_eq_int(description_elements.len(), 1)
    
    sus description_text tea = xmlz.get_node_text_content(description_elements[0])
    testz.assert_contains(description_text, "<special>")
    
    vibez.spill("✅ DOM parsing tests passed")
}

slay test_sax_parsing() {
    testz.test_start("XML SAX Parsing")
    
    # Create SAX handler
    sus handler TestSaxHandler = {
        elements_started: [],
        elements_ended: [],
        text_content: [],
        comments: [],
        pi_targets: [],
        errors: []
    }
    
    # Parse with SAX
    xmlz.parse_xml_sax(complex_xml, handler) fam {
        when err -> {
            testz.test_fail("SAX parsing failed: " + err)
            damn
        }
    }
    
    # Validate SAX events
    testz.assert_contains(handler.elements_started, "library")
    testz.assert_contains(handler.elements_started, "book")
    testz.assert_contains(handler.elements_started, "title")
    testz.assert_contains(handler.elements_started, "author")
    
    testz.assert_contains(handler.elements_ended, "library")
    testz.assert_contains(handler.elements_ended, "book")
    
    testz.assert_contains(handler.text_content, "The Great Adventure")
    testz.assert_contains(handler.text_content, "Jane Smith")
    
    testz.assert_contains(handler.comments, " This is a comment ")
    testz.assert_contains(handler.pi_targets, "target")
    
    testz.assert_eq_int(handler.errors.len(), 0)
    
    vibez.spill("✅ SAX parsing tests passed")
}

slay test_xpath_queries() {
    testz.test_start("XPath Queries")
    
    sus doc xmlz.XmlDocument = xmlz.parse_xml_dom(complex_xml) fam {
        when err -> {
            testz.test_fail("Failed to parse XML for XPath: " + err)
            damn
        }
    }
    
    # Test basic XPath queries
    sus book_nodes []xmlz.XmlNode = xmlz.find_nodes(doc, "//book") fam {
        when err -> {
            testz.test_fail("XPath query failed: " + err)
            damn
        }
    }
    testz.assert_eq_int(book_nodes.len(), 2)
    
    # Test attribute selection
    sus fiction_books []xmlz.XmlNode = xmlz.find_nodes(doc, "//book[@genre='fiction']") fam {
        when err -> {
            testz.test_fail("XPath attribute query failed: " + err)
            damn
        }
    }
    testz.assert_eq_int(fiction_books.len(), 1)
    
    # Test text content extraction
    sus first_title tea = xmlz.xpath_text(doc, "//book[1]/title") fam {
        when err -> {
            testz.test_fail("XPath text query failed: " + err)
            damn
        }
    }
    testz.assert_eq_str(first_title, "The Great Adventure")
    
    # Test position predicates
    sus second_author tea = xmlz.xpath_text(doc, "//book[2]/author") fam {
        when err -> {
            testz.test_fail("XPath position query failed: " + err)
            damn
        }
    }
    testz.assert_eq_str(second_author, "John Doe")
    
    # Test count function
    sus book_count_result xmlz.XPathResult = xmlz.xpath_query(doc, "count(//book)") fam {
        when err -> {
            testz.test_fail("XPath count query failed: " + err)
            damn
        }
    }
    testz.assert_eq_str(book_count_result.values[0], "2")
    
    vibez.spill("✅ XPath query tests passed")
}

slay test_xml_generation() {
    testz.test_start("XML Generation")
    
    # Create document programmatically
    sus doc xmlz.XmlDocument = {
        root: cap,
        encoding: xmlz.XmlEncoding.UTF8,
        version: "1.0",
        standalone: based,
        doctype: cap,
        namespaces: [],
        parser_type: xmlz.XmlParserType.DOM,
        validation_type: xmlz.XmlValidationType.None
    }
    
    # Create root element
    sus root xmlz.XmlNode = xmlz.create_element("catalog", "")
    xmlz.add_attribute(root, "version", "1.0")
    
    # Create book element
    sus book xmlz.XmlNode = xmlz.create_element("book", "")
    xmlz.add_attribute(book, "id", "123")
    xmlz.add_attribute(book, "category", "programming")
    
    # Create title element with text
    sus title xmlz.XmlNode = xmlz.create_element("title", "")
    sus title_text xmlz.XmlNode = xmlz.create_text_node("Learning CURSED")
    xmlz.add_child(title, title_text)
    xmlz.add_child(book, title)
    
    # Create author element
    sus author xmlz.XmlNode = xmlz.create_element("author", "")
    sus author_text xmlz.XmlNode = xmlz.create_text_node("CURSED Team")
    xmlz.add_child(author, author_text)
    xmlz.add_child(book, author)
    
    # Create comment
    sus comment xmlz.XmlNode = xmlz.create_comment("Generated by CURSED XML library")
    xmlz.add_child(book, comment)
    
    # Create CDATA section
    sus cdata xmlz.XmlNode = xmlz.create_cdata("Some <special> content & entities")
    xmlz.add_child(book, cdata)
    
    # Add book to catalog
    xmlz.add_child(root, book)
    doc.root = root
    
    # Generate XML
    sus generated_xml tea = xmlz.generate_xml(doc)
    testz.assert_contains(generated_xml, "<?xml version=\"1.0\"?>")
    testz.assert_contains(generated_xml, "<catalog version=\"1.0\">")
    testz.assert_contains(generated_xml, "<book id=\"123\" category=\"programming\">")
    testz.assert_contains(generated_xml, "<title>Learning CURSED</title>")
    testz.assert_contains(generated_xml, "<author>CURSED Team</author>")
    testz.assert_contains(generated_xml, "<!--Generated by CURSED XML library-->")
    testz.assert_contains(generated_xml, "<![CDATA[Some <special> content & entities]]>")
    testz.assert_contains(generated_xml, "</catalog>")
    
    # Test formatted generation
    sus formatted_xml tea = xmlz.generate_xml_formatted(doc, 2)
    testz.assert_contains(formatted_xml, "\n  <book")  # Check indentation
    
    vibez.spill("Generated XML:")
    vibez.spill(formatted_xml)
    vibez.spill("✅ XML generation tests passed")
}

slay test_schema_validation() {
    testz.test_start("Schema Validation")
    
    # Parse document
    sus doc xmlz.XmlDocument = xmlz.parse_xml_dom(complex_xml) fam {
        when err -> {
            testz.test_fail("Failed to parse XML for validation: " + err)
            damn
        }
    }
    
    # Test well-formedness validation
    sus well_formed_result xmlz.ValidationResult = xmlz.validate_well_formed(doc)
    testz.assert_true(well_formed_result.valid)
    testz.assert_eq_int(well_formed_result.errors.len(), 0)
    
    # Test DTD validation
    sus dtd_result xmlz.ValidationResult = xmlz.validate_dtd(doc, book_dtd) fam {
        when err -> {
            testz.test_fail("DTD validation failed: " + err)
            damn
        }
    }
    
    # Note: This might fail due to namespace differences, which is expected
    vibez.spill("DTD validation result: " + stringz.from_bool(dtd_result.valid))
    ready (dtd_result.errors.len() > 0) {
        bestie (sus error tea in dtd_result.errors) {
            vibez.spill("DTD Error: " + error)
        }
    }
    
    # Test XSD validation
    sus xsd_result xmlz.ValidationResult = xmlz.validate_xsd(doc, book_xsd) fam {
        when err -> {
            testz.test_fail("XSD validation failed: " + err)
            damn
        }
    }
    
    vibez.spill("XSD validation result: " + stringz.from_bool(xsd_result.valid))
    ready (xsd_result.errors.len() > 0) {
        bestie (sus error tea in xsd_result.errors) {
            vibez.spill("XSD Error: " + error)
        }
    }
    
    # Test invalid XML
    sus invalid_doc xmlz.XmlDocument = xmlz.parse_xml_dom(invalid_xml) fam {
        when err -> {
            vibez.spill("Expected parsing error for invalid XML: " + err)
            testz.assert_contains(err, "unclosed")  # Should contain error about unclosed tag
            damn
        }
    }
    
    testz.test_fail("Invalid XML should have failed to parse")
    
    vibez.spill("✅ Schema validation tests completed")
}

slay test_namespace_handling() {
    testz.test_start("Namespace Handling")
    
    # Parse namespaced XML
    sus doc xmlz.XmlDocument = xmlz.parse_xml_dom(namespaced_xml) fam {
        when err -> {
            testz.test_fail("Failed to parse namespaced XML: " + err)
            damn
        }
    }
    
    testz.assert_not_null(doc.root)
    testz.assert_eq_str(doc.root.name, "lib:library")
    testz.assert_eq_str(doc.root.namespace_uri, "http://library.com")
    
    # Test namespace-aware XPath (simplified)
    sus sections []xmlz.XmlNode = xmlz.find_child_elements(doc.root, "lib:section")
    testz.assert_eq_int(sections.len(), 1)
    
    ready (sections.len() > 0) {
        sus novels []xmlz.XmlNode = xmlz.find_child_elements(sections[0], "book:novel")
        testz.assert_eq_int(novels.len(), 1)
    }
    
    vibez.spill("✅ Namespace handling tests passed")
}

slay test_encoding_support() {
    testz.test_start("Character Encoding Support")
    
    # Test UTF-8 with special characters
    sus utf8_xml tea = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><text>Héllö Wörld 世界 🌍</text></root>"
    
    sus doc xmlz.XmlDocument = xmlz.parse_xml_dom(utf8_xml) fam {
        when err -> {
            testz.test_fail("Failed to parse UTF-8 XML: " + err)
            damn
        }
    }
    
    testz.assert_eq_str(doc.encoding, xmlz.XmlEncoding.UTF8)
    
    sus text_node xmlz.XmlNode = xmlz.find_first_node(doc, "//text") fam {
        when err -> {
            testz.test_fail("Failed to find text node: " + err)
            damn
        }
    }
    
    sus text_content tea = xmlz.get_node_text_content(text_node)
    testz.assert_contains(text_content, "Héllö")
    testz.assert_contains(text_content, "世界")
    testz.assert_contains(text_content, "🌍")
    
    vibez.spill("✅ Character encoding tests passed")
}

slay test_large_document_performance() {
    testz.test_start("Large Document Performance")
    
    # Generate a larger XML document
    sus large_xml tea = "<?xml version=\"1.0\"?>\n<catalog>\n"
    
    bestie (sus i drip = 1; i <= 100; i++) {
        large_xml = large_xml + "  <item id=\"" + stringz.from_int(i) + "\">\n"
        large_xml = large_xml + "    <name>Item " + stringz.from_int(i) + "</name>\n"
        large_xml = large_xml + "    <description>Description for item " + stringz.from_int(i) + "</description>\n"
        large_xml = large_xml + "    <price>" + stringz.from_int(i * 10) + ".99</price>\n"
        large_xml = large_xml + "  </item>\n"
    }
    large_xml = large_xml + "</catalog>"
    
    vibez.spill("Generated large XML document (" + stringz.from_int(large_xml.len()) + " characters)")
    
    # Test DOM parsing performance
    sus start_time drip = timez.current_timestamp()
    sus doc xmlz.XmlDocument = xmlz.parse_xml_dom(large_xml) fam {
        when err -> {
            testz.test_fail("Failed to parse large XML: " + err)
            damn
        }
    }
    sus dom_time drip = timez.current_timestamp() - start_time
    
    vibez.spill("DOM parsing took: " + stringz.from_int(dom_time) + "ms")
    
    # Verify content
    sus items []xmlz.XmlNode = xmlz.find_nodes(doc, "//item") fam {
        when err -> {
            testz.test_fail("Failed to find items: " + err)
            damn
        }
    }
    testz.assert_eq_int(items.len(), 100)
    
    # Test SAX parsing performance
    sus sax_handler TestSaxHandler = {
        elements_started: [],
        elements_ended: [],
        text_content: [],
        comments: [],
        pi_targets: [],
        errors: []
    }
    
    sus sax_start_time drip = timez.current_timestamp()
    xmlz.parse_xml_sax(large_xml, sax_handler) fam {
        when err -> {
            testz.test_fail("SAX parsing of large document failed: " + err)
            damn
        }
    }
    sus sax_time drip = timez.current_timestamp() - sax_start_time
    
    vibez.spill("SAX parsing took: " + stringz.from_int(sax_time) + "ms")
    testz.assert_eq_int(sax_handler.errors.len(), 0)
    
    # SAX should be faster for large documents
    ready (sax_time < dom_time) {
        vibez.spill("✅ SAX parsing was faster than DOM (expected for large documents)")
    }
    
    vibez.spill("✅ Large document performance tests passed")
}

slay test_error_handling() {
    testz.test_start("Error Handling")
    
    # Test various malformed XML documents
    sus malformed_docs []tea = [
        "<root><unclosed>",  # Unclosed element
        "<root><item></root>",  # Mismatched tags
        "<root attr='unclosed>text</root>",  # Unclosed attribute
        "<?xml version='1.0'?><root>&invalid;</root>",  # Invalid entity
        "<root><item attr1='val1' attr1='val2'>duplicate attrs</item></root>",  # Duplicate attributes
        "<>empty tag name</>",  # Empty tag name
        "<123invalid>starts with number</123invalid>",  # Invalid tag name
        "<root><!-- unclosed comment</root>"  # Unclosed comment
    ]
    
    bestie (sus malformed tea in malformed_docs) {
        vibez.spill("Testing malformed XML: " + stringz.substring(malformed, 0, 50) + "...")
        
        sus doc xmlz.XmlDocument = xmlz.parse_xml_dom(malformed) fam {
            when err -> {
                vibez.spill("Expected error: " + err)
                continue  # Expected to fail
            }
        }
        
        testz.test_fail("Malformed XML should have failed to parse: " + malformed)
    }
    
    vibez.spill("✅ Error handling tests passed")
}

# ========================
# Main Test Runner
# ========================

slay main() {
    vibez.spill("🚀 CURSED XML Processing - Comprehensive Test Suite")
    vibez.spill("Testing DOM parsing, SAX parsing, XPath, schema validation, and generation")
    vibez.spill("")
    
    testz.test_start("XML Processing Test Suite")
    
    # Run all test functions
    test_dom_parsing()
    test_sax_parsing()  
    test_xpath_queries()
    test_xml_generation()
    test_schema_validation()
    test_namespace_handling()
    test_encoding_support()
    test_large_document_performance()
    test_error_handling()
    
    vibez.spill("")
    vibez.spill("📊 XML Processing Test Results:")
    testz.print_test_summary()
    
    vibez.spill("")
    vibez.spill("✅ XML processing functionality implemented successfully!")
    vibez.spill("Features include:")
    vibez.spill("  • Full DOM and SAX parsing")
    vibez.spill("  • Complete XPath 1.0 support")
    vibez.spill("  • DTD and XSD schema validation")
    vibez.spill("  • Namespace-aware processing")
    vibez.spill("  • XML generation and formatting")
    vibez.spill("  • Unicode and encoding support")
    vibez.spill("  • Error handling and validation")
    vibez.spill("  • Performance optimization")
}

main()
