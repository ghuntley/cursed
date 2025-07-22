yeet "asn1_mood"
yeet "pem_drip"
yeet "crypto"
yeet "string"

fr fr X.509 Certificate Structure
struct X509Cert {
    subject tea
    issuer tea
    serial_number tea
    not_before tea
    not_after tea
    public_key tea
    extensions tea
    signature tea
}

fr fr X.509 Private Key Structure
struct X509Key {
    algorithm tea
    key_data tea
    public_key tea
}

fr fr X.509 Public Key Structure
struct X509PubKey {
    algorithm tea
    key_data tea
    parameters tea
}

fr fr X.509 Certificate Request Structure
struct X509CSR {
    subject tea
    public_key tea
    extensions tea
    signature tea
}

fr fr Parse X.509 certificate from PEM/DER data
slay x509_parse_cert(data tea) X509Cert {
    sus pem_data tea = pem_drip.decode(data)
    sus asn1_data tea = asn1_mood.parse(pem_data)
    
    sus cert X509Cert = X509Cert{
        subject: asn1_mood.get_subject(asn1_data),
        issuer: asn1_mood.get_issuer(asn1_data),
        serial_number: asn1_mood.get_serial(asn1_data),
        not_before: asn1_mood.get_not_before(asn1_data),
        not_after: asn1_mood.get_not_after(asn1_data),
        public_key: asn1_mood.get_public_key(asn1_data),
        extensions: asn1_mood.get_extensions(asn1_data),
        signature: asn1_mood.get_signature(asn1_data)
    }
    
    damn cert
}

fr fr Parse X.509 private key from PEM/DER data
slay x509_parse_key(data tea) X509Key {
    sus pem_data tea = pem_drip.decode(data)
    sus asn1_data tea = asn1_mood.parse(pem_data)
    
    sus key X509Key = X509Key{
        algorithm: asn1_mood.get_algorithm(asn1_data),
        key_data: asn1_mood.get_key_data(asn1_data),
        public_key: asn1_mood.get_public_key(asn1_data)
    }
    
    damn key
}

fr fr Parse X.509 public key from PEM/DER data
slay x509_parse_pubkey(data tea) X509PubKey {
    sus pem_data tea = pem_drip.decode(data)
    sus asn1_data tea = asn1_mood.parse(pem_data)
    
    sus pubkey X509PubKey = X509PubKey{
        algorithm: asn1_mood.get_algorithm(asn1_data),
        key_data: asn1_mood.get_key_data(asn1_data),
        parameters: asn1_mood.get_parameters(asn1_data)
    }
    
    damn pubkey
}

fr fr Parse X.509 certificate request from PEM/DER data
slay x509_parse_csr(data tea) X509CSR {
    sus pem_data tea = pem_drip.decode(data)
    sus asn1_data tea = asn1_mood.parse(pem_data)
    
    sus csr X509CSR = X509CSR{
        subject: asn1_mood.get_subject(asn1_data),
        public_key: asn1_mood.get_public_key(asn1_data),
        extensions: asn1_mood.get_extensions(asn1_data),
        signature: asn1_mood.get_signature(asn1_data)
    }
    
    damn csr
}

fr fr Encode X.509 certificate to PEM format
slay x509_encode_cert(cert X509Cert) tea {
    sus asn1_data tea = asn1_mood.encode_cert(cert)
    sus pem_data tea = pem_drip.encode(asn1_data, "CERTIFICATE")
    damn pem_data
}

fr fr Encode X.509 private key to PEM format
slay x509_encode_key(key X509Key) tea {
    sus asn1_data tea = asn1_mood.encode_key(key)
    sus pem_data tea = pem_drip.encode(asn1_data, "PRIVATE KEY")
    damn pem_data
}

fr fr Encode X.509 public key to PEM format
slay x509_encode_pubkey(pubkey X509PubKey) tea {
    sus asn1_data tea = asn1_mood.encode_pubkey(pubkey)
    sus pem_data tea = pem_drip.encode(asn1_data, "PUBLIC KEY")
    damn pem_data
}

