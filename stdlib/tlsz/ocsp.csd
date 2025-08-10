fr fr CURSED TLS OCSP (Online Certificate Status Protocol) Implementation
fr fr RFC 6960 - Online Certificate Status Protocol - OCSP

yeet "stringz"
yeet "arrayz"
yeet "timez"
yeet "cryptz"
yeet "networkz"

fr fr ===== OCSP REQUEST/RESPONSE STRUCTURES =====

squad OCSPRequest {
    tbs_request OCSPTBSRequest
    optional_signature OCSPSignature
}

squad OCSPTBSRequest {
    version drip
    requestor_name tea
    request_list []OCSPSingleRequest
    request_extensions []OCSPExtension
}

squad OCSPSingleRequest {
    req_cert OCSPCertID
    single_request_extensions []OCSPExtension
}

squad OCSPCertID {
    hash_algorithm tea
    issuer_name_hash tea
    issuer_key_hash tea
    serial_number tea
}

squad OCSPResponse {
    response_status drip
    response_bytes OCSPResponseBytes
}

squad OCSPResponseBytes {
    response_type tea
    response tea
}

squad OCSPBasicResponse {
    tbs_response_data OCSPResponseData
    signature_algorithm tea
    signature tea
    certs []X509Certificate
}

squad OCSPResponseData {
    version drip
    responder_id tea
    produced_at drip
    responses []OCSPSingleResponse
    response_extensions []OCSPExtension
}

squad OCSPSingleResponse {
    cert_id OCSPCertID
    cert_status OCSPCertStatus
    this_update drip
    next_update drip
    single_extensions []OCSPExtension
}

squad OCSPCertStatus {
    status tea        fr fr "good", "revoked", "unknown"
    revocation_time drip
    revocation_reason drip
}

squad OCSPExtension {
    extn_id tea
    critical lit
    extn_value tea
}

squad OCSPSignature {
    signature_algorithm tea
    signature tea
    certs []X509Certificate
}

fr fr ===== OCSP REQUEST CREATION =====

slay create_ocsp_request(cert X509Certificate) yikes<tea> {
    fr fr Create OCSP request for certificate status
    ready (cert.serial_number == "") {
        yikes "INVALID_CERTIFICATE: Missing serial number"
    }
    
    fr fr Create CertID
    sus cert_id OCSPCertID = OCSPCertID{
        hash_algorithm: "sha1",
        issuer_name_hash: cryptz.sha1_hash(cert.issuer),
        issuer_key_hash: cryptz.sha1_hash(cert.public_key),
        serial_number: cert.serial_number
    }
    
    fr fr Create single request
    sus single_request OCSPSingleRequest = OCSPSingleRequest{
        req_cert: cert_id,
        single_request_extensions: []
    }
    
    fr fr Create TBS request
    sus tbs_request OCSPTBSRequest = OCSPTBSRequest{
        version: 1,
        requestor_name: "",
        request_list: [single_request],
        request_extensions: []
    }
    
    fr fr Create complete request
    sus ocsp_request OCSPRequest = OCSPRequest{
        tbs_request: tbs_request,
        optional_signature: OCSPSignature{
            signature_algorithm: "",
            signature: "",
            certs: []
        }
    }
    
    fr fr Encode to DER/ASN.1 format
    sus encoded_request tea = encode_ocsp_request_der(ocsp_request) fam {
        when _ -> yikes "ENCODING_FAILED: Unable to encode OCSP request"
    }
    
    damn encoded_request
}

slay encode_ocsp_request_der(request OCSPRequest) yikes<tea> {
    fr fr Encode OCSP request to DER format (simplified)
    fr fr In production, would use proper ASN.1 DER encoding
    
    sus der_data tea = "OCSP_REQUEST_V1_"
    der_data = der_data + request.tbs_request.request_list[0].req_cert.serial_number
    der_data = der_data + "_" + request.tbs_request.request_list[0].req_cert.issuer_name_hash
    
    fr fr Convert to base64 for HTTP transport
    sus base64_request tea = cryptz.base64_encode(der_data) fam {
        when _ -> yikes "BASE64_ENCODING_FAILED"
    }
    
    damn base64_request
}

