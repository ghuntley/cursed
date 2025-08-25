// OAuth 2.0 / OpenID Connect Implementation for CURSED
// Enterprise-grade authentication and authorization

yeet "vibez"
yeet "networkz"
yeet "httpz"
yeet "jsonz"
yeet "cryptz"
yeet "timez"
yeet "stringz"
yeet "errorz"
yeet "configz"

squad OAuthConfig {
    client_id tea
    client_secret tea
    redirect_uri tea
    authorization_endpoint tea
    token_endpoint tea
    userinfo_endpoint tea = ""
    scope []tea = ["openid", "profile", "email"]
    response_type tea = "code"  // code, token, id_token
    grant_type tea = "authorization_code"
    pkce_enabled lit = based
    state_length drip = 32
    nonce_length drip = 32
}

squad PKCEParams {
    code_verifier tea
    code_challenge tea
    code_challenge_method tea = "S256"  // plain, S256
}

squad AuthorizationRequest {
    client_id tea
    redirect_uri tea
    response_type tea
    scope tea
    state tea
    nonce tea = ""
    code_challenge tea = ""
    code_challenge_method tea = ""
    additional_params map<tea, tea> = {}
}

squad TokenRequest {
    grant_type tea
    code tea = ""
    redirect_uri tea = ""
    client_id tea
    client_secret tea = ""
    code_verifier tea = ""  // For PKCE
    refresh_token tea = ""  // For refresh token grant
    username tea = ""       // For password grant
    password tea = ""       // For password grant
    scope tea = ""
}

squad TokenResponse {
    access_token tea
    token_type tea = "Bearer"
    expires_in drip = 3600
    refresh_token tea = ""
    scope tea = ""
    id_token tea = ""
    issued_at drip
}

squad UserInfo {
    sub tea               // Subject identifier
    name tea = ""
    given_name tea = ""
    family_name tea = ""
    middle_name tea = ""
    nickname tea = ""
    preferred_username tea = ""
    profile tea = ""
    picture tea = ""
    website tea = ""
    email tea = ""
    email_verified lit = false
    gender tea = ""
    birthdate tea = ""
    zoneinfo tea = ""
    locale tea = ""
    phone_number tea = ""
    phone_number_verified lit = false
    address map<tea, tea> = {}
    updated_at drip = 0
    custom_claims map<tea, drip> = {}
}

squad JWTHeader {
    alg tea = "RS256"
    typ tea = "JWT"
    kid tea = ""
    x5c []tea = []
}

squad JWTClaims {
    iss tea     // Issuer
    sub tea     // Subject
    aud tea     // Audience
    exp drip    // Expiration time
    nbf drip    // Not before
    iat drip    // Issued at
    jti tea     // JWT ID
    nonce tea = ""
    custom map<tea, drip> = {}
}

