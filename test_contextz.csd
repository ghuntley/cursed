# Context Implementation Test
yeet "contextz"
yeet "vibez"

vibez.spill("Testing context functionality...")

# Test context creation
sus root_ctx = context_background()
sus timeout_ctx = context_with_timeout(root_ctx, duration_from_seconds(5))
sus cancel_ctx = context_with_cancel(root_ctx)

vibez.spill("✅ Context creation working")

# Test context values
sus value_ctx = context_with_value(root_ctx, "user_id", "12345")
sus retrieved_value = context_get_value(value_ctx, "user_id")

ready (retrieved_value != "12345") {
    vibez.spill("Context value retrieval failed")
    yikes "Context value retrieval failed"
}

vibez.spill("✅ Context values working")

# Test context cancellation
sus is_cancelled = context_is_cancelled(cancel_ctx)
ready (is_cancelled) {
    vibez.spill("Context should not be cancelled initially")
    yikes "Context cancellation check failed"
}

context_cancel(cancel_ctx)
sus now_cancelled = context_is_cancelled(cancel_ctx)
ready (!now_cancelled) {
    vibez.spill("Context should be cancelled after cancel()")
    yikes "Context cancellation failed"
}

vibez.spill("✅ Context cancellation working")

# Test context deadline
sus deadline_ctx = context_with_deadline(root_ctx, time_now_add(duration_from_seconds(10)))
sus has_deadline = context_has_deadline(deadline_ctx)

ready (!has_deadline) {
    vibez.spill("Context should have deadline")
    yikes "Context deadline check failed"
}

vibez.spill("✅ Context deadlines working")
vibez.spill("✅ All context tests passed")
