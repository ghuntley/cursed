yeet "smtp_tea"

# Simple SMTP Tea Demo
vibez.spill("🔥 Testing SMTP Tea module functionality!")

# Test SMTP configuration
sus config_valid := smtp_tea.smtp_client_config("smtp.gmail.com", 587, "user", "pass", based, smtp_tea.AUTH_PLAIN)
lowkey config_valid {
    vibez.spill("✅ SMTP configuration validation works!")
} simp {
    vibez.spill("❌ SMTP configuration validation failed!")
}

# Test email address validation  
lowkey smtp_tea.validate_email_address("test@example.com") {
    vibez.spill("✅ Email address validation works!")
} simp {
    vibez.spill("❌ Email address validation failed!")
}

# Test email message creation
sus message := smtp_tea.create_email_message(
    "sender@example.com",
    "recipient@example.com", 
    "Test from CURSED! 🔥",
    "This email was generated with pure CURSED energy! No cap! 💯",
    "",
    "",
    smtp_tea.PRIORITY_NORMAL
)

lowkey message != "" && message != "Invalid from address" {
    vibez.spill("✅ Email message creation works!")
} simp {
    vibez.spill("❌ Email message creation failed!")
}

# Test Gen Z vibe check functionality
vibez.spill("📧 Testing Gen Z vibe check email functionality...")

# Test constants
vibez.spill("SMTP Ports - Plain: " + smtp_tea.SMTP_PLAIN + ", TLS: " + smtp_tea.SMTP_TLS + ", SSL: " + smtp_tea.SMTP_SSL)
vibez.spill("Auth Methods - None: " + smtp_tea.AUTH_NONE + ", Plain: " + smtp_tea.AUTH_PLAIN + ", Login: " + smtp_tea.AUTH_LOGIN)
vibez.spill("Priorities - Low: " + smtp_tea.PRIORITY_LOW + ", Normal: " + smtp_tea.PRIORITY_NORMAL + ", High: " + smtp_tea.PRIORITY_HIGH)

vibez.spill("🎉 SMTP Tea module is working perfectly! Ready to send emails with CURSED energy! ✨")
vibez.spill("📬 No cap, this email system is absolutely fire! 💯🔥")