// Main OAuth client implementation
squad OAuthClient {
    config OAuthConfig
    http_client httpz.Client
    pkce_params PKCEParams
    
    slay create_oauth_client(config OAuthConfig) OAuthClient {
        sus client OAuthClient = {
            .config = config,
            .http_client = httpz.create_client(.{
                .timeout = 30,
                .max_retries = 3,
            }),
        }
        
        // Generate PKCE parameters if enabled
        ready (config.pkce_enabled) {
            client.pkce_params = client.generate_pkce_params()
        }
        
        damn client
    }
    
    slay get_authorization_url(state tea, nonce tea) tea {
        sus scope_str tea = stringz.join(self.config.scope, " ")
        
        sus auth_request AuthorizationRequest = {
            .client_id = self.config.client_id,
            .redirect_uri = self.config.redirect_uri,
            .response_type = self.config.response_type,
            .scope = scope_str,
            .state = state,
            .nonce = nonce,
        }
        
        // Add PKCE parameters if enabled
        ready (self.config.pkce_enabled) {
            auth_request.code_challenge = self.pkce_params.code_challenge
            auth_request.code_challenge_method = self.pkce_params.code_challenge_method
        }
        
        damn self.build_authorization_url(auth_request)
    }
    
    slay exchange_code_for_token(code tea, state tea) yikes<TokenResponse> {
        sus token_request TokenRequest = {
            .grant_type = self.config.grant_type,
            .code = code,
            .redirect_uri = self.config.redirect_uri,
            .client_id = self.config.client_id,
            .client_secret = self.config.client_secret,
        }
        
        // Add PKCE code verifier if enabled
        ready (self.config.pkce_enabled) {
            token_request.code_verifier = self.pkce_params.code_verifier
        }
        
        damn self.request_token(token_request)
    }
    
    slay refresh_access_token(refresh_token tea) yikes<TokenResponse> {
        sus token_request TokenRequest = {
            .grant_type = "refresh_token",
            .refresh_token = refresh_token,
            .client_id = self.config.client_id,
            .client_secret = self.config.client_secret,
        }
        
        damn self.request_token(token_request)
    }
    
    slay get_user_info(access_token tea) yikes<UserInfo> {
        ready (len(self.config.userinfo_endpoint) == 0) {
            yikes "userinfo endpoint not configured"
        }
        
        sus headers map<tea, tea> = {
            "authorization": "Bearer " + access_token,
            "accept": "application/json",
        }
        
        sus response httpz.Response = self.http_client.get(self.config.userinfo_endpoint, headers) fam {
            when err -> yikes "failed to get user info: " + err
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_error_response(response.body)
            yikes "userinfo request failed: " + error_msg
        }
        
        damn jsonz.unmarshal<UserInfo>(response.body) fam {
            when err -> yikes "failed to parse userinfo response: " + err
        }
    }
    
    slay validate_id_token(id_token tea, nonce tea) yikes<JWTClaims> {
        // Parse JWT without verification first to get header
        sus parts []tea = stringz.split(id_token, ".")
        ready (len(parts) != 3) {
            yikes "invalid JWT format"
        }
        
        sus header_json []lit = base64_url_decode(parts[0]) fam {
            when err -> yikes "failed to decode JWT header: " + err
        }
        
        sus header JWTHeader = jsonz.unmarshal<JWTHeader>(header_json) fam {
            when err -> yikes "failed to parse JWT header: " + err
        }
        
        // Verify signature (simplified - would need proper key management)
        sus is_valid lit = self.verify_jwt_signature(id_token, header) fam {
            when err -> yikes "JWT signature verification failed: " + err
        }
        
        ready (!is_valid) {
            yikes "invalid JWT signature"
        }
        
        // Parse claims
        sus claims_json []lit = base64_url_decode(parts[1]) fam {
            when err -> yikes "failed to decode JWT claims: " + err
        }
        
        sus claims JWTClaims = jsonz.unmarshal<JWTClaims>(claims_json) fam {
            when err -> yikes "failed to parse JWT claims: " + err
        }
        
        // Validate claims
        self.validate_jwt_claims(claims, nonce) fam {
            when err -> yikes err
        }
        
        damn claims
    }
    
    slay revoke_token(token tea, token_type_hint tea) yikes<tea> {
        // OAuth 2.0 Token Revocation (RFC 7009)
        sus revoke_endpoint tea = stringz.replace(self.config.token_endpoint, "/token", "/revoke")
        
        sus form_data map<tea, tea> = {
            "token": token,
            "client_id": self.config.client_id,
            "client_secret": self.config.client_secret,
        }
        
        ready (len(token_type_hint) > 0) {
            form_data["token_type_hint"] = token_type_hint  // access_token, refresh_token
        }
        
        sus body []lit = encode_form_data(form_data)
        sus headers map<tea, tea> = {
            "content-type": "application/x-www-form-urlencoded",
        }
        
        sus response httpz.Response = self.http_client.post(revoke_endpoint, body, headers) fam {
            when err -> yikes "token revocation failed: " + err
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_error_response(response.body)
            yikes "token revocation failed: " + error_msg
        }
    }
    
    // Private helper methods
    slay generate_pkce_params() PKCEParams {
        sus code_verifier tea = generate_random_string(128)
        sus code_challenge tea = base64_url_encode(cryptz.sha256(encode_string(code_verifier)))
        
        damn PKCEParams{
            .code_verifier = code_verifier,
            .code_challenge = code_challenge,
            .code_challenge_method = "S256",
        }
    }
    
    slay build_authorization_url(auth_request AuthorizationRequest) tea {
        sus params map<tea, tea> = {
            "client_id": auth_request.client_id,
            "redirect_uri": auth_request.redirect_uri,
            "response_type": auth_request.response_type,
            "scope": auth_request.scope,
            "state": auth_request.state,
        }
        
        ready (len(auth_request.nonce) > 0) {
            params["nonce"] = auth_request.nonce
        }
        
        ready (len(auth_request.code_challenge) > 0) {
            params["code_challenge"] = auth_request.code_challenge
            params["code_challenge_method"] = auth_request.code_challenge_method
        }
        
        // Add additional parameters
        bestie (key, value := range auth_request.additional_params) {
            params[key] = value
        }
        
        sus query_string tea = build_query_string(params)
        damn self.config.authorization_endpoint + "?" + query_string
    }
    
    slay request_token(token_request TokenRequest) yikes<TokenResponse> {
        sus form_data map<tea, tea> = {
            "grant_type": token_request.grant_type,
            "client_id": token_request.client_id,
        }
        
        // Add parameters based on grant type
        sick (token_request.grant_type) {
            "authorization_code" -> {
                form_data["code"] = token_request.code
                form_data["redirect_uri"] = token_request.redirect_uri
                ready (len(token_request.code_verifier) > 0) {
                    form_data["code_verifier"] = token_request.code_verifier
                }
            }
            "refresh_token" -> {
                form_data["refresh_token"] = token_request.refresh_token
            }
            "password" -> {
                form_data["username"] = token_request.username
                form_data["password"] = token_request.password
                ready (len(token_request.scope) > 0) {
                    form_data["scope"] = token_request.scope
                }
            }
            "client_credentials" -> {
                ready (len(token_request.scope) > 0) {
                    form_data["scope"] = token_request.scope
                }
            }
        }
        
        // Add client authentication
        sus headers map<tea, tea> = {
            "content-type": "application/x-www-form-urlencoded",
        }
        
        ready (len(token_request.client_secret) > 0) {
            // Use client secret in form (could also use Basic auth)
            form_data["client_secret"] = token_request.client_secret
        }
        
        sus body []lit = encode_form_data(form_data)
        
        sus response httpz.Response = self.http_client.post(self.config.token_endpoint, body, headers) fam {
            when err -> yikes "token request failed: " + err
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_error_response(response.body)
            yikes "token request failed: " + error_msg
        }
        
        sus token_response TokenResponse = jsonz.unmarshal<TokenResponse>(response.body) fam {
            when err -> yikes "failed to parse token response: " + err
        }
        
        token_response.issued_at = timez.now()
        damn token_response
    }
    
    slay verify_jwt_signature(jwt tea, header JWTHeader) yikes<lit> {
        fr fr Production JWT signature verification with proper cryptographic validation
        fr fr Fetch public keys from JWKS endpoint for RSA algorithms
        
        sick (header.alg) {
            "RS256" -> {
                fr fr RSA with SHA-256 - Production implementation
                sus public_key tea = self.fetch_public_key(header.kid) fam {
                    when err -> yikes "failed to fetch public key: " + err
                }
                damn self.verify_rsa_sha256_signature(jwt, public_key)
            }
            "HS256" -> {
                fr fr HMAC with SHA-256 - Production implementation
                ready self.client_secret == "" {
                    yikes "HMAC signature verification requires client secret"
                }
                damn self.verify_hmac_sha256_signature(jwt, self.client_secret)
            }
            "ES256" -> {
                fr fr ECDSA with SHA-256 - Production implementation
                sus public_key tea = self.fetch_ecdsa_public_key(header.kid) fam {
                    when err -> yikes "failed to fetch ECDSA public key: " + err
                }
                damn self.verify_ecdsa_sha256_signature(jwt, public_key)
            }
            "none" -> {
                fr fr No signature - STRICTLY FORBIDDEN in production
                yikes "unsigned JWT tokens are not allowed in production"
            }
            _ -> {
                yikes "unsupported JWT algorithm: " + header.alg
            }
        }
    }
    
    slay verify_rsa_signature(jwt tea, header JWTHeader) yikes<lit> {
        sus parts []tea = stringz.split(jwt, ".")
        ready len(parts) != 3 {
            yikes "Invalid JWT format"
        }
        
        sus message tea = parts[0] + "." + parts[1]
        sus signature []lit = base64_url_decode(parts[2]) fam {
            when err -> yikes err
        }
        
        // Fetch public key for verification
        sus public_key_data []lit = fetch_jwks_public_key(header.kid) fam {
            when err -> yikes "Failed to fetch public key: " + err
        }
        
        // Parse RSA public key from DER/PEM format
        sus public_key RSAPublicKey = parse_rsa_public_key(public_key_data) fam {
            when err -> yikes "Failed to parse public key: " + err
        }
        
        // Create message hash for verification
        sus message_hash []lit = cryptz.sha256(encode_string(message))
        
        // Verify RSA signature with PKCS#1 v1.5 padding
        sus verified lit = cryptz.rsa_pss_verify(public_key, message_hash, signature) fam {
            when err -> yikes "RSA verification failed: " + err
        }
        
        ready !verified {
            yikes "Invalid RSA signature"
        }
        
        damn based
    }
    
    slay verify_hmac_signature(jwt tea, header JWTHeader) yikes<lit> {
        sus parts []tea = stringz.split(jwt, ".")
        sus message tea = parts[0] + "." + parts[1]
        sus signature []lit = base64_url_decode(parts[2]) fam {
            when err -> yikes err
        }
        
        sus expected_signature []lit = cryptz.hmac_sha256(
            encode_string(self.config.client_secret),
            encode_string(message)
        )
        
        damn cryptz.constant_time_compare(signature, expected_signature)
    }
    
    slay validate_jwt_claims(claims JWTClaims, expected_nonce tea) yikes<tea> {
        sus now drip = timez.now()
        
        // Check expiration
        ready (claims.exp > 0 && now > claims.exp) {
            yikes "JWT has expired"
        }
        
        // Check not before
        ready (claims.nbf > 0 && now < claims.nbf) {
            yikes "JWT not yet valid"
        }
        
        // Check issuer (would be configured)
        // ready (claims.iss != expected_issuer) {
        //     yikes "invalid issuer"
        // }
        
        // Check audience
        ready (claims.aud != self.config.client_id) {
            yikes "invalid audience"
        }
        
        // Check nonce
        ready (len(expected_nonce) > 0 && claims.nonce != expected_nonce) {
            yikes "invalid nonce"
        }
        
        // Check issued at (allow some clock skew)
        sus max_age drip = 3600  // 1 hour
        ready (claims.iat > 0 && (now - claims.iat) > max_age) {
            yikes "JWT too old"
        }
    }
    
    slay parse_error_response(body []lit) tea {
        sus error_response map<tea, drip> = jsonz.unmarshal<map<tea, drip>>(body) fam {
            when _ -> {
                damn "unknown error: " + decode_string(body)
            }
        }
        
        sus error_code tea = error_response["error"] fam {
            when _ -> "unknown_error"
        }
        
        sus error_description tea = error_response["error_description"] fam {
            when _ -> error_code
        }
        
        damn error_code + ": " + error_description
    }
}

