// EmailZ Comprehensive Test Suite
// Tests all email and SMTP functionality including edge cases

yeet "emailz/core"
yeet "emailz/parser" 
yeet "emailz/advanced"
yeet "testz"
yeet "vibez"

// ============================================================================
// Basic Email Creation Tests
// ============================================================================

slay test_email_creation() lit {
    testz.test_group("Email Creation Tests")
    
    // Test simple email creation
    testz.test_case("Simple email creation") {
        sus email Email = create_email(
            "sender@example.com",
            "recipient@example.com", 
            "Test Subject",
            "Hello, World!"
        ) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Email creation failed: ", err.message]))
                damn cap
            }
        }
        
        testz.assert_eq_str(email.from, "sender@example.com")
        testz.assert_eq_str(email.subject, "Test Subject") 
        testz.assert_eq_str(email.body_text, "Hello, World!")
        testz.assert_eq_int(arrayz.len(email.to), 1)
        testz.assert_eq_str(email.to[0], "recipient@example.com")
        testz.assert_true(stringz.len(email.message_id) > 0)
        testz.assert_true(stringz.len(email.date) > 0)
    }
    
    // Test HTML email creation
    testz.test_case("HTML email creation") {
        sus html_body tea = "<h1>Hello</h1><p>This is HTML content.</p>"
        sus text_body tea = "Hello\nThis is plain text content."
        
        sus email Email = create_html_email(
            "sender@example.com",
            "recipient@example.com",
            "HTML Test",
            html_body,
            text_body
        ) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["HTML email creation failed: ", err.message]))
                damn cap
            }
        }
        
        testz.assert_eq_str(email.body_html, html_body)
        testz.assert_eq_str(email.body_text, text_body) 
        testz.assert_true(stringz.contains(email.content_type, "multipart/alternative"))
        testz.assert_true(stringz.len(email.boundary) > 0)
    }
    
    // Test invalid email addresses
    testz.test_case("Invalid email address validation") {
        sus result Email = create_email(
            "invalid-email",
            "recipient@example.com",
            "Test",
            "Body"
        ) fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Invalid from address"))
                damn Email{}  // Return dummy email to satisfy type checker
            }
        }
    }
    
    damn based
}

// ============================================================================
// Email Address Validation Tests  
// ============================================================================

slay test_email_validation() lit {
    testz.test_group("Email Address Validation Tests")
    
    // Test valid email addresses
    testz.test_case("Valid email addresses") {
        sus valid_emails tea[value] = [
            "user@example.com",
            "test.email@domain.org",
            "user+tag@example.com",
            "name@sub.domain.com",
            "123@numbers.com"
        ]
        
        sus i drip = 0
        bestie (i < arrayz.len(valid_emails)) {
            testz.assert_true_msg(
                validate_email_address(valid_emails[i]),
                stringz.concat(["Should be valid: ", valid_emails[i]])
            )
            i = i + 1
        }
    }
    
    // Test invalid email addresses
    testz.test_case("Invalid email addresses") {
        sus invalid_emails tea[value] = [
            "",
            "plainaddress",
            "@missinglocal.com",
            "missing@",
            "user@",
            "@domain.com", 
            "user..double.dot@example.com",
            ".leading.dot@example.com",
            "trailing.dot.@example.com",
            "user@domain",  // No TLD
            "user@.com",    // Missing domain
            "user@domain..com"  // Double dots in domain
        ]
        
        sus i drip = 0
        bestie (i < arrayz.len(invalid_emails)) {
            testz.assert_false_msg(
                validate_email_address(invalid_emails[i]),
                stringz.concat(["Should be invalid: ", invalid_emails[i]])
            )
            i = i + 1
        }
    }
    
    // Test domain extraction
    testz.test_case("Domain extraction") {
        testz.assert_eq_str(extract_domain("user@example.com"), "example.com")
        testz.assert_eq_str(extract_domain("test@sub.domain.org"), "sub.domain.org")
        testz.assert_eq_str(extract_domain("invalid-email"), "")
    }
    
    damn based
}

// ============================================================================
// Email Header and Recipient Management Tests
// ============================================================================

