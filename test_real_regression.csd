# This should trigger actual parser errors

# Test with actual malformed imports that should cause UnexpectedToken
yeet "mathz" "stringz"  # Missing comma - should error

# Test chained imports without proper syntax
yeet "mathz" as math yeet "stringz"  # Two yeets on one line

# Test incomplete import
yeet 

# Test import with unexpected tokens after
yeet "mathz" { vibez.spill("inside import?") }

vibez.spill("Done")