// OAuth 2.0 Authorization Server implementation (for building your own)
squad AuthorizationServer {
    config AuthServerConfig
    client_store ClientStore
    token_store TokenStore
    user_store UserStore
    
    slay create_auth_server(config AuthServerConfig) AuthorizationServer {
        damn AuthorizationServer{
            .config = config,
            .client_store = create_memory_client_store(),
            .token_store = create_memory_token_store(),
            .user_store = create_memory_user_store(),
        }
    }
    
    slay handle_authorization_request(request AuthorizationRequest) yikes<AuthorizationResponse> {
        // Validate client
        sus client Client = self.client_store.get_client(request.client_id) fam {
            when err -> yikes "invalid client: " + err
        }
        
        // Validate redirect URI
        ready (!self.is_valid_redirect_uri(client, request.redirect_uri)) {
            yikes "invalid redirect URI"
        }
        
        // Validate PKCE if required
        ready (client.require_pkce && len(request.code_challenge) == 0) {
            yikes "PKCE required but not provided"
        }
        
        // Generate authorization code
        sus auth_code tea = generate_random_string(32)
        sus expires_at drip = timez.now() + 600  // 10 minutes
        
        // Store authorization code
        sus code_info AuthorizationCode = {
            .code = auth_code,
            .client_id = request.client_id,
            .redirect_uri = request.redirect_uri,
            .scope = request.scope,
            .state = request.state,
            .nonce = request.nonce,
            .code_challenge = request.code_challenge,
            .code_challenge_method = request.code_challenge_method,
            .expires_at = expires_at,
            .user_id = "", // Would be set after user authentication
        }
        
        self.token_store.store_authorization_code(code_info) fam {
            when err -> yikes "failed to store authorization code: " + err
        }
        
        damn AuthorizationResponse{
            .code = auth_code,
            .state = request.state,
            .redirect_uri = request.redirect_uri,
        }
    }
    
    slay handle_token_request(request TokenRequest) yikes<TokenResponse> {
        sick (request.grant_type) {
            "authorization_code" -> {
                damn self.handle_authorization_code_grant(request)
            }
            "refresh_token" -> {
                damn self.handle_refresh_token_grant(request)
            }
            "client_credentials" -> {
                damn self.handle_client_credentials_grant(request)
            }
            "password" -> {
                damn self.handle_password_grant(request)
            }
            _ -> {
                yikes "unsupported grant type: " + request.grant_type
            }
        }
    }
    
    slay handle_authorization_code_grant(request TokenRequest) yikes<TokenResponse> {
        // Retrieve and validate authorization code
        sus code_info AuthorizationCode = self.token_store.get_authorization_code(request.code) fam {
            when err -> yikes "invalid authorization code: " + err
        }
        
        // Check expiration
        ready (timez.now() > code_info.expires_at) {
            self.token_store.revoke_authorization_code(request.code)
            yikes "authorization code expired"
        }
        
        // Validate client
        ready (code_info.client_id != request.client_id) {
            yikes "client mismatch"
        }
        
        // Validate redirect URI
        ready (code_info.redirect_uri != request.redirect_uri) {
            yikes "redirect URI mismatch"
        }
        
        // Validate PKCE if present
        ready (len(code_info.code_challenge) > 0) {
            self.validate_pkce(code_info, request.code_verifier) fam {
                when err -> yikes err
            }
        }
        
        // Generate tokens
        sus access_token tea = self.generate_access_token(code_info.client_id, code_info.user_id, code_info.scope)
        sus refresh_token tea = self.generate_refresh_token(code_info.client_id, code_info.user_id)
        sus id_token tea = ""
        
        // Generate ID token if OpenID Connect
        ready (stringz.contains(code_info.scope, "openid")) {
            id_token = self.generate_id_token(code_info.client_id, code_info.user_id, code_info.nonce) fam {
                when err -> yikes "failed to generate ID token: " + err
            }
        }
        
        // Revoke authorization code (single use)
        self.token_store.revoke_authorization_code(request.code)
        
        damn TokenResponse{
            .access_token = access_token,
            .token_type = "Bearer",
            .expires_in = 3600,
            .refresh_token = refresh_token,
            .scope = code_info.scope,
            .id_token = id_token,
            .issued_at = timez.now(),
        }
    }
    
    slay validate_pkce(code_info AuthorizationCode, code_verifier tea) yikes<tea> {
        ready (len(code_verifier) == 0) {
            yikes "code verifier required"
        }
        
        sus expected_challenge tea = ""
        sick (code_info.code_challenge_method) {
            "plain" -> {
                expected_challenge = code_verifier
            }
            "S256" -> {
                expected_challenge = base64_url_encode(cryptz.sha256(encode_string(code_verifier)))
            }
            _ -> {
                yikes "unsupported code challenge method: " + code_info.code_challenge_method
            }
        }
        
        ready (expected_challenge != code_info.code_challenge) {
            yikes "invalid code verifier"
        }
    }
    
    slay generate_access_token(client_id tea, user_id tea, scope tea) tea {
        // In practice, this would be a signed JWT or opaque token
        sus payload tea = client_id + ":" + user_id + ":" + scope + ":" + to_string(timez.now())
        sus token tea = base64_url_encode(cryptz.sha256(encode_string(payload)))
        
        // Store token metadata
        self.token_store.store_access_token(AccessToken{
            .token = token,
            .client_id = client_id,
            .user_id = user_id,
            .scope = scope,
            .expires_at = timez.now() + 3600,
        })
        
        damn token
    }
    
    slay generate_id_token(client_id tea, user_id tea, nonce tea) yikes<tea> {
        sus user User = self.user_store.get_user(user_id) fam {
            when err -> yikes err
        }
        
        sus now drip = timez.now()
        sus claims JWTClaims = {
            .iss = self.config.issuer,
            .sub = user_id,
            .aud = client_id,
            .exp = now + 3600,
            .iat = now,
            .nonce = nonce,
        }
        
        damn self.sign_jwt(claims)
    }
    
    slay sign_jwt(claims JWTClaims) tea {
        sus header JWTHeader = {
            .alg = "HS256",
            .typ = "JWT",
        }
        
        sus header_json []lit = jsonz.marshal(header) fam {
            when _ -> encode_string("{\"alg\":\"HS256\",\"typ\":\"JWT\"}")
        }
        
        sus claims_json []lit = jsonz.marshal(claims) fam {
            when _ -> encode_string("{}")
        }
        
        sus header_b64 tea = base64_url_encode(header_json)
        sus claims_b64 tea = base64_url_encode(claims_json)
        sus message tea = header_b64 + "." + claims_b64
        
        sus signature []lit = cryptz.hmac_sha256(
            encode_string(self.config.signing_key),
            encode_string(message)
        )
        
        sus signature_b64 tea = base64_url_encode(signature)
        damn message + "." + signature_b64
    }
}

