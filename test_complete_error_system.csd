slay risky_operation() {
    sus error_result = yikes "Something went wrong"
    damn error_result
}

fam {
    sus result = risky_operation()
    vibez.spill("Operation succeeded:", result)
} sus error_var {
    vibez.spill("Caught error:", error_var)
}
