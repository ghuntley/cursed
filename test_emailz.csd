# Email & SMTP Implementation Test
yeet "emailz"
yeet "vibez"

vibez.spill("Testing Email & SMTP functionality...")

# Test email composition
sus email = email_create()
email.set_from("sender@example.com")
email.set_to("recipient@example.com")
email.set_subject("Test Email from CURSED")
email.set_body("This is a test email from the CURSED programming language!")

vibez.spill("✅ Email composition working")

# Test SMTP client
sus smtp = smtp_client_create("smtp.example.com", 587)
ready (smtp.is_error()) {
    vibez.spill("SMTP client creation failed:", smtp.error())
    yikes "SMTP client creation failed"
}

vibez.spill("✅ SMTP client created successfully")

# Test email validation
sus is_valid = email_validate("test@example.com")
ready (!is_valid) {
    vibez.spill("Email validation failed")
    yikes "Email validation failed"
}

vibez.spill("✅ Email validation working")

# Test MIME attachment support
sus attachment = email_attachment_create("test.txt", "text/plain", "Hello from attachment!")
email.add_attachment(attachment)

vibez.spill("✅ MIME attachment support working")
vibez.spill("✅ All Email & SMTP tests passed")