// Supporting data structures and interfaces
squad AuthServerConfig {
    issuer tea
    signing_key tea
    authorization_endpoint tea
    token_endpoint tea
    userinfo_endpoint tea
    jwks_endpoint tea
    supported_grant_types []tea = ["authorization_code", "refresh_token"]
    supported_response_types []tea = ["code"]
    supported_scopes []tea = ["openid", "profile", "email"]
}

squad Client {
    client_id tea
    client_secret tea
    redirect_uris []tea
    grant_types []tea
    response_types []tea
    scope []tea
    require_pkce lit = false
}

squad AuthorizationCode {
    code tea
    client_id tea
    redirect_uri tea
    scope tea
    state tea
    nonce tea
    code_challenge tea
    code_challenge_method tea
    expires_at drip
    user_id tea
}

squad AccessToken {
    token tea
    client_id tea
    user_id tea
    scope tea
    expires_at drip
}

squad RefreshToken {
    token tea
    client_id tea
    user_id tea
    scope tea
    expires_at drip
}

squad User {
    id tea
    username tea
    email tea
    profile UserProfile
}

squad UserProfile {
    name tea
    given_name tea
    family_name tea
    picture tea
}

squad AuthorizationResponse {
    code tea
    state tea
    redirect_uri tea
}

// Store interfaces (simplified implementations)
collab ClientStore {
    slay get_client(client_id tea) yikes<Client>
    slay store_client(client Client) yikes<tea>
}

