# OpenTelemetry Implementation Fixes

## Issues Identified

1. Using incorrect API calls (`:otel_tracer` instead of `OpenTelemetry.Tracer`)
2. Function arity mismatches (incorrect parameter passing)
3. Improper span context extraction (wrong functions for trace and span IDs)
4. Missing setup function that was referenced in application.ex
5. Insufficient error logging that would make debugging difficult

## Changes Made

1. Fixed `with_span/3` implementation:
   - Changed from `:otel_tracer.with_span/3` to `OpenTelemetry.Tracer.with_span/2` with proper attribute setting
   - Added proper attribute conversion from maps to key-value lists

2. Fixed `set_attributes/1` implementation:
   - Changed from `:otel_tracer.set_attributes/1` to `OpenTelemetry.Tracer.set_attributes/1`
   - Added conversion from map format to key-value tuples

3. Fixed `add_event/2` implementation:
   - Changed from `:otel_tracer.add_event/2` to `OpenTelemetry.Tracer.add_event/2`
   - Added attribute conversion for proper format

4. Fixed `set_status_error/1` implementation:
   - Changed from `:otel_tracer.set_status/2` to `OpenTelemetry.Tracer.set_status/2`

5. Fixed `current_trace_info/0` implementation:
   - Changed from `:otel_tracer.current_span_ctx()` to `OpenTelemetry.Tracer.current_span_ctx()`
   - Changed from `:opentelemetry.span_context_trace_id/1` to `OpenTelemetry.Span.trace_id/1`
   - Changed from `:opentelemetry.span_context_span_id/1` to `OpenTelemetry.Span.span_id/1`

6. Added a proper `setup/0` function that calls `setup_span_processors/0`

7. Enhanced error logging:
   - Added detailed error messages with proper context info
   - Included full exception details in logs

8. Added helper function `to_attribute_key/1`:
   - To convert different types of keys (atoms, strings, etc.) to proper format
   - Enables more flexible attribute passing

## Next Steps

1. Update the Audit module to use correct OpenTelemetry API
2. Update the Logger module to use correct OpenTelemetry API
3. Create tests to verify the OpenTelemetry integration works as expected
4. Ensure the Application calls the setup function correctly on startup
5. Update any other modules that use the OpenTelemetry API directly