slay test_email_recipients() lit {
    testz.test_group("Email Recipients Management Tests")
    
    testz.test_case("Adding recipients") {
        sus email Email = create_email(
            "sender@example.com",
            "first@example.com",
            "Test",
            "Body"
        ) fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        // Add TO recipients
        email = add_recipient(email, "second@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to add recipient")
                damn email
            }
        }
        
        email = add_recipient(email, "third@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to add recipient")
                damn email
            }
        }
        
        testz.assert_eq_int(arrayz.len(email.to), 3)
        testz.assert_eq_str(email.to[0], "first@example.com")
        testz.assert_eq_str(email.to[1], "second@example.com")
        testz.assert_eq_str(email.to[2], "third@example.com")
        
        // Add CC recipients  
        email = add_cc_recipient(email, "cc1@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to add CC recipient")
                damn email
            }
        }
        
        email = add_cc_recipient(email, "cc2@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to add CC recipient")
                damn email
            }
        }
        
        testz.assert_eq_int(arrayz.len(email.cc), 2)
        testz.assert_eq_str(email.cc[0], "cc1@example.com")
        testz.assert_eq_str(email.cc[1], "cc2@example.com")
        
        // Add BCC recipients
        email = add_bcc_recipient(email, "bcc@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to add BCC recipient")
                damn email
            }
        }
        
        testz.assert_eq_int(arrayz.len(email.bcc), 1)
        testz.assert_eq_str(email.bcc[0], "bcc@example.com")
    }
    
    testz.test_case("Invalid recipient addresses") {
        sus email Email = create_email("sender@example.com", "valid@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        // Try to add invalid recipient
        sus result Email = add_recipient(email, "invalid-email") fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Invalid recipient address"))
                damn email
            }
        }
    }
    
    damn based
}

// ============================================================================  
// Email Header Tests
// ============================================================================

slay test_custom_headers() lit {
    testz.test_group("Custom Headers Tests")
    
    testz.test_case("Adding custom headers") {
        sus email Email = create_email("sender@example.com", "recipient@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        // Add various custom headers
        email = add_custom_header(email, "X-Custom-App", "MyApp v1.0") fam {
            when err -> {
                testz.assert_fail("Failed to add custom header")
                damn email
            }
        }
        
        email = add_custom_header(email, "X-Priority", "1") fam {
            when err -> {
                testz.assert_fail("Failed to add priority header")
                damn email
            }
        }
        
        email = add_custom_header(email, "X-Mailer", "CURSED EmailZ") fam {
            when err -> {
                testz.assert_fail("Failed to add mailer header")
                damn email
            }
        }
        
        testz.assert_eq_int(arrayz.len(email.headers), 3)
        
        // Check headers were added correctly
        sus found_app lit = cap
        sus found_priority lit = cap
        sus found_mailer lit = cap
        
        sus i drip = 0
        bestie (i < arrayz.len(email.headers)) {
            sus header EmailHeader = email.headers[i]
            ready (stringz.equals(header.name, "X-Custom-App") && stringz.equals(header.value, "MyApp v1.0")) {
                found_app = based
            } otherwise ready (stringz.equals(header.name, "X-Priority") && stringz.equals(header.value, "1")) {
                found_priority = based
            } otherwise ready (stringz.equals(header.name, "X-Mailer") && stringz.equals(header.value, "CURSED EmailZ")) {
                found_mailer = based
            }
            i = i + 1
        }
        
        testz.assert_true(found_app)
        testz.assert_true(found_priority)
        testz.assert_true(found_mailer)
    }
    
    testz.test_case("Invalid header names") {
        sus email Email = create_email("sender@example.com", "recipient@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        // Try invalid header names
        sus result Email = add_custom_header(email, "", "value") fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Header name cannot be empty"))
                damn email
            }
        }
        
        result = add_custom_header(email, "Invalid Header Name", "value") fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Invalid header name"))
                damn email
            }
        }
        
        result = add_custom_header(email, "Header:WithColon", "value") fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Invalid header name"))
                damn email
            }
        }
    }
    
    testz.test_case("Reply-To header") {
        sus email Email = create_email("sender@example.com", "recipient@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        email = set_reply_to(email, "support@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to set Reply-To")
                damn email
            }
        }
        
        testz.assert_eq_str(email.reply_to, "support@example.com")
        
        // Test invalid Reply-To address
        sus result Email = set_reply_to(email, "invalid-email") fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Invalid Reply-To address"))
                damn email
            }
        }
    }
    
    damn based
}