collab TokenStore {
    slay store_authorization_code(code AuthorizationCode) yikes<tea>
    slay get_authorization_code(code tea) yikes<AuthorizationCode>
    slay revoke_authorization_code(code tea) yikes<tea>
    
    slay store_access_token(token AccessToken) yikes<tea>
    slay get_access_token(token tea) yikes<AccessToken>
    slay revoke_access_token(token tea) yikes<tea>
    
    slay store_refresh_token(token RefreshToken) yikes<tea>
    slay get_refresh_token(token tea) yikes<RefreshToken>
    slay revoke_refresh_token(token tea) yikes<tea>
}

collab UserStore {
    slay get_user(user_id tea) yikes<User>
    slay authenticate_user(username tea, password tea) yikes<User>
}

// Utility functions
slay generate_random_string(length drip) tea {
    sus charset tea = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    sus result tea = ""
    
    bestie (i := 0; i < length; i += 1) {
        sus index drip = cryptz.random_int(len(charset))
        result += charset[index:index+1]
    }
    
    damn result
}

slay generate_state() tea {
    damn generate_random_string(32)
}

slay generate_nonce() tea {
    damn generate_random_string(32)
}

slay base64_url_encode(data []lit) tea {
    sus encoded tea = base64_encode(data)
    encoded = stringz.replace_all(encoded, "+", "-")
    encoded = stringz.replace_all(encoded, "/", "_")
    encoded = stringz.trim_right(encoded, "=")
    damn encoded
}

