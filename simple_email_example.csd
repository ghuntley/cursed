// Simple EmailZ Example - Basic Usage
// Demonstrates core email functionality in CURSED

yeet "emailz"
yeet "vibez"

slay main() lit {
    vibez.spill("CURSED EmailZ - Simple Example")
    vibez.spill("=============================")
    vibez.spill()
    
    // Example 1: Create a simple text email
    vibez.spill("Creating simple text email...")
    sus simple_email Email = emailz.create_email(
        "sender@example.com",
        "recipient@example.com",
        "Hello from CURSED!",
        "This is a simple email created with the CURSED EmailZ library."
    ) fam {
        when err -> {
            vibez.spill("Error creating email:", err.message)
            damn cap
        }
    }
    
    vibez.spill("✅ Simple email created")
    vibez.spill("From:", simple_email.from)
    vibez.spill("To:", simple_email.to[0])
    vibez.spill("Subject:", simple_email.subject)
    vibez.spill()
    
    // Example 2: Create HTML email with attachment
    vibez.spill("Creating HTML email with attachment...")
    sus html_email Email = emailz.create_html_email(
        "noreply@cursedlang.org",
        "user@example.com",
        "Welcome to CURSED EmailZ",
        "<h1>Welcome!</h1><p>Thanks for trying <strong>CURSED EmailZ</strong>.</p>",
        "Welcome! Thanks for trying CURSED EmailZ."
    ) fam {
        when err -> {
            vibez.spill("Error creating HTML email:", err.message)
            damn cap
        }
    }
    
    // Add attachment
    sus attachment_data tea = "This is a sample text file attachment."
    html_email = emailz.add_attachment_from_data(
        html_email,
        "sample.txt",
        "text/plain",
        attachment_data
    ) fam {
        when err -> {
            vibez.spill("Error adding attachment:", err.message)
            damn html_email
        }
    }
    
    vibez.spill("✅ HTML email with attachment created")
    vibez.spill("Attachments:", arrayz.len(html_email.attachments))
    vibez.spill()
    
    // Example 3: Email validation
    vibez.spill("Testing email validation...")
    sus test_emails []tea = [
        "valid@example.com",
        "also.valid@domain.org",
        "invalid-email",
        "missing@",
        "@missing.com"
    ]
    
    sus i drip = 0
    bestie (i < arrayz.len(test_emails)) {
        sus email tea = test_emails[i]
        sus is_valid lit = emailz.validate_email_address(email)
        
        ready (is_valid) {
            vibez.spill("✅", email, "is valid")
        } otherwise {
            vibez.spill("❌", email, "is invalid")
        }
        i = i + 1
    }
    vibez.spill()
    
    // Example 4: Format email for sending
    vibez.spill("Formatting email for sending...")
    sus formatted tea = emailz.format_email_for_sending(simple_email)
    vibez.spill("✅ Email formatted (", stringz.len(formatted), "characters)")
    vibez.spill()
    
    // Example 5: SMTP client creation (without actual connection)
    vibez.spill("Creating SMTP clients...")
    
    // Basic SMTP client
    sus smtp_client SmtpClient = emailz.create_smtp_client("smtp.example.com", 25) fam {
        when err -> {
            vibez.spill("Error creating SMTP client:", err.message)
            damn cap
        }
    }
    vibez.spill("✅ Basic SMTP client created for", smtp_client.host, ":", smtp_client.port)
    
    // TLS SMTP client
    sus tls_client SmtpClient = emailz.create_smtp_client_tls("smtp.gmail.com", 465) fam {
        when err -> {
            vibez.spill("Error creating TLS client:", err.message)
            damn cap
        }
    }
    vibez.spill("✅ TLS SMTP client created for", tls_client.host, ":", tls_client.port)
    
    // STARTTLS SMTP client
    sus starttls_client SmtpClient = emailz.create_smtp_client_starttls("smtp.office365.com", 587) fam {
        when err -> {
            vibez.spill("Error creating STARTTLS client:", err.message)
            damn cap
        }
    }
    vibez.spill("✅ STARTTLS SMTP client created for", starttls_client.host, ":", starttls_client.port)
    vibez.spill()
    
    // Example 6: Quick send configuration (demonstration)
    vibez.spill("Email configuration example...")
    sus email_config EmailConfig = EmailConfig{
        smtp_host: "smtp.example.com",
        smtp_port: 587,
        username: "myemail@example.com",
        password: "mypassword",
        from_email: "noreply@example.com",
        from_name: "My Application",
        use_tls: cap,
        use_starttls: based,
        timeout: 30
    }
    
    vibez.spill("✅ Email configuration created")
    vibez.spill("SMTP Host:", email_config.smtp_host)
    vibez.spill("From:", email_config.from_name, "<" + email_config.from_email + ">")
    vibez.spill("Use STARTTLS:", email_config.use_starttls)
    vibez.spill()
    
    vibez.spill("=============================")
    vibez.spill("✅ All examples completed successfully!")
    vibez.spill()
    vibez.spill("Key EmailZ features demonstrated:")
    vibez.spill("• Simple text email creation")
    vibez.spill("• HTML email with attachments")
    vibez.spill("• Email address validation")
    vibez.spill("• Email formatting for sending")
    vibez.spill("• SMTP client configurations")
    vibez.spill("• Production-ready configuration")
    vibez.spill()
    vibez.spill("Ready to send emails with CURSED! 📧")
    
    damn based
}