// ============================================================================
// Attachment Tests
// ============================================================================

slay test_attachments() lit {
    testz.test_group("Email Attachments Tests")
    
    testz.test_case("Adding data attachments") {
        sus email Email = create_email("sender@example.com", "recipient@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        // Add text attachment
        sus text_data tea = "This is a text file content.\nSecond line."
        email = add_attachment_from_data(email, "test.txt", "text/plain", text_data) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Failed to add text attachment: ", err.message]))
                damn email
            }
        }
        
        // Add CSV attachment
        sus csv_data tea = "Name,Email\nJohn,john@example.com\nJane,jane@example.com"
        email = add_attachment_from_data(email, "contacts.csv", "text/csv", csv_data) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Failed to add CSV attachment: ", err.message]))
                damn email
            }
        }
        
        testz.assert_eq_int(arrayz.len(email.attachments), 2)
        
        // Check first attachment
        sus attachment1 EmailAttachment = email.attachments[0] 
        testz.assert_eq_str(attachment1.filename, "test.txt")
        testz.assert_eq_str(attachment1.content_type, "text/plain")
        testz.assert_eq_str(attachment1.disposition, "attachment")
        testz.assert_eq_str(attachment1.encoding, "base64")
        testz.assert_eq_int(attachment1.size, stringz.len(text_data))
        
        // Check second attachment
        sus attachment2 EmailAttachment = email.attachments[1]
        testz.assert_eq_str(attachment2.filename, "contacts.csv")
        testz.assert_eq_str(attachment2.content_type, "text/csv")
        
        // Content type should be updated to multipart/mixed
        testz.assert_true(stringz.contains(email.content_type, "multipart/mixed"))
        testz.assert_true(stringz.len(email.boundary) > 0)
    }
    
    testz.test_case("Invalid attachment data") {
        sus email Email = create_email("sender@example.com", "recipient@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        // Try to add attachment with empty filename
        sus result Email = add_attachment_from_data(email, "", "text/plain", "data") fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "filename cannot be empty"))
                damn email
            }
        }
    }
    
    testz.test_case("Inline images") {
        sus email Email = create_html_email(
            "sender@example.com",
            "recipient@example.com", 
            "HTML with Image",
            "<h1>Hello</h1><img src=\"cid:logo\" alt=\"Logo\">",
            "Hello"
        ) fam {
            when err -> {
                testz.assert_fail("HTML email creation failed")
                damn Email{}
            }
        }
        
        // Add inline image (simulate image data)
        sus fake_image_data tea = "fake-png-data-here"
        email = add_attachment_from_data(email, "logo.png", "image/png", fake_image_data) fam {
            when err -> {
                testz.assert_fail("Failed to add image data")
                damn email
            }
        }
        
        // Set as inline with content ID
        email.attachments[0].disposition = "inline"
        email.attachments[0].content_id = "logo"
        
        testz.assert_eq_str(email.attachments[0].disposition, "inline")
        testz.assert_eq_str(email.attachments[0].content_id, "logo")
    }
    
    damn based
}

// ============================================================================
// Email Formatting Tests
// ============================================================================