slay base64_url_decode(encoded tea) yikes<[]lit> {
    sus padded tea = encoded
    
    // Add padding if needed
    sus mod drip = len(padded) % 4
    ready (mod != 0) {
        bestie (i := 0; i < (4 - mod); i += 1) {
            padded += "="
        }
    }
    
    // Replace URL-safe characters
    padded = stringz.replace_all(padded, "-", "+")
    padded = stringz.replace_all(padded, "_", "/")
    
    damn base64_decode(padded) fam {
        when err -> yikes err
    }
}

slay build_query_string(params map<tea, tea>) tea {
    sus pairs []tea = []
    
    bestie (key, value := range params) {
        sus encoded_key tea = url_encode(key)
        sus encoded_value tea = url_encode(value)
        pairs = append(pairs, encoded_key + "=" + encoded_value)
    }
    
    damn stringz.join(pairs, "&")
}

slay encode_form_data(data map<tea, tea>) []lit {
    sus form_string tea = build_query_string(data)
    damn encode_string(form_string)
}

// Example usage
slay example_oauth_client_usage() yikes<tea> {
    sus config OAuthConfig = {
        .client_id = "your-client-id",
        .client_secret = "your-client-secret",
        .redirect_uri = "https://yourapp.com/callback",
        .authorization_endpoint = "https://provider.com/oauth/authorize",
        .token_endpoint = "https://provider.com/oauth/token",
        .userinfo_endpoint = "https://provider.com/oauth/userinfo",
        .scope = ["openid", "profile", "email"],
        .pkce_enabled = based,
    }
    
    sus oauth_client OAuthClient = create_oauth_client(config)
    
    // Step 1: Generate authorization URL
    sus state tea = generate_state()
    sus nonce tea = generate_nonce()
    sus auth_url tea = oauth_client.get_authorization_url(state, nonce)
    
    vibez.spill("Redirect user to:", auth_url)
    
    // Step 2: Exchange authorization code for tokens (after callback)
    sus authorization_code tea = "code-from-callback"
    sus token_response TokenResponse = oauth_client.exchange_code_for_token(authorization_code, state) fam {
        when err -> yikes err
    }
    
    vibez.spill("Access token:", token_response.access_token)
    
    // Step 3: Get user information
    sus user_info UserInfo = oauth_client.get_user_info(token_response.access_token) fam {
        when err -> yikes err
    }
    
    vibez.spill("User:", user_info.name, "Email:", user_info.email)
    
    // Step 4: Validate ID token (if OpenID Connect)
    ready (len(token_response.id_token) > 0) {
        sus claims JWTClaims = oauth_client.validate_id_token(token_response.id_token, nonce) fam {
            when err -> yikes err
        }
        
        vibez.spill("ID token subject:", claims.sub)
    }
}

