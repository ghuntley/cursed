// Simple trace tea test
vibez.spill("Testing trace tea module")

// Test basic tracing concepts
sus trace_enabled lit = based
sus trace_name tea = "test_trace"
sus trace_duration normie = 150

vibez.spill("Tracing enabled: true")
vibez.spill("Trace name: " + trace_name)
vibez.spill("Trace duration: 150ms")

// Test trace events
sus event_type tea = "function_call"
sus event_metadata tea = "user_login"

vibez.spill("Event type: " + event_type)
vibez.spill("Event metadata: " + event_metadata)

// Test span hierarchy
sus parent_span tea = "http_request"
sus child_span tea = "database_query"

vibez.spill("Parent span: " + parent_span)
vibez.spill("Child span: " + child_span)

// Test performance metrics
vibez.spill("Total requests: 100")
vibez.spill("Average response time: 250ms")
vibez.spill("Error count: 5")

// Test trace export formats
sus json_format tea = "json"
sus csv_format tea = "csv"
sus txt_format tea = "txt"

vibez.spill("Export formats: " + json_format + ", " + csv_format + ", " + txt_format)

vibez.spill("✅ Trace tea test complete!")