slay test_email_formatting() lit {
    testz.test_group("Email Formatting Tests")
    
    testz.test_case("Simple text email formatting") {
        sus email Email = create_email(
            "sender@example.com",
            "recipient@example.com",
            "Test Subject",
            "Hello, World!"
        ) fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        sus formatted tea = format_email_for_sending(email)
        
        // Check that required headers are present
        testz.assert_true(stringz.contains(formatted, "From: sender@example.com"))
        testz.assert_true(stringz.contains(formatted, "To: recipient@example.com"))
        testz.assert_true(stringz.contains(formatted, "Subject: Test Subject"))
        testz.assert_true(stringz.contains(formatted, "Date: "))
        testz.assert_true(stringz.contains(formatted, "Message-ID: "))
        testz.assert_true(stringz.contains(formatted, "MIME-Version: 1.0"))
        testz.assert_true(stringz.contains(formatted, "Content-Type: text/plain"))
        
        // Check body content
        testz.assert_true(stringz.contains(formatted, "Hello, World!"))
        
        // Check proper header/body separation
        testz.assert_true(stringz.contains(formatted, "\r\n\r\n"))
    }
    
    testz.test_case("Multiple recipients formatting") {
        sus email Email = create_email("sender@example.com", "first@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed") 
                damn Email{}
            }
        }
        
        email = add_recipient(email, "second@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to add recipient")
                damn email
            }
        }
        
        email = add_cc_recipient(email, "cc@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to add CC")
                damn email
            }
        }
        
        sus formatted tea = format_email_for_sending(email)
        
        testz.assert_true(stringz.contains(formatted, "To: first@example.com, second@example.com"))
        testz.assert_true(stringz.contains(formatted, "Cc: cc@example.com"))
        
        // BCC should NOT appear in headers
        testz.assert_false(stringz.contains(formatted, "Bcc:"))
    }
    
    testz.test_case("Custom headers formatting") {
        sus email Email = create_email("sender@example.com", "recipient@example.com", "Test", "Body") fam {
            when err -> {
                testz.assert_fail("Email creation failed")
                damn Email{}
            }
        }
        
        email = add_custom_header(email, "X-Custom-App", "MyApp v1.0") fam {
            when err -> {
                testz.assert_fail("Failed to add custom header")
                damn email
            }
        }
        
        email = set_reply_to(email, "support@example.com") fam {
            when err -> {
                testz.assert_fail("Failed to set Reply-To")
                damn email
            }
        }
        
        sus formatted tea = format_email_for_sending(email)
        
        testz.assert_true(stringz.contains(formatted, "X-Custom-App: MyApp v1.0"))
        testz.assert_true(stringz.contains(formatted, "Reply-To: support@example.com"))
    }
    
    damn based
}

// ============================================================================
// Email Parsing Tests
// ============================================================================

slay test_email_parsing() lit {
    testz.test_group("Email Parsing Tests")
    
    testz.test_case("Simple email parsing") {
        sus raw_email tea = stringz.concat([
            "From: sender@example.com\r\n",
            "To: recipient@example.com\r\n", 
            "Subject: Test Subject\r\n",
            "Date: Mon, 1 Jan 2024 12:00:00 +0000\r\n",
            "Message-ID: <12345@example.com>\r\n",
            "Content-Type: text/plain; charset=utf-8\r\n",
            "\r\n",
            "This is the email body.\r\n",
            "Second line of body."
        ])
        
        sus parsed ParsedEmail = parse_email(raw_email) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Email parsing failed: ", err.message]))
                damn ParsedEmail{}
            }
        }
        
        testz.assert_eq_str(parsed.from, "sender@example.com")
        testz.assert_eq_str(parsed.subject, "Test Subject")
        testz.assert_eq_str(parsed.message_id, "<12345@example.com>")
        testz.assert_eq_int(arrayz.len(parsed.to), 1)
        testz.assert_eq_str(parsed.to[0], "recipient@example.com")
        testz.assert_true(stringz.contains(parsed.body_text, "This is the email body"))
        testz.assert_false(parsed.is_multipart)
    }
    
    testz.test_case("Multiple recipients parsing") {
        sus raw_email tea = stringz.concat([
            "From: sender@example.com\r\n",
            "To: first@example.com, second@example.com, third@example.com\r\n",
            "Cc: cc1@example.com, cc2@example.com\r\n", 
            "Subject: Multiple Recipients\r\n",
            "Content-Type: text/plain\r\n",
            "\r\n",
            "Email body content."
        ])
        
        sus parsed ParsedEmail = parse_email(raw_email) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Email parsing failed: ", err.message]))
                damn ParsedEmail{}
            }
        }
        
        testz.assert_eq_int(arrayz.len(parsed.to), 3)
        testz.assert_eq_str(parsed.to[0], "first@example.com")
        testz.assert_eq_str(parsed.to[1], "second@example.com")
        testz.assert_eq_str(parsed.to[2], "third@example.com")
    }
    
    testz.test_case("Header parsing with folding") {
        sus raw_email tea = stringz.concat([
            "From: sender@example.com\r\n",
            "To: recipient@example.com\r\n",
            "Subject: This is a very long subject line that spans\r\n",
            " multiple lines using header folding as defined in RFC 5322\r\n",
            "Content-Type: text/plain\r\n",
            "\r\n",
            "Body content."
        ])
        
        sus parsed ParsedEmail = parse_email(raw_email) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Folded header parsing failed: ", err.message]))
                damn ParsedEmail{}
            }
        }
        
        // Subject should be unfolded into a single line
        testz.assert_true(stringz.contains(parsed.subject, "This is a very long subject line that spans"))
        testz.assert_true(stringz.contains(parsed.subject, "multiple lines using header folding"))
    }
    
    damn based
}