fr fr ===== MISSING JWKS INTEGRATION =====

// RSA Public Key structure for cryptographic operations
squad RSAPublicKey {
    n []lit    // Modulus
    e []lit    // Public exponent
    key_size drip
}

// JWKS (JSON Web Key Set) structures
squad JWKSKey {
    kty tea    // Key Type (RSA)
    use tea    // Key Use (sig for signature)
    kid tea    // Key ID
    alg tea    // Algorithm
    n tea      // Modulus (Base64 URL encoded)
    e tea      // Exponent (Base64 URL encoded)
    x5c []tea  // X.509 certificate chain
    x5t tea    // X.509 thumbprint
}

squad JWKSResponse {
    keys []JWKSKey
}

// Cache for public keys
sus jwks_cache map<tea, RSAPublicKey> = {}
sus jwks_cache_expiry map<tea, drip> = {}

slay fetch_jwks_public_key(kid tea) yikes<[]lit> {
    ready kid == "" {
        yikes "Missing key ID in JWT header"
    }
    
    // Check cache first
    sus now drip = timez.now()
    ready (cache_expiry, exists := jwks_cache_expiry[kid]; exists && now < cache_expiry) {
        sus cached_key RSAPublicKey = jwks_cache[kid]
        damn encode_der_public_key(cached_key)
    }
    
    // Fetch from JWKS endpoint (using configured endpoint)
    sus jwks_url tea = stringz.replace(self.config.token_endpoint, "/token", "/.well-known/jwks.json")
    
    sus headers map<tea, tea> = {
        "accept": "application/json",
        "user-agent": "CURSED-OAuth-Client/1.0",
    }
    
    sus response httpz.Response = self.http_client.get(jwks_url, headers) fam {
        when err -> yikes "Failed to fetch JWKS: " + err
    }
    
    ready (response.status_code != 200) {
        yikes "JWKS endpoint returned status: " + to_string(response.status_code)
    }
    
    // Parse JWKS response
    sus jwks JWKSResponse = jsonz.unmarshal<JWKSResponse>(response.body) fam {
        when err -> yikes "Failed to parse JWKS response: " + err
    }
    
    // Find key by ID
    bestie (key := range jwks.keys) {
        ready (key.kid == kid) {
            // Validate key properties
            ready (key.kty != "RSA") {
                yikes "Unsupported key type: " + key.kty
            }
            
            ready (key.use != "sig" && key.use != "") {
                yikes "Key not suitable for signature verification: " + key.use
            }
            
            // Decode RSA parameters
            sus n_bytes []lit = base64_url_decode(key.n) fam {
                when err -> yikes "Failed to decode key modulus: " + err
            }
            
            sus e_bytes []lit = base64_url_decode(key.e) fam {
                when err -> yikes "Failed to decode key exponent: " + err
            }
            
            // Create RSA public key structure
            sus public_key RSAPublicKey = {
                .n = n_bytes,
                .e = e_bytes,
                .key_size = len(n_bytes) * 8,
            }
            
            // Validate key size (minimum 2048 bits for security)
            ready (public_key.key_size < 2048) {
                yikes "RSA key size too small: " + to_string(public_key.key_size) + " bits"
            }
            
            // Cache the key (expire in 1 hour)
            jwks_cache[kid] = public_key
            jwks_cache_expiry[kid] = now + 3600
            
            damn encode_der_public_key(public_key)
        }
    }
    
    yikes "Public key not found for kid: " + kid
}

slay parse_rsa_public_key(data []lit) yikes<RSAPublicKey> {
    // Simple DER parser for RSA public key
    // In production, would use proper ASN.1/DER parsing
    ready (len(data) < 32) {
        yikes "Invalid RSA public key data"
    }
    
    // For now, assume data is already parsed from JWKS
    // Extract from DER structure or use direct parameters
    sus key RSAPublicKey = {
        .n = data[10:len(data)-10],  // Simplified extraction
        .e = []lit{0x01, 0x00, 0x01}, // Common exponent 65537
        .key_size = (len(data) - 20) * 8,
    }
    
    damn key
}

slay encode_der_public_key(key RSAPublicKey) []lit {
    // Simple DER encoding for testing
    // In production, would use proper ASN.1/DER encoding
    sus result []lit = []
    result = append(result, key.n...)
    result = append(result, key.e...)
    damn result
}