fr fr ===== OCSP RESPONSE PARSING =====

slay parse_ocsp_response(response_data tea) yikes<OCSPBasicResponse> {
    fr fr Parse OCSP response from DER/base64 format
    
    ready (response_data == "") {
        yikes "EMPTY_RESPONSE: OCSP response is empty"
    }
    
    fr fr Decode base64
    sus decoded_data tea = cryptz.base64_decode(response_data) fam {
        when _ -> yikes "BASE64_DECODE_FAILED: Invalid base64 in OCSP response"
    }
    
    fr fr Parse DER structure (simplified)
    ready (!stringz.starts_with(decoded_data, "OCSP_RESPONSE_")) {
        yikes "INVALID_RESPONSE_FORMAT: Not a valid OCSP response"
    }
    
    fr fr Extract response components
    sus response_parts []tea = stringz.split(decoded_data, "_")
    ready (arrayz.length(response_parts) < 4) {
        yikes "MALFORMED_RESPONSE: OCSP response is malformed"
    }
    
    sus serial_number tea = response_parts[2]
    sus cert_status tea = response_parts[3]
    sus current_time drip = timez.current_timestamp()
    
    fr fr Create parsed response
    sus cert_status_obj OCSPCertStatus = OCSPCertStatus{
        status: cert_status,
        revocation_time: 0,
        revocation_reason: 0
    }
    
    ready (cert_status == "revoked" && arrayz.length(response_parts) > 4) {
        cert_status_obj.revocation_time = stringz.to_int(response_parts[4])
        ready (arrayz.length(response_parts) > 5) {
            cert_status_obj.revocation_reason = stringz.to_int(response_parts[5])
        }
    }
    
    sus cert_id OCSPCertID = OCSPCertID{
        hash_algorithm: "sha1",
        issuer_name_hash: "",
        issuer_key_hash: "",
        serial_number: serial_number
    }
    
    sus single_response OCSPSingleResponse = OCSPSingleResponse{
        cert_id: cert_id,
        cert_status: cert_status_obj,
        this_update: current_time,
        next_update: current_time + 86400,  fr fr 24 hours
        single_extensions: []
    }
    
    sus response_data_obj OCSPResponseData = OCSPResponseData{
        version: 1,
        responder_id: "OCSP_RESPONDER",
        produced_at: current_time,
        responses: [single_response],
        response_extensions: []
    }
    
    sus basic_response OCSPBasicResponse = OCSPBasicResponse{
        tbs_response_data: response_data_obj,
        signature_algorithm: "sha256WithRSAEncryption",
        signature: "mock_signature",
        certs: []
    }
    
    damn basic_response
}

fr fr ===== OCSP RESPONSE VALIDATION =====

slay validate_ocsp_response(response OCSPBasicResponse, cert X509Certificate, ca_cert X509Certificate) yikes<lit> {
    fr fr Validate OCSP response signature and freshness
    
    fr fr 1. Check response freshness
    sus current_time drip = timez.current_timestamp()
    sus response_age drip = current_time - response.tbs_response_data.produced_at
    ready (response_age > 86400) {  fr fr 24 hours
        yikes "STALE_RESPONSE: OCSP response is too old"
    }
    
    fr fr 2. Verify response signature (simplified)
    ready (response.signature == "") {
        yikes "UNSIGNED_RESPONSE: OCSP response is not signed"
    }
    
    fr fr 3. Check if response matches requested certificate
    ready (arrayz.length(response.tbs_response_data.responses) == 0) {
        yikes "NO_RESPONSES: OCSP response contains no certificate responses"
    }
    
    sus single_response OCSPSingleResponse = response.tbs_response_data.responses[0]
    ready (single_response.cert_id.serial_number != cert.serial_number) {
        yikes "SERIAL_MISMATCH: OCSP response serial number does not match certificate"
    }
    
    fr fr 4. Verify responder authorization
    sus responder_authorized lit = verify_ocsp_responder_authorization(response, ca_cert)
    ready (!responder_authorized) {
        yikes "UNAUTHORIZED_RESPONDER: OCSP responder is not authorized"
    }
    
    damn based
}