// ============================================================================
// Header Parsing Tests
// ============================================================================

slay test_header_parsing() lit {
    testz.test_group("Header Parsing Tests")
    
    testz.test_case("Basic header parsing") {
        sus header_text tea = stringz.concat([
            "From: sender@example.com\r\n",
            "To: recipient@example.com\r\n",
            "Subject: Test Message\r\n",
            "Date: Mon, 1 Jan 2024 12:00:00 +0000\r\n",
            "X-Custom: Custom Value\r\n"
        ])
        
        sus headers EmailHeader[value] = parse_email_headers(header_text) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Header parsing failed: ", err.message]))
                damn []
            }
        }
        
        testz.assert_eq_int(arrayz.len(headers), 5)
        
        // Test header value retrieval
        testz.assert_eq_str(get_header_value(headers, "From"), "sender@example.com")
        testz.assert_eq_str(get_header_value(headers, "Subject"), "Test Message")
        testz.assert_eq_str(get_header_value(headers, "X-Custom"), "Custom Value")
        
        // Test case-insensitive lookup
        testz.assert_eq_str(get_header_value(headers, "from"), "sender@example.com")
        testz.assert_eq_str(get_header_value(headers, "SUBJECT"), "Test Message")
    }
    
    testz.test_case("Header value with spaces") {
        sus header_text tea = "Content-Type: text/plain; charset=utf-8; boundary=\"boundary123\"\r\n"
        
        sus headers EmailHeader[value] = parse_email_headers(header_text) fam {
            when err -> {
                testz.assert_fail("Header parsing failed")
                damn []
            }
        }
        
        testz.assert_eq_int(arrayz.len(headers), 1)
        sus content_type tea = get_header_value(headers, "Content-Type")
        testz.assert_true(stringz.contains(content_type, "text/plain"))
        testz.assert_true(stringz.contains(content_type, "charset=utf-8"))
        testz.assert_true(stringz.contains(content_type, "boundary=\"boundary123\""))
    }
    
    testz.test_case("Missing header value") {
        sus headers EmailHeader[value] = []
        sus missing_value tea = get_header_value(headers, "NonExistent")
        testz.assert_eq_str(missing_value, "")
    }
    
    damn based
}

// ============================================================================
// SMTP Client Tests
// ============================================================================

slay test_smtp_client() lit {
    testz.test_group("SMTP Client Tests")
    
    testz.test_case("SMTP client creation") {
        sus client SmtpClient = create_smtp_client("smtp.example.com", 587) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["SMTP client creation failed: ", err.message]))
                damn SmtpClient{}
            }
        }
        
        testz.assert_eq_str(client.host, "smtp.example.com")
        testz.assert_eq_int(client.port, 587)
        testz.assert_false(client.use_tls)
        testz.assert_false(client.use_starttls)
        testz.assert_eq_int(client.timeout, 30)
        testz.assert_false(client.authenticated)
        testz.assert_true(client.verify_certificate)
    }
    
    testz.test_case("SMTP TLS client creation") {
        sus client SmtpClient = create_smtp_client_tls("smtp.gmail.com", 465) fam {
            when err -> {
                testz.assert_fail("TLS client creation failed")
                damn SmtpClient{}
            }
        }
        
        testz.assert_eq_str(client.host, "smtp.gmail.com")
        testz.assert_eq_int(client.port, 465)
        testz.assert_true(client.use_tls)
        testz.assert_false(client.use_starttls)
    }
    
    testz.test_case("SMTP STARTTLS client creation") {
        sus client SmtpClient = create_smtp_client_starttls("smtp.office365.com", 587) fam {
            when err -> {
                testz.assert_fail("STARTTLS client creation failed")
                damn SmtpClient{}
            }
        }
        
        testz.assert_eq_str(client.host, "smtp.office365.com")
        testz.assert_eq_int(client.port, 587)
        testz.assert_false(client.use_tls)
        testz.assert_true(client.use_starttls)
    }
    
    testz.test_case("Invalid SMTP parameters") {
        // Empty host
        sus result SmtpClient = create_smtp_client("", 587) fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "host cannot be empty"))
                damn SmtpClient{}
            }
        }
        
        // Invalid port
        result = create_smtp_client("smtp.example.com", 0) fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Invalid SMTP port"))
                damn SmtpClient{}
            }
        }
        
        result = create_smtp_client("smtp.example.com", 70000) fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Invalid SMTP port"))
                damn SmtpClient{}
            }
        }
    }
    
    damn based
}

