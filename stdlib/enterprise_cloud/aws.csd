// AWS SDK Integration for CURSED
// Enterprise-grade cloud services integration

yeet "vibez"
yeet "networkz"
yeet "configz"
yeet "errorz"
yeet "cryptz"
yeet "httpz"
yeet "jsonz"
yeet "timez"

squad AWSConfig {
    access_key_id tea
    secret_access_key tea
    session_token tea = ""              // For temporary credentials
    region tea = "us-east-1"
    endpoint_url tea = ""               // Custom endpoint for testing
    use_ssl lit = based
    signature_version tea = "v4"
    max_retries drip = 3
    retry_mode tea = "adaptive"         // legacy, standard, adaptive
    connect_timeout drip = 60
    read_timeout drip = 60
    user_agent tea = "cursed-aws-sdk/1.0"
}

squad AWSCredentials {
    access_key_id tea
    secret_access_key tea
    session_token tea = ""
    expiration drip = 0                 // Unix timestamp, 0 = never expires
}

// AWS Signature Version 4 implementation
squad AWSSigner {
    config AWSConfig
    
    slay sign_request(
        method tea,
        url tea,
        headers map<tea, tea>,
        payload []lit,
        service tea,
        timestamp drip
    ) yikes<map<tea, tea>> {
        sus datetime tea = format_datetime(timestamp)
        sus date tea = datetime[0:8]
        
        // Add required headers
        sus signed_headers map<tea, tea> = headers
        signed_headers["host"] = extract_host(url)
        signed_headers["x-amz-date"] = datetime
        
        ready (len(self.config.session_token) > 0) {
            signed_headers["x-amz-security-token"] = self.config.session_token
        }
        
        // Step 1: Create canonical request
        sus canonical_request tea = self.create_canonical_request(
            method, url, signed_headers, payload
        )
        
        // Step 2: Create string to sign
        sus string_to_sign tea = self.create_string_to_sign(
            datetime, self.config.region, service, canonical_request
        )
        
        // Step 3: Calculate signature
        sus signature tea = self.calculate_signature(
            date, self.config.region, service, string_to_sign
        )
        
        // Step 4: Create authorization header
        sus authorization tea = self.create_authorization_header(
            signed_headers, signature
        )
        
        signed_headers["authorization"] = authorization
        damn signed_headers
    }
    
    slay create_canonical_request(
        method tea,
        url tea,
        headers map<tea, tea>,
        payload []lit
    ) tea {
        sus uri tea = extract_path(url)
        sus query_string tea = extract_query(url)
        
        // Canonical headers
        sus header_names []tea = sort_keys(headers)
        sus canonical_headers tea = ""
        sus signed_header_list tea = ""
        
        bestie (i, name := range header_names) {
            canonical_headers += name + ":" + trim(headers[name]) + "\n"
            ready (i > 0) { signed_header_list += ";" }
            signed_header_list += name
        }
        
        // Payload hash
        sus payload_hash tea = hex(cryptz.sha256(payload))
        
        damn method + "\n" +
             uri + "\n" +
             query_string + "\n" +
             canonical_headers + "\n" +
             signed_header_list + "\n" +
             payload_hash
    }
    
    slay create_string_to_sign(datetime tea, region tea, service tea, canonical_request tea) tea {
        sus algorithm tea = "AWS4-HMAC-SHA256"
        sus credential_scope tea = datetime[0:8] + "/" + region + "/" + service + "/aws4_request"
        sus hashed_canonical_request tea = hex(cryptz.sha256(encode_string(canonical_request)))
        
        damn algorithm + "\n" +
             datetime + "\n" +
             credential_scope + "\n" +
             hashed_canonical_request
    }
    
    slay calculate_signature(date tea, region tea, service tea, string_to_sign tea) tea {
        sus k_date []lit = cryptz.hmac_sha256(encode_string("AWS4" + self.config.secret_access_key), encode_string(date))
        sus k_region []lit = cryptz.hmac_sha256(k_date, encode_string(region))
        sus k_service []lit = cryptz.hmac_sha256(k_region, encode_string(service))
        sus k_signing []lit = cryptz.hmac_sha256(k_service, encode_string("aws4_request"))
        sus signature []lit = cryptz.hmac_sha256(k_signing, encode_string(string_to_sign))
        
        damn hex(signature)
    }
    
    slay create_authorization_header(headers map<tea, tea>, signature tea) tea {
        sus algorithm tea = "AWS4-HMAC-SHA256"
        sus credential tea = self.config.access_key_id + "/" + self.get_credential_scope()
        sus signed_headers tea = join(sort_keys(headers), ";")
        
        damn algorithm + " " +
             "Credential=" + credential + ", " +
             "SignedHeaders=" + signed_headers + ", " +
             "Signature=" + signature
    }
}