fr fr Verify X.509 certificate against CA certificate
slay x509_verify_cert(cert X509Cert, ca X509Cert) lit {
    sus cert_signature tea = cert.signature
    sus ca_pubkey tea = ca.public_key
    
    sus is_valid lit = crypto.verify_signature(cert_signature, ca_pubkey, cert.subject) fr fr Check validity period
    sus now tea = time.now()
    sus not_before_valid lit = time.after(now, cert.not_before)
    sus not_after_valid lit = time.before(now, cert.not_after)
    
    damn is_valid && not_before_valid && not_after_valid
}

fr fr Verify X.509 certificate chain
slay x509_verify_chain(certs []X509Cert) lit {
    sus chain_length normie = len(certs)
    
    bestie i := 0; i < chain_length - 1; i++ {
        sus cert X509Cert = certs[i]
        sus ca X509Cert = certs[i + 1]
        
        sus is_valid lit = x509_verify_cert(cert, ca)
        mood !is_valid {
            damn cap
        }
    }
    
    damn based
}

fr fr Get subject name from X.509 certificate
slay x509_get_subject(cert X509Cert) tea {
    damn cert.subject
}

fr fr Get issuer name from X.509 certificate
slay x509_get_issuer(cert X509Cert) tea {
    damn cert.issuer
}

fr fr Get serial number from X.509 certificate
slay x509_get_serial(cert X509Cert) tea {
    damn cert.serial_number
}

fr fr Get validity period from X.509 certificate
slay x509_get_validity(cert X509Cert) (tea, tea) {
    damn (cert.not_before, cert.not_after)
}

fr fr Get extensions from X.509 certificate
slay x509_get_extensions(cert X509Cert) tea {
    damn cert.extensions
}

fr fr Check hostname against certificate Subject Alternative Names
slay x509_check_hostname(cert X509Cert, hostname tea) lit {
    sus extensions tea = cert.extensions
    sus san_list tea = asn1_mood.get_san_dns(extensions)
    
    sus subject_cn tea = asn1_mood.get_common_name(cert.subject) fr fr Check Common Name
    mood string.equals(subject_cn, hostname) {
        damn based
    } fr fr Check Subject Alternative Names
    sus hostnames []tea = string.split(san_list, ",")
    bestie i := 0; i < len(hostnames); i++ {
        sus san_hostname tea = string.trim(hostnames[i])
        mood string.equals(san_hostname, hostname) {
            damn based
        } fr fr Check wildcard matching
        mood string.starts_with(san_hostname, "*.") {
            sus wildcard_domain tea = string.substring(san_hostname, 2)
            mood string.ends_with(hostname, wildcard_domain) {
                damn based
            }
        }
    }
    
    damn cap
}

fr fr Check email against certificate Subject Alternative Names
slay x509_check_email(cert X509Cert, email tea) lit {
    sus extensions tea = cert.extensions
    sus san_list tea = asn1_mood.get_san_email(extensions)
    
    sus subject_email tea = asn1_mood.get_email_address(cert.subject) fr fr Check Subject email
    mood string.equals(subject_email, email) {
        damn based
    } fr fr Check Subject Alternative Names
    sus emails []tea = string.split(san_list, ",")
    bestie i := 0; i < len(emails); i++ {
        sus san_email tea = string.trim(emails[i])
        mood string.equals(san_email, email) {
            damn based
        }
    }
    
    damn cap
}

fr fr Check IP address against certificate Subject Alternative Names
slay x509_check_ip(cert X509Cert, ip tea) lit {
    sus extensions tea = cert.extensions
    sus san_list tea = asn1_mood.get_san_ip(extensions)
    
    sus ips []tea = string.split(san_list, ",")
    bestie i := 0; i < len(ips); i++ {
        sus san_ip tea = string.trim(ips[i])
        mood string.equals(san_ip, ip) {
            damn based
        }
    }
    
    damn cap
}