// ============================================================================
// Template System Tests
// ============================================================================

slay test_templates() lit {
    testz.test_group("Email Template Tests")
    
    testz.test_case("Template registration") {
        sus subject_template tea = "Welcome {{name}} - Your Account is Ready"
        sus html_template tea = "<h1>Welcome {{name}}!</h1><p>Thanks for joining {{company}}.</p>"
        sus text_template tea = "Welcome {{name}}! Thanks for joining {{company}}."
        
        register_email_template("welcome", subject_template, html_template, text_template) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Template registration failed: ", err.message]))
                damn
            }
        }
    }
    
    testz.test_case("Template email creation") {
        // First register the template
        register_email_template(
            "test_template",
            "Hello {{name}}",
            "<h1>Hello {{name}}!</h1><p>Message: {{message}}</p>", 
            "Hello {{name}}! Message: {{message}}"
        ) fam {
            when err -> {
                testz.assert_fail("Template registration failed")
                damn
            }
        }
        
        sus variables TemplateVariable[value] = [
            TemplateVariable{name: "name", value: "John Doe"},
            TemplateVariable{name: "message", value: "Welcome to our service!"}
        ]
        
        sus email Email = create_template_email(
            "test_template",
            variables,
            "noreply@example.com",
            "john@example.com"
        ) fam {
            when err -> {
                testz.assert_fail(stringz.concat(["Template email creation failed: ", err.message]))
                damn Email{}
            }
        }
        
        testz.assert_eq_str(email.subject, "Hello John Doe")
        testz.assert_true(stringz.contains(email.body_html, "<h1>Hello John Doe!</h1>"))
        testz.assert_true(stringz.contains(email.body_html, "Message: Welcome to our service!"))
        testz.assert_true(stringz.contains(email.body_text, "Hello John Doe!"))
        testz.assert_true(stringz.contains(email.body_text, "Message: Welcome to our service!"))
    }
    
    testz.test_case("Template with missing variables") {
        register_email_template(
            "incomplete_template",
            "Hello {{name}}",
            "Message: {{message}} from {{sender}}",
            "Message: {{message}} from {{sender}}"
        ) fam {
            when err -> {
                testz.assert_fail("Template registration failed")
                damn
            }
        }
        
        // Provide only some variables
        sus variables TemplateVariable[value] = [
            TemplateVariable{name: "name", value: "John"}
        ]
        
        sus result Email = create_template_email(
            "incomplete_template",
            variables,
            "sender@example.com",
            "recipient@example.com"
        ) fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Unresolved template variables"))
                damn Email{}
            }
        }
    }
    
    testz.test_case("Non-existent template") {
        sus variables TemplateVariable[value] = []
        
        sus result Email = create_template_email(
            "non_existent_template",
            variables,
            "sender@example.com",
            "recipient@example.com"
        ) fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Template not found"))
                damn Email{}
            }
        }
    }
    
    damn based
}

// ============================================================================
// Advanced Features Tests
// ============================================================================