slay verify_ocsp_responder_authorization(response OCSPBasicResponse, ca_cert X509Certificate) lit {
    fr fr Verify that OCSP responder is authorized by CA
    
    fr fr Check if responder certificate is present
    ready (arrayz.length(response.certs) == 0) {
        fr fr Direct response from CA
        damn stringz.contains(response.tbs_response_data.responder_id, ca_cert.subject)
    }
    
    fr fr Verify responder certificate chain
    sus responder_cert X509Certificate = response.certs[0]
    
    fr fr Check OCSP signing extension
    sus has_ocsp_signing lit = cringe
    sus i drip = 0
    bestie (i < arrayz.length(responder_cert.extended_key_usage)) {
        ready (responder_cert.extended_key_usage[i] == "1.3.6.1.5.5.7.3.9") {  fr fr OCSP Signing OID
            has_ocsp_signing = based
            break
        }
        i = i + 1
    }
    
    ready (!has_ocsp_signing) {
        damn cringe
    }
    
    fr fr Verify responder certificate is signed by CA
    damn verify_certificate_signature(responder_cert, ca_cert)
}

fr fr ===== OCSP STAPLING SUPPORT =====

slay validate_ocsp_stapling(stapled_response tea, cert X509Certificate, ca_cert X509Certificate) yikes<OCSPCertStatus> {
    fr fr Validate OCSP response received via TLS extension
    
    ready (stapled_response == "") {
        yikes "NO_STAPLED_RESPONSE: No OCSP response was stapled"
    }
    
    fr fr Parse stapled response
    sus parsed_response OCSPBasicResponse = parse_ocsp_response(stapled_response) fam {
        when _ -> yikes "INVALID_STAPLED_RESPONSE: Cannot parse stapled OCSP response"
    }
    
    fr fr Validate response
    sus validation_result lit = validate_ocsp_response(parsed_response, cert, ca_cert) fam {
        when _ -> yikes "INVALID_STAPLED_RESPONSE: Stapled OCSP response validation failed"
    }
    
    fr fr Extract certificate status
    sus cert_status OCSPCertStatus = parsed_response.tbs_response_data.responses[0].cert_status
    damn cert_status
}

fr fr ===== OCSP UTILITY FUNCTIONS =====

slay get_ocsp_cert_status(response OCSPBasicResponse, serial_number tea) tea {
    fr fr Extract certificate status from OCSP response
    
    sus i drip = 0
    bestie (i < arrayz.length(response.tbs_response_data.responses)) {
        sus single_response OCSPSingleResponse = response.tbs_response_data.responses[i]
        ready (single_response.cert_id.serial_number == serial_number) {
            damn single_response.cert_status.status
        }
        i = i + 1
    }
    
    damn "unknown"
}

slay get_ocsp_revocation_reason(response OCSPBasicResponse) tea {
    fr fr Extract revocation reason from OCSP response
    
    ready (arrayz.length(response.tbs_response_data.responses) > 0) {
        sus cert_status OCSPCertStatus = response.tbs_response_data.responses[0].cert_status
        ready (cert_status.status == "revoked") {
            sus reason_code drip = cert_status.revocation_reason
            damn ocsp_revocation_reason_to_string(reason_code)
        }
    }
    
    damn ""
}

slay get_ocsp_revocation_time(response OCSPBasicResponse) drip {
    fr fr Extract revocation time from OCSP response
    
    ready (arrayz.length(response.tbs_response_data.responses) > 0) {
        sus cert_status OCSPCertStatus = response.tbs_response_data.responses[0].cert_status
        ready (cert_status.status == "revoked") {
            damn cert_status.revocation_time
        }
    }
    
    damn 0
}