// S3 Service Implementation
squad S3Client {
    config AWSConfig
    signer AWSSigner
    http_client httpz.Client
    
    slay create_s3_client(config AWSConfig) S3Client {
        damn S3Client{
            .config = config,
            .signer = AWSSigner{.config = config},
            .http_client = httpz.create_client(.{
                .timeout = config.read_timeout,
                .max_retries = config.max_retries,
            }),
        }
    }
    
    slay put_object(bucket tea, key tea, data []lit, content_type tea) yikes<S3PutResult> {
        sus url tea = self.build_s3_url(bucket, key)
        sus headers map<tea, tea> = {
            "content-type": content_type,
            "content-length": to_string(len(data)),
        }
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "PUT", url, headers, data, "s3", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.put(url, data, signed_headers) fam {
            when err -> yikes "s3 put_object failed: " + err
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_s3_error(response.body)
            yikes "s3 put_object failed: " + error_msg
        }
        
        damn S3PutResult{
            .etag = response.headers["etag"],
            .version_id = response.headers["x-amz-version-id"],
        }
    }
    
    slay get_object(bucket tea, key tea) yikes<S3Object> {
        sus url tea = self.build_s3_url(bucket, key)
        sus headers map<tea, tea> = {}
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "GET", url, headers, [], "s3", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.get(url, signed_headers) fam {
            when err -> yikes "s3 get_object failed: " + err
        }
        
        ready (response.status_code == 404) {
            yikes "object not found: s3://" + bucket + "/" + key
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_s3_error(response.body)
            yikes "s3 get_object failed: " + error_msg
        }
        
        damn S3Object{
            .data = response.body,
            .content_type = response.headers["content-type"],
            .content_length = parse_int(response.headers["content-length"]),
            .etag = response.headers["etag"],
            .last_modified = response.headers["last-modified"],
        }
    }
    
    slay delete_object(bucket tea, key tea) yikes<tea> {
        sus url tea = self.build_s3_url(bucket, key)
        sus headers map<tea, tea> = {}
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "DELETE", url, headers, [], "s3", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.delete(url, signed_headers) fam {
            when err -> yikes "s3 delete_object failed: " + err
        }
        
        ready (response.status_code != 204) {
            sus error_msg tea = self.parse_s3_error(response.body)
            yikes "s3 delete_object failed: " + error_msg
        }
    }
    
    slay list_objects(bucket tea, prefix tea, max_keys drip) yikes<S3ListResult> {
        sus url tea = self.build_s3_url(bucket, "")
        sus query_params map<tea, tea> = {
            "list-type": "2",
            "prefix": prefix,
            "max-keys": to_string(max_keys),
        }
        
        url = add_query_params(url, query_params)
        sus headers map<tea, tea> = {}
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "GET", url, headers, [], "s3", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.get(url, signed_headers) fam {
            when err -> yikes "s3 list_objects failed: " + err
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_s3_error(response.body)
            yikes "s3 list_objects failed: " + error_msg
        }
        
        damn self.parse_list_objects_response(response.body) fam {
            when err -> yikes "failed to parse list objects response: " + err
        }
    }
    
    slay create_presigned_url(bucket tea, key tea, method tea, expires_in drip) yikes<tea> {
        sus url tea = self.build_s3_url(bucket, key)
        sus timestamp drip = timez.now()
        sus datetime tea = format_datetime(timestamp)
        
        sus query_params map<tea, tea> = {
            "X-Amz-Algorithm": "AWS4-HMAC-SHA256",
            "X-Amz-Credential": self.config.access_key_id + "/" + self.get_credential_scope(datetime),
            "X-Amz-Date": datetime,
            "X-Amz-Expires": to_string(expires_in),
            "X-Amz-SignedHeaders": "host",
        }
        
        ready (len(self.config.session_token) > 0) {
            query_params["X-Amz-Security-Token"] = self.config.session_token
        }
        
        sus canonical_request tea = self.create_presigned_canonical_request(method, url, query_params)
        sus string_to_sign tea = self.create_string_to_sign(datetime, self.config.region, "s3", canonical_request)
        sus signature tea = self.calculate_signature(datetime[0:8], self.config.region, "s3", string_to_sign)
        
        query_params["X-Amz-Signature"] = signature
        
        damn add_query_params(url, query_params)
    }
    
    // Private helpers
    slay build_s3_url(bucket tea, key tea) tea {
        sus scheme tea = ready (self.config.use_ssl) "https" otherwise "http"
        sus endpoint tea = ready (len(self.config.endpoint_url) > 0) 
            self.config.endpoint_url 
        otherwise 
            "s3." + self.config.region + ".amazonaws.com"
        
        sus path tea = ready (len(key) > 0) "/" + url_encode(key) otherwise ""
        damn scheme + "://" + bucket + "." + endpoint + path
    }
}

// EC2 Service Implementation
squad EC2Client {
    config AWSConfig
    signer AWSSigner
    http_client httpz.Client
    
    slay create_ec2_client(config AWSConfig) EC2Client {
        damn EC2Client{
            .config = config,
            .signer = AWSSigner{.config = config},
            .http_client = httpz.create_client(.{
                .timeout = config.read_timeout,
                .max_retries = config.max_retries,
            }),
        }
    }
    
    slay describe_instances(instance_ids []tea) yikes<[]EC2Instance> {
        sus params map<tea, tea> = {
            "Action": "DescribeInstances",
            "Version": "2016-11-15",
        }
        
        // Add instance IDs to params
        bestie (i, instance_id := range instance_ids) {
            params["InstanceId." + to_string(i + 1)] = instance_id
        }
        
        sus response tea = self.make_ec2_request(params) fam {
            when err -> yikes err
        }
        
        damn self.parse_describe_instances_response(response) fam {
            when err -> yikes "failed to parse describe instances response: " + err
        }
    }
    
    slay run_instances(ami_id tea, instance_type tea, min_count drip, max_count drip) yikes<[]EC2Instance> {
        sus params map<tea, tea> = {
            "Action": "RunInstances",
            "Version": "2016-11-15",
            "ImageId": ami_id,
            "InstanceType": instance_type,
            "MinCount": to_string(min_count),
            "MaxCount": to_string(max_count),
        }
        
        sus response tea = self.make_ec2_request(params) fam {
            when err -> yikes err
        }
        
        damn self.parse_run_instances_response(response) fam {
            when err -> yikes "failed to parse run instances response: " + err
        }
    }
    
    slay terminate_instances(instance_ids []tea) yikes<[]EC2InstanceStateChange> {
        sus params map<tea, tea> = {
            "Action": "TerminateInstances",
            "Version": "2016-11-15",
        }
        
        bestie (i, instance_id := range instance_ids) {
            params["InstanceId." + to_string(i + 1)] = instance_id
        }
        
        sus response tea = self.make_ec2_request(params) fam {
            when err -> yikes err
        }
        
        damn self.parse_terminate_instances_response(response) fam {
            when err -> yikes "failed to parse terminate instances response: " + err
        }
    }
    
    // Private helper
    slay make_ec2_request(params map<tea, tea>) yikes<tea> {
        sus url tea = "https://ec2." + self.config.region + ".amazonaws.com/"
        sus body tea = build_query_string(params)
        sus headers map<tea, tea> = {
            "content-type": "application/x-amz-json-1.0",
        }
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "POST", url, headers, encode_string(body), "ec2", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.post(url, encode_string(body), signed_headers) fam {
            when err -> yikes "ec2 request failed: " + err
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_ec2_error(response.body)
            yikes "ec2 request failed: " + error_msg
        }
        
        damn decode_string(response.body)
    }
}