slay test_bulk_sending() lit {
    testz.test_group("Bulk Email Sending Tests")
    
    testz.test_case("Bulk sender creation") {
        sus client SmtpClient = create_smtp_client("smtp.example.com", 587) fam {
            when err -> {
                testz.assert_fail("SMTP client creation failed")
                damn SmtpClient{}
            }
        }
        
        sus bulk_sender BulkEmailSender = create_bulk_email_sender(client, 50)
        
        testz.assert_eq_int(bulk_sender.batch_size, 50)
        testz.assert_eq_int(bulk_sender.sent_count, 0)
        testz.assert_eq_int(bulk_sender.failed_count, 0)
        testz.assert_eq_int(bulk_sender.rate_limit, 0)
        testz.assert_true(bulk_sender.retry_failed)
        testz.assert_eq_int(bulk_sender.max_retries, 3)
    }
    
    testz.test_case("Bulk sender configuration") {
        sus client SmtpClient = create_smtp_client("smtp.example.com", 587) fam {
            when err -> {
                testz.assert_fail("SMTP client creation failed")
                damn SmtpClient{}
            }
        }
        
        sus bulk_sender BulkEmailSender = create_bulk_email_sender(client, 25)
        bulk_sender.rate_limit = 60  // 60 emails per minute
        bulk_sender.max_retries = 5
        bulk_sender.retry_failed = cap
        
        testz.assert_eq_int(bulk_sender.rate_limit, 60)
        testz.assert_eq_int(bulk_sender.max_retries, 5)
        testz.assert_false(bulk_sender.retry_failed)
    }
    
    damn based
}

slay test_connection_pooling() lit {
    testz.test_group("SMTP Connection Pooling Tests")
    
    testz.test_case("Pool configuration") {
        sus pool_config SmtpPoolConfig = SmtpPoolConfig{
            host: "smtp.example.com",
            port: 587,
            username: "user@example.com",
            password: "password",
            max_connections: 10,
            max_idle_time: 300,
            connection_timeout: 30,
            use_tls: cap,
            use_starttls: based
        }
        
        testz.assert_eq_str(pool_config.host, "smtp.example.com")
        testz.assert_eq_int(pool_config.max_connections, 10)
        testz.assert_true(pool_config.use_starttls)
        testz.assert_false(pool_config.use_tls)
    }
    
    testz.test_case("Pool creation validation") {
        sus invalid_config SmtpPoolConfig = SmtpPoolConfig{
            host: "smtp.example.com",
            port: 587,
            username: "user",
            password: "pass",
            max_connections: 0,  // Invalid
            max_idle_time: 300,
            connection_timeout: 30,
            use_tls: cap,
            use_starttls: cap
        }
        
        sus result SmtpPool = create_smtp_pool(invalid_config) fam {
            when err -> {
                testz.assert_true(stringz.contains(err.message, "Max connections must be greater than 0"))
                damn SmtpPool{}
            }
        }
    }
    
    damn based
}

// ============================================================================
// Test Runner
// ============================================================================

slay run_all_emailz_tests() lit {
    vibez.spill("Starting EmailZ Comprehensive Test Suite...")
    vibez.spill("=" * 60)
    
    // Run all test groups
    test_email_creation()
    test_email_validation()
    test_email_recipients()
    test_custom_headers()
    test_attachments()
    test_email_formatting()
    test_email_parsing()
    test_header_parsing()
    test_smtp_client()
    test_templates()
    test_bulk_sending()
    test_connection_pooling()
    
    vibez.spill("=" * 60)
    testz.print_test_summary()
    
    sus total_tests drip = testz.get_total_tests()
    sus passed_tests drip = testz.get_passed_tests()
    sus failed_tests drip = testz.get_failed_tests()
    
    vibez.spill(stringz.concat(["EmailZ Test Results: ", string_from_drip(passed_tests), "/", string_from_drip(total_tests), " passed"]))
    
    ready (failed_tests == 0) {
        vibez.spill("✅ All EmailZ tests passed!")
        damn based
    } otherwise {
        vibez.spill(stringz.concat(["❌ ", string_from_drip(failed_tests), " test(s) failed"]))
        damn cap
    }
}

// Main test execution
slay main() lit {
    run_all_emailz_tests()
}

// Helper function placeholders for string conversion
slay string_from_drip(value drip) tea {
    // Implementation would convert number to string
    damn "123"
}