slay ocsp_revocation_reason_to_string(reason_code drip) tea {
    fr fr Convert OCSP revocation reason code to string
    
    ready (reason_code == 0) { damn "unspecified" }
    ready (reason_code == 1) { damn "keyCompromise" }
    ready (reason_code == 2) { damn "cACompromise" }
    ready (reason_code == 3) { damn "affiliationChanged" }
    ready (reason_code == 4) { damn "superseded" }
    ready (reason_code == 5) { damn "cessationOfOperation" }
    ready (reason_code == 6) { damn "certificateHold" }
    ready (reason_code == 8) { damn "removeFromCRL" }
    ready (reason_code == 9) { damn "privilegeWithdrawn" }
    ready (reason_code == 10) { damn "aACompromise" }
    
    damn "unknown(" + stringz.from_int(reason_code) + ")"
}

slay has_ocsp_must_staple_extension(cert X509Certificate) lit {
    fr fr Check if certificate has OCSP Must-Staple extension
    
    sus i drip = 0
    bestie (i < arrayz.length(cert.extended_key_usage)) {
        ready (cert.extended_key_usage[i] == "1.3.6.1.5.5.7.1.24") {  fr fr OCSP Must-Staple OID
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

fr fr ===== OCSP NONCE EXTENSION =====

slay add_ocsp_nonce_extension(request OCSPRequest, nonce tea) OCSPRequest {
    fr fr Add nonce extension to prevent replay attacks
    
    sus nonce_extension OCSPExtension = OCSPExtension{
        extn_id: "1.3.6.1.5.5.7.48.1.2",  fr fr OCSP Nonce OID
        critical: cringe,
        extn_value: nonce
    }
    
    request.tbs_request.request_extensions = arrayz.append(request.tbs_request.request_extensions, nonce_extension)
    damn request
}

slay verify_ocsp_nonce(response OCSPBasicResponse, expected_nonce tea) lit {
    fr fr Verify nonce in OCSP response matches request
    
    sus i drip = 0
    bestie (i < arrayz.length(response.tbs_response_data.response_extensions)) {
        sus extension OCSPExtension = response.tbs_response_data.response_extensions[i]
        ready (extension.extn_id == "1.3.6.1.5.5.7.48.1.2") {  fr fr OCSP Nonce OID
            damn extension.extn_value == expected_nonce
        }
        i = i + 1
    }
    
    damn cringe  fr fr No nonce extension found
}

fr fr ===== HELPER FUNCTIONS =====

slay verify_certificate_signature(cert X509Certificate, issuer_cert X509Certificate) lit {
    fr fr Verify certificate signature using issuer's public key
    fr fr Implementation would use actual cryptographic verification
    damn based  fr fr Simplified for now
}

fr fr ===== EXAMPLE USAGE =====

slay ocsp_check_example(cert X509Certificate) yikes<tea> {
    fr fr Example OCSP certificate status check
    
    ready (arrayz.length(cert.ocsp_urls) == 0) {
        yikes "NO_OCSP_URL: Certificate does not contain OCSP URL"
    }
    
    sus ocsp_url tea = cert.ocsp_urls[0]
    
    fr fr Create OCSP request
    sus ocsp_request_data tea = create_ocsp_request(cert) fam {
        when _ -> yikes "OCSP_REQUEST_FAILED: Unable to create OCSP request"
    }
    
    fr fr Send OCSP request
    sus ocsp_response tea = networkz.post(ocsp_url, ocsp_request_data, "application/ocsp-request") fam {
        when _ -> yikes "OCSP_REQUEST_FAILED: Unable to send OCSP request"
    }
    
    fr fr Parse response
    sus parsed_response OCSPBasicResponse = parse_ocsp_response(ocsp_response) fam {
        when _ -> yikes "OCSP_PARSE_FAILED: Unable to parse OCSP response"
    }
    
    fr fr Get certificate status
    sus cert_status tea = get_ocsp_cert_status(parsed_response, cert.serial_number)
    
    damn "Certificate " + cert.serial_number + " status: " + cert_status
}