// Lambda Service Implementation  
squad LambdaClient {
    config AWSConfig
    signer AWSSigner
    http_client httpz.Client
    
    slay create_lambda_client(config AWSConfig) LambdaClient {
        damn LambdaClient{
            .config = config,
            .signer = AWSSigner{.config = config},
            .http_client = httpz.create_client(.{
                .timeout = config.read_timeout,
                .max_retries = config.max_retries,
            }),
        }
    }
    
    slay invoke(function_name tea, payload []lit, invocation_type tea) yikes<LambdaInvokeResult> {
        sus url tea = "https://lambda." + self.config.region + ".amazonaws.com/2015-03-31/functions/" + function_name + "/invocations"
        sus headers map<tea, tea> = {
            "content-type": "application/json",
            "x-amz-invocation-type": invocation_type,  // Event, RequestResponse, DryRun
        }
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "POST", url, headers, payload, "lambda", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.post(url, payload, signed_headers) fam {
            when err -> yikes "lambda invoke failed: " + err
        }
        
        ready (response.status_code >= 400) {
            sus error_msg tea = self.parse_lambda_error(response.body)
            yikes "lambda invoke failed: " + error_msg
        }
        
        damn LambdaInvokeResult{
            .status_code = response.status_code,
            .payload = response.body,
            .executed_version = response.headers["x-amz-executed-version"],
            .function_error = response.headers["x-amz-function-error"],
            .log_result = response.headers["x-amz-log-result"],
        }
    }
    
    slay create_function(function_config LambdaFunctionConfig) yikes<LambdaFunction> {
        sus url tea = "https://lambda." + self.config.region + ".amazonaws.com/2015-03-31/functions"
        sus body []lit = jsonz.marshal(function_config) fam {
            when err -> yikes "failed to marshal function config: " + err
        }
        sus headers map<tea, tea> = {
            "content-type": "application/json",
        }
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "POST", url, headers, body, "lambda", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.post(url, body, signed_headers) fam {
            when err -> yikes "lambda create_function failed: " + err
        }
        
        ready (response.status_code != 201) {
            sus error_msg tea = self.parse_lambda_error(response.body)
            yikes "lambda create_function failed: " + error_msg
        }
        
        damn jsonz.unmarshal<LambdaFunction>(response.body) fam {
            when err -> yikes "failed to parse create function response: " + err
        }
    }
    
    slay update_function_code(function_name tea, zip_file []lit) yikes<LambdaFunction> {
        sus url tea = "https://lambda." + self.config.region + ".amazonaws.com/2015-03-31/functions/" + function_name + "/code"
        sus body []lit = jsonz.marshal({
            "ZipFile": base64_encode(zip_file),
        }) fam {
            when err -> yikes "failed to marshal update request: " + err
        }
        sus headers map<tea, tea> = {
            "content-type": "application/json",
        }
        
        sus signed_headers map<tea, tea> = self.signer.sign_request(
            "PUT", url, headers, body, "lambda", timez.now()
        ) fam {
            when err -> yikes "failed to sign request: " + err
        }
        
        sus response httpz.Response = self.http_client.put(url, body, signed_headers) fam {
            when err -> yikes "lambda update_function_code failed: " + err
        }
        
        ready (response.status_code != 200) {
            sus error_msg tea = self.parse_lambda_error(response.body)
            yikes "lambda update_function_code failed: " + error_msg
        }
        
        damn jsonz.unmarshal<LambdaFunction>(response.body) fam {
            when err -> yikes "failed to parse update function response: " + err
        }
    }
}

// Data structures
squad S3Object {
    data []lit
    content_type tea
    content_length drip
    etag tea
    last_modified tea
}

squad S3PutResult {
    etag tea
    version_id tea
}

squad S3ListResult {
    objects []S3ObjectInfo
    is_truncated lit
    next_continuation_token tea
}

squad S3ObjectInfo {
    key tea
    size drip
    last_modified tea
    etag tea
    storage_class tea
}

squad EC2Instance {
    instance_id tea
    image_id tea
    state tea
    instance_type tea
    public_ip tea
    private_ip tea
    vpc_id tea
    subnet_id tea
    security_groups []tea
    tags map<tea, tea>
}

squad EC2InstanceStateChange {
    instance_id tea
    current_state tea
    previous_state tea
}

squad LambdaInvokeResult {
    status_code drip
    payload []lit
    executed_version tea
    function_error tea
    log_result tea
}

squad LambdaFunction {
    function_name tea
    function_arn tea
    runtime tea
    role tea
    handler tea
    code_size drip
    description tea
    timeout drip
    memory_size drip
    last_modified tea
    version tea
}

squad LambdaFunctionConfig {
    function_name tea
    runtime tea
    role tea
    handler tea
    code LambdaCode
    description tea = ""
    timeout drip = 3
    memory_size drip = 128
    environment map<tea, tea>
    tags map<tea, tea>
}

squad LambdaCode {
    zip_file []lit
    s3_bucket tea = ""
    s3_key tea = ""
    s3_object_version tea = ""
}

// High-level AWS client factory
squad AWSClient {
    config AWSConfig
    s3 S3Client
    ec2 EC2Client
    lambda LambdaClient
    
    slay create_aws_client(config AWSConfig) AWSClient {
        damn AWSClient{
            .config = config,
            .s3 = create_s3_client(config),
            .ec2 = create_ec2_client(config),
            .lambda = create_lambda_client(config),
        }
    }
}

// Credential providers
slay load_credentials_from_env() yikes<AWSCredentials> {
    sus access_key tea = envz.get("AWS_ACCESS_KEY_ID") fam {
        when _ -> yikes "AWS_ACCESS_KEY_ID environment variable not set"
    }
    
    sus secret_key tea = envz.get("AWS_SECRET_ACCESS_KEY") fam {
        when _ -> yikes "AWS_SECRET_ACCESS_KEY environment variable not set"
    }
    
    sus session_token tea = envz.get("AWS_SESSION_TOKEN") fam {
        when _ -> ""
    }
    
    damn AWSCredentials{
        .access_key_id = access_key,
        .secret_access_key = secret_key,
        .session_token = session_token,
    }
}

slay load_credentials_from_profile(profile_name tea) yikes<AWSCredentials> {
    sus credentials_file tea = envz.get("AWS_SHARED_CREDENTIALS_FILE") fam {
        when _ -> pathz.join(envz.get("HOME"), ".aws", "credentials")
    }
    
    sus profile_section tea = "[" + profile_name + "]"
    sus lines []tea = filez.read_lines(credentials_file) fam {
        when err -> yikes "failed to read credentials file: " + err
    }
    
    sus in_profile lit = false
    sus credentials AWSCredentials = {}
    
    bestie (line := range lines) {
        line = stringz.trim(line)
        
        ready (stringz.starts_with(line, "[")) {
            in_profile = stringz.equals(line, profile_section)
            continue
        }
        
        ready (in_profile && stringz.contains(line, "=")) {
            sus parts []tea = stringz.split(line, "=", 2)
            sus key tea = stringz.trim(parts[0])
            sus value tea = stringz.trim(parts[1])
            
            sick (key) {
                "aws_access_key_id" -> credentials.access_key_id = value
                "aws_secret_access_key" -> credentials.secret_access_key = value
                "aws_session_token" -> credentials.session_token = value
            }
        }
    }
    
    ready (len(credentials.access_key_id) == 0) {
        yikes "aws_access_key_id not found in profile " + profile_name
    }
    
    ready (len(credentials.secret_access_key) == 0) {
        yikes "aws_secret_access_key not found in profile " + profile_name
    }
    
    damn credentials
}

// Example usage functions
slay example_s3_usage() yikes<tea> {
    sus config AWSConfig = {
        .access_key_id = "your-access-key",
        .secret_access_key = "your-secret-key",
        .region = "us-west-2",
    }
    
    sus s3 S3Client = create_s3_client(config)
    
    // Upload file
    sus file_data []lit = filez.read_file("example.txt") fam {
        when err -> yikes err
    }
    
    sus put_result S3PutResult = s3.put_object(
        "my-bucket", 
        "uploads/example.txt", 
        file_data, 
        "text/plain"
    ) fam {
        when err -> yikes err
    }
    
    vibez.spill("File uploaded with ETag:", put_result.etag)
    
    // List objects
    sus list_result S3ListResult = s3.list_objects("my-bucket", "uploads/", 10) fam {
        when err -> yikes err
    }
    
    vibez.spill("Found", len(list_result.objects), "objects:")
    bestie (obj := range list_result.objects) {
        vibez.spill("-", obj.key, "size:", obj.size)
    }
    
    // Generate presigned URL
    sus presigned_url tea = s3.create_presigned_url(
        "my-bucket", 
        "uploads/example.txt", 
        "GET", 
        3600  // 1 hour
    ) fam {
        when err -> yikes err
    }
    
    vibez.spill("Presigned URL:", presigned_url)
}

slay example_ec2_usage() yikes<tea> {
    sus config AWSConfig = {
        .access_key_id = "your-access-key",
        .secret_access_key = "your-secret-key",
        .region = "us-west-2",
    }
    
    sus ec2 EC2Client = create_ec2_client(config)
    
    // Launch instance
    sus instances []EC2Instance = ec2.run_instances(
        "ami-0abcdef1234567890",  // Amazon Linux 2 AMI
        "t2.micro",
        1,  // min count
        1   // max count
    ) fam {
        when err -> yikes err
    }
    
    sus instance_id tea = instances[0].instance_id
    vibez.spill("Launched instance:", instance_id)
    
    // Wait a bit then describe instance
    concurrenz.sleep(5000)
    
    sus described_instances []EC2Instance = ec2.describe_instances([instance_id]) fam {
        when err -> yikes err
    }
    
    bestie (instance := range described_instances) {
        vibez.spill("Instance", instance.instance_id, "state:", instance.state)
        vibez.spill("Public IP:", instance.public_ip)
    }
}

slay example_lambda_usage() yikes<tea> {
    sus config AWSConfig = {
        .access_key_id = "your-access-key",
        .secret_access_key = "your-secret-key",
        .region = "us-west-2",
    }
    
    sus lambda LambdaClient = create_lambda_client(config)
    
    // Invoke existing function
    sus payload []lit = jsonz.marshal({
        "name": "World",
        "message": "Hello from CURSED!",
    }) fam {
        when err -> yikes err
    }
    
    sus result LambdaInvokeResult = lambda.invoke(
        "my-hello-function",
        payload,
        "RequestResponse"
    ) fam {
        when err -> yikes err
    }
    
    vibez.spill("Lambda response status:", result.status_code)
    vibez.spill("Lambda response payload:", decode_string(result.payload))
    
    ready (len(result.function_error) > 0) {
        vibez.spill("Function error:", result.function_error)
    }
}
