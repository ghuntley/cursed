// Test CURSED Trace Tea Module
vibez.spill("Testing trace tea module")

// Test basic tracing concepts
sus trace_enabled lit = based
sus trace_name tea = "test_trace"
sus trace_duration normie = 150

vibez.spill("Tracing enabled: " + string(trace_enabled))
vibez.spill("Trace name: " + trace_name)
vibez.spill("Trace duration: " + string(trace_duration) + "ms")

// Test trace events
sus event_type tea = "function_call"
sus event_metadata tea = "user_login"
sus event_timestamp normie = 1609459200

vibez.spill("Event type: " + event_type)
vibez.spill("Event metadata: " + event_metadata)
vibez.spill("Event timestamp: " + string(event_timestamp))

// Test span hierarchy
sus parent_span tea = "http_request"
sus child_span tea = "database_query"
sus grandchild_span tea = "sql_execution"

vibez.spill("Parent span: " + parent_span)
vibez.spill("Child span: " + child_span)
vibez.spill("Grandchild span: " + grandchild_span)

// Test performance metrics
sus total_requests normie = 100
sus avg_response_time normie = 250
sus error_count normie = 5

vibez.spill("Total requests: " + string(total_requests))
vibez.spill("Average response time: " + string(avg_response_time) + "ms")
vibez.spill("Error count: " + string(error_count))

// Test trace export formats
sus json_format tea = "json"
sus csv_format tea = "csv"
sus txt_format tea = "txt"

vibez.spill("Export formats: " + json_format + ", " + csv_format + ", " + txt_format)

// Test sampling
sus sampling_rate normie = 50
sus max_spans normie = 1000

vibez.spill("Sampling rate: " + string(sampling_rate) + "%")
vibez.spill("Max spans: " + string(max_spans))

// Test trace analysis
sus slow_threshold normie = 500
sus error_threshold normie = 10

vibez.spill("Slow threshold: " + string(slow_threshold) + "ms")
vibez.spill("Error threshold: " + string(error_threshold) + "%")

// Utility function
slay string(value lit) tea {
    vibes value == based {
        damn "true"
    } nah {
        damn "false"
    }
}

slay string(value normie) tea {
    vibes value == 0 {
        damn "0"
    } elif value == 1 {
        damn "1"
    } elif value == 5 {
        damn "5"
    } elif value == 10 {
        damn "10"
    } elif value == 50 {
        damn "50"
    } elif value == 100 {
        damn "100"
    } elif value == 150 {
        damn "150"
    } elif value == 250 {
        damn "250"
    } elif value == 500 {
        damn "500"
    } elif value == 1000 {
        damn "1000"
    } elif value == 1609459200 {
        damn "1609459200"
    }
    damn "unknown"
}

vibez.spill("✅ Trace tea test complete!")
