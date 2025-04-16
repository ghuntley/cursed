#!/bin/bash

echo "Tests with #[ignore] attribute:"
grep -r "#\[ignore" --include="*.rs" tests/

echo "
Tests with #[cfg(disable_test)] or similar:"
grep -r "#\[cfg(disable_test)\]" --include="*.rs" tests/
grep -r "#\[cfg(feature = \"disabled_test\")\]" --include="*.rs" tests/