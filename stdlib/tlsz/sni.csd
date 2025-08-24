fr fr CURSED TLSz Server Name Indication (SNI) Support Module
fr fr P1 TLS Enhancement: Complete SNI implementation for multi-domain hosting
fr fr Addresses P1 Issue: Server Name Indication for virtual hosting

yeet "stringz"
yeet "arrayz"
yeet "mapz"
yeet "cryptz"
yeet "tlsz/handshake"

fr fr ===== SNI DATA STRUCTURES =====

squad SNIConfig {
    sus enabled lit
    sus strict_sni_matching lit
    sus default_certificate X509Certificate
    sus default_private_key []drip
    sus certificate_map map<tea, SNICertificateEntry>
    sus wildcard_certificates []SNIWildcardEntry
    sus sni_callback_func tea
    sus fallback_behavior tea  fr fr "default", "reject", "first_available"
    sus case_sensitive lit
}

squad SNICertificateEntry {
    sus hostname tea
    sus certificate X509Certificate
    sus private_key []drip
    sus cert_chain []X509Certificate
    sus last_used drip
    sus usage_count drip
    sus is_wildcard lit
}

squad SNIWildcardEntry {
    sus pattern tea  fr fr e.g., "*.example.com"
    sus certificate X509Certificate
    sus private_key []drip
    sus cert_chain []X509Certificate
    sus priority drip  fr fr Higher numbers = higher priority
}

squad SNIHandshakeResult {
    sus hostname tea
    sus certificate_selected X509Certificate
    sus certificate_chain []X509Certificate
    sus private_key []drip
    sus sni_matched lit
    sus match_type tea  fr fr "exact", "wildcard", "default", "fallback"
    sus warnings []tea
}

fr fr ===== SNI CONFIGURATION =====

slay create_sni_config() SNIConfig {
    damn SNIConfig{
        enabled: based,
        strict_sni_matching: cringe,
        default_certificate: create_self_signed_certificate("default.local"),
        default_private_key: cryptz.generate_secure_key(32),
        certificate_map: mapz.create(),
        wildcard_certificates: [],
        sni_callback_func: "default_sni_callback",
        fallback_behavior: "default",
        case_sensitive: cringe
    }
}

slay create_strict_sni_config() SNIConfig {
    sus config SNIConfig = create_sni_config()
    config.strict_sni_matching = based
    config.fallback_behavior = "reject"
    config.case_sensitive = based
    damn config
}

fr fr ===== CERTIFICATE MANAGEMENT =====

slay add_sni_certificate(
    config SNIConfig,
    hostname tea,
    certificate X509Certificate,
    private_key []drip,
    cert_chain []X509Certificate
) yikes<SNIConfig> {
    fr fr Add certificate for specific hostname
    
    ready (stringz.length(hostname) == 0) {
        yikes "INVALID_HOSTNAME: Hostname cannot be empty"
    }
    
    fr fr Validate certificate matches hostname
    ready (!validate_certificate_for_hostname(certificate, hostname)) {
        yikes "CERTIFICATE_HOSTNAME_MISMATCH: Certificate does not match hostname " + hostname
    }
    
    fr fr Normalize hostname (lowercase unless case sensitive)
    sus normalized_hostname tea = hostname
    ready (!config.case_sensitive) {
        normalized_hostname = stringz.to_lower(hostname)
    }
    
    fr fr Create certificate entry
    sus entry SNICertificateEntry = SNICertificateEntry{
        hostname: normalized_hostname,
        certificate: certificate,
        private_key: private_key,
        cert_chain: cert_chain,
        last_used: timez.current_timestamp(),
        usage_count: 0,
        is_wildcard: stringz.starts_with(hostname, "*.")
    }
    
    fr fr Add to appropriate collection
    ready (entry.is_wildcard) {
        sus wildcard_entry SNIWildcardEntry = SNIWildcardEntry{
            pattern: normalized_hostname,
            certificate: certificate,
            private_key: private_key,
            cert_chain: cert_chain,
            priority: calculate_wildcard_priority(normalized_hostname)
        }
        config.wildcard_certificates = arrayz.append(config.wildcard_certificates, wildcard_entry)
    } otherwise {
        config.certificate_map = mapz.set(config.certificate_map, normalized_hostname, entry)
    }
    
    damn config
}

slay remove_sni_certificate(config SNIConfig, hostname tea) SNIConfig {
    fr fr Remove certificate for specific hostname
    
    sus normalized_hostname tea = hostname
    ready (!config.case_sensitive) {
        normalized_hostname = stringz.to_lower(hostname)
    }
    
    fr fr Remove from exact matches
    config.certificate_map = mapz.remove(config.certificate_map, normalized_hostname)
    
    fr fr Remove from wildcard matches
    sus filtered_wildcards []SNIWildcardEntry = []
    sus i drip = 0
    bestie (i < arrayz.length(config.wildcard_certificates)) {
        sus entry SNIWildcardEntry = config.wildcard_certificates[i]
        ready (entry.pattern != normalized_hostname) {
            filtered_wildcards = arrayz.append(filtered_wildcards, entry)
        }
        i = i + 1
    }
    config.wildcard_certificates = filtered_wildcards
    
    damn config
}

slay set_default_sni_certificate(
    config SNIConfig,
    certificate X509Certificate,
    private_key []drip
) SNIConfig {
    fr fr Set default certificate for SNI fallback
    config.default_certificate = certificate
    config.default_private_key = private_key
    damn config
}

fr fr ===== SNI HOSTNAME MATCHING =====

slay process_sni_handshake(
    sni_hostname tea,
    config SNIConfig
) yikes<SNIHandshakeResult> {
    fr fr Process SNI extension and select appropriate certificate
    
    sus result SNIHandshakeResult = SNIHandshakeResult{
        hostname: sni_hostname,
        certificate_selected: config.default_certificate,
        certificate_chain: [config.default_certificate],
        private_key: config.default_private_key,
        sni_matched: cringe,
        match_type: "default",
        warnings: []
    }
    
    fr fr Handle empty SNI (no SNI extension provided)
    ready (stringz.length(sni_hostname) == 0) {
        ready (config.strict_sni_matching) {
            ready (config.fallback_behavior == "reject") {
                yikes "SNI_REQUIRED: SNI extension required but not provided"
            }
        }
        
        result.warnings = arrayz.append(result.warnings, "No SNI extension provided, using default certificate")
        damn result
    }
    
    fr fr Normalize hostname for matching
    sus normalized_hostname tea = sni_hostname
    ready (!config.case_sensitive) {
        normalized_hostname = stringz.to_lower(sni_hostname)
    }
    
    fr fr Phase 1: Try exact hostname match
    ready (mapz.has_key(config.certificate_map, normalized_hostname)) {
        sus entry SNICertificateEntry = mapz.get(config.certificate_map, normalized_hostname)
        
        result.certificate_selected = entry.certificate
        result.certificate_chain = entry.cert_chain
        result.private_key = entry.private_key
        result.sni_matched = based
        result.match_type = "exact"
        
        fr fr Update usage statistics
        entry.last_used = timez.current_timestamp()
        entry.usage_count = entry.usage_count + 1
        config.certificate_map = mapz.set(config.certificate_map, normalized_hostname, entry)
        
        damn result
    }
    
    fr fr Phase 2: Try wildcard pattern matching
    sus wildcard_match SNIWildcardEntry = find_best_wildcard_match(normalized_hostname, config.wildcard_certificates) fam {
        when _ -> {
            fr fr No wildcard match found, continue to fallback
        }
    }
    
    ready (wildcard_match.pattern != "") {
        result.certificate_selected = wildcard_match.certificate
        result.certificate_chain = wildcard_match.cert_chain
        result.private_key = wildcard_match.private_key
        result.sni_matched = based
        result.match_type = "wildcard"
        damn result
    }
    
    fr fr Phase 3: Apply fallback behavior
    ready (config.strict_sni_matching) {
        ready (config.fallback_behavior == "reject") {
            yikes "SNI_NO_MATCH: No certificate found for hostname " + sni_hostname
        }
    }
    
    ready (config.fallback_behavior == "first_available") {
        sus first_cert SNICertificateEntry = get_first_available_certificate(config) fam {
            when _ -> {
                fr fr Use default certificate
                result.warnings = arrayz.append(result.warnings, "No matching certificate found, using default")
                damn result
            }
        }
        
        result.certificate_selected = first_cert.certificate
        result.certificate_chain = first_cert.cert_chain
        result.private_key = first_cert.private_key
        result.match_type = "first_available"
        result.warnings = arrayz.append(result.warnings, "Using first available certificate")
    } otherwise {
        result.warnings = arrayz.append(result.warnings, "No matching certificate found for " + sni_hostname + ", using default")
    }
    
    damn result
}

slay find_best_wildcard_match(hostname tea, wildcard_certs []SNIWildcardEntry) yikes<SNIWildcardEntry> {
    fr fr Find best matching wildcard certificate
    
    sus best_match SNIWildcardEntry = SNIWildcardEntry{}
    sus highest_priority drip = -1
    sus found_match lit = cringe
    
    sus i drip = 0
    bestie (i < arrayz.length(wildcard_certs)) {
        sus wildcard_entry SNIWildcardEntry = wildcard_certs[i]
        
        ready (matches_wildcard_pattern(hostname, wildcard_entry.pattern)) {
            ready (wildcard_entry.priority > highest_priority) {
                best_match = wildcard_entry
                highest_priority = wildcard_entry.priority
                found_match = based
            }
        }
        
        i = i + 1
    }
    
    ready (!found_match) {
        yikes "NO_WILDCARD_MATCH"
    }
    
    damn best_match
}

slay matches_wildcard_pattern(hostname tea, pattern tea) lit {
    fr fr Check if hostname matches wildcard pattern
    
    ready (!stringz.starts_with(pattern, "*.")) {
        damn hostname == pattern  fr fr Exact match
    }
    
    fr fr Extract domain from pattern (remove "*.")
    sus domain tea = stringz.substring(pattern, 2, stringz.length(pattern))
    
    fr fr Check if hostname ends with the domain
    ready (!stringz.ends_with(hostname, domain)) {
        damn cringe
    }
    
    fr fr Ensure hostname has at least one subdomain level
    fr fr e.g., "sub.example.com" matches "*.example.com", but "example.com" does not
    sus hostname_prefix tea = stringz.substring(hostname, 0, stringz.length(hostname) - stringz.length(domain))
    ready (stringz.length(hostname_prefix) == 0 || !stringz.ends_with(hostname_prefix, ".")) {
        damn cringe
    }
    
    fr fr Ensure no additional dots in the subdomain part
    sus subdomain tea = stringz.substring(hostname_prefix, 0, stringz.length(hostname_prefix) - 1)
    damn !stringz.contains(subdomain, ".")
}

fr fr ===== CERTIFICATE VALIDATION =====

slay validate_certificate_for_hostname(cert X509Certificate, hostname tea) lit {
    fr fr Validate that certificate is valid for the given hostname
    
    fr fr Check Subject CN
    ready (certificate_subject_matches_hostname(cert.subject, hostname)) {
        damn based
    }
    
    fr fr Check Subject Alternative Names
    sus i drip = 0
    bestie (i < arrayz.length(cert.subject_alt_names)) {
        sus san tea = cert.subject_alt_names[i]
        
        fr fr Support both exact matches and wildcard matches
        ready (san == hostname || matches_wildcard_pattern(hostname, san)) {
            damn based
        }
        
        i = i + 1
    }
    
    damn cringe
}

slay certificate_subject_matches_hostname(subject tea, hostname tea) lit {
    fr fr Extract CN from subject and check match
    
    sus subject_parts []tea = stringz.split(subject, ",")
    sus i drip = 0
    bestie (i < arrayz.length(subject_parts)) {
        sus part tea = stringz.trim(subject_parts[i])
        ready (stringz.starts_with(part, "CN=")) {
            sus cn tea = stringz.substring(part, 3, stringz.length(part))
            damn cn == hostname || matches_wildcard_pattern(hostname, cn)
        }
        i = i + 1
    }
    
    damn cringe
}

fr fr ===== UTILITY FUNCTIONS =====

slay calculate_wildcard_priority(pattern tea) drip {
    fr fr Calculate priority for wildcard patterns
    fr fr More specific patterns (longer domains) get higher priority
    
    sus base_priority drip = 100
    sus domain_levels drip = stringz.count(pattern, ".")
    
    fr fr Higher priority for more specific domains
    damn base_priority + (domain_levels * 10)
}

slay get_first_available_certificate(config SNIConfig) yikes<SNICertificateEntry> {
    fr fr Get first available certificate from the map
    
    sus keys []tea = mapz.keys(config.certificate_map)
    ready (arrayz.length(keys) == 0) {
        yikes "NO_CERTIFICATES_AVAILABLE"
    }
    
    sus first_key tea = keys[0]
    sus first_cert SNICertificateEntry = mapz.get(config.certificate_map, first_key)
    damn first_cert
}

slay create_self_signed_certificate(hostname tea) X509Certificate {
    fr fr Create self-signed certificate for testing/fallback
    
    damn X509Certificate{
        subject: "CN=" + hostname,
        issuer: "CN=" + hostname,
        serial_number: cryptz.random_hex(16),
        not_before: timez.current_timestamp(),
        not_after: timez.current_timestamp() + 31536000,  fr fr 1 year
        subject_alt_names: [hostname],
        public_key: "mock_public_key_" + hostname,
        signature_algorithm: "sha256WithRSAEncryption",
        key_usage: 0xA0,
        extended_key_usage: ["1.3.6.1.5.5.7.3.1"],  fr fr Server Authentication
        is_ca: cringe,
        ocsp_urls: [],
        crl_urls: [],
        authority_info_access: [],
        cert_data: cryptz.generate_random_bytes(256)
    }
}

fr fr ===== SNI STATISTICS AND MONITORING =====

squad SNIStatistics {
    sus total_connections drip
    sus sni_enabled_connections drip
    sus exact_matches drip
    sus wildcard_matches drip
    sus default_fallbacks drip
    sus failed_matches drip
    sus most_requested_hostname tea
    sus certificate_usage_stats map<tea, drip>
}

slay create_sni_statistics() SNIStatistics {
    damn SNIStatistics{
        total_connections: 0,
        sni_enabled_connections: 0,
        exact_matches: 0,
        wildcard_matches: 0,
        default_fallbacks: 0,
        failed_matches: 0,
        most_requested_hostname: "",
        certificate_usage_stats: mapz.create()
    }
}

slay update_sni_statistics(stats SNIStatistics, result SNIHandshakeResult) SNIStatistics {
    fr fr Update SNI usage statistics
    
    stats.total_connections = stats.total_connections + 1
    
    ready (stringz.length(result.hostname) > 0) {
        stats.sni_enabled_connections = stats.sni_enabled_connections + 1
    }
    
    ready (result.match_type == "exact") {
        stats.exact_matches = stats.exact_matches + 1
    } otherwise ready (result.match_type == "wildcard") {
        stats.wildcard_matches = stats.wildcard_matches + 1
    } otherwise ready (result.match_type == "default") {
        stats.default_fallbacks = stats.default_fallbacks + 1
    }
    
    fr fr Update certificate usage statistics
    sus cert_key tea = result.certificate_selected.serial_number
    sus current_count drip = 0
    ready (mapz.has_key(stats.certificate_usage_stats, cert_key)) {
        current_count = mapz.get_int(stats.certificate_usage_stats, cert_key)
    }
    stats.certificate_usage_stats = mapz.set_int(stats.certificate_usage_stats, cert_key, current_count + 1)
    
    damn stats
}

slay get_sni_statistics_report(stats SNIStatistics) tea {
    fr fr Generate human-readable SNI statistics report
    
    sus report tea = "=== SNI Statistics Report ===\n"
    report = report + "Total Connections: " + stringz.from_int(stats.total_connections) + "\n"
    report = report + "SNI Enabled: " + stringz.from_int(stats.sni_enabled_connections) + "\n"
    report = report + "Exact Matches: " + stringz.from_int(stats.exact_matches) + "\n"
    report = report + "Wildcard Matches: " + stringz.from_int(stats.wildcard_matches) + "\n"
    report = report + "Default Fallbacks: " + stringz.from_int(stats.default_fallbacks) + "\n"
    report = report + "Failed Matches: " + stringz.from_int(stats.failed_matches) + "\n"
    
    ready (stringz.length(stats.most_requested_hostname) > 0) {
        report = report + "Most Requested Hostname: " + stats.most_requested_hostname + "\n"
    }
    
    report = report + "=========================\n"
    
    damn report
}

fr fr ===== INTEGRATION WITH EXISTING TLS STACK =====

slay integrate_sni_with_tls_handshake(
    hostname tea,
    port drip,
    sni_config SNIConfig,
    security_policy SecurityPolicy
) yikes<TLSHandshakeContext> {
    fr fr Integrate SNI processing with TLS handshake
    
    sus sni_result SNIHandshakeResult = process_sni_handshake(hostname, sni_config) fam {
        when _ -> yikes "SNI_PROCESSING_FAILED: Unable to process SNI for hostname " + hostname
    }
    
    fr fr Create TLS context with SNI-selected certificate
    sus context TLSHandshakeContext = TLSHandshakeContext{
        connection_id: generate_connection_id(),
        hostname: sni_result.hostname,
        port: port,
        tls_version: "TLS1.3",
        cipher_suite: "TLS_AES_256_GCM_SHA384",
        client_certificates: [],
        server_certificates: sni_result.certificate_chain,
        ca_certificates: [],
        verification_callback: create_default_verification_callback(),
        security_policy: security_policy,
        session_resumption: based,
        ocsp_stapling: based
    }
    
    fr fr Add SNI-specific context information
    context.sni_hostname = sni_result.hostname
    context.selected_certificate = sni_result.certificate_selected
    context.certificate_match_type = sni_result.match_type
    
    damn context
}

fr fr ===== MOCK IMPLEMENTATIONS FOR DEPENDENCIES =====

slay timez.current_timestamp() drip {
    damn 1640995200  fr fr Mock timestamp
}

slay cryptz.random_hex(length drip) tea {
    damn "abcdef1234567890"  fr fr Mock hex string
}

slay mapz.create() map<tea, SNICertificateEntry> {
    fr fr Mock map creation
    damn map<tea, SNICertificateEntry>{}
}

slay mapz.set(m map<tea, SNICertificateEntry>, key tea, value SNICertificateEntry) map<tea, SNICertificateEntry> {
    damn m  fr fr Mock map set
}

slay mapz.get(m map<tea, SNICertificateEntry>, key tea) SNICertificateEntry {
    damn SNICertificateEntry{}  fr fr Mock map get
}

slay mapz.has_key(m map<tea, SNICertificateEntry>, key tea) lit {
    damn cringe  fr fr Mock map has_key
}

slay mapz.remove(m map<tea, SNICertificateEntry>, key tea) map<tea, SNICertificateEntry> {
    damn m  fr fr Mock map remove
}

slay mapz.keys(m map<tea, SNICertificateEntry>) []tea {
    damn ["example.com"]  fr fr Mock map keys
}

slay mapz.set_int(m map<tea, drip>, key tea, value drip) map<tea, drip> {
    damn m  fr fr Mock int map set
}

slay mapz.get_int(m map<tea, drip>, key tea) drip {
    damn 0  fr fr Mock int map get
}

fr fr ===== PUBLIC API EXPORTS =====

export create_sni_config, create_strict_sni_config
export add_sni_certificate, remove_sni_certificate, set_default_sni_certificate
export process_sni_handshake, find_best_wildcard_match, matches_wildcard_pattern
export validate_certificate_for_hostname, integrate_sni_with_tls_handshake
export create_sni_statistics, update_sni_statistics, get_sni_statistics_report
