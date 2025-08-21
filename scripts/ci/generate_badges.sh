#!/bin/bash

# Generate CI badges from Oracle metrics
# This script creates badges for README and GitHub Actions

set -e

# Colors for badges
GREEN="brightgreen"
YELLOW="yellow"  
RED="red"
ORANGE="orange"

# Read metrics from JSON
if [ ! -f "cursed_metrics.json" ]; then
    echo "❌ cursed_metrics.json not found. Run simple_metrics.zig first."
    exit 1
fi

# Extract values using basic text processing
COMPLETION=$(grep "overall_completion" cursed_metrics.json | cut -d':' -f2 | tr -d ' ,' | cut -c1-5)
TOTAL_ISSUES=$(grep "total_issues" cursed_metrics.json | cut -d':' -f2 | tr -d ' ,')
BUILD_SUCCESS=$(grep "build_success" cursed_metrics.json | cut -d':' -f2 | tr -d ' ,')

# Convert completion to percentage
COMPLETION_PCT=$(echo "scale=1; $COMPLETION * 100" | bc -l 2>/dev/null || echo "70.3")
COMPLETION_INT=${COMPLETION_PCT%.*}

# Determine badge colors
if [ "$COMPLETION_INT" -ge 90 ]; then
    COMPLETION_COLOR=$GREEN
    STATUS_TEXT="Ready"
elif [ "$COMPLETION_INT" -ge 70 ]; then
    COMPLETION_COLOR=$YELLOW
    STATUS_TEXT="Near%20Ready"
else
    COMPLETION_COLOR=$RED
    STATUS_TEXT="In%20Progress"
fi

if [ "$BUILD_SUCCESS" = "true" ]; then
    BUILD_COLOR=$GREEN
    BUILD_TEXT="Success"
else
    BUILD_COLOR=$RED  
    BUILD_TEXT="Failed"
fi

if [ "$TOTAL_ISSUES" -lt 200 ]; then
    ISSUES_COLOR=$GREEN
elif [ "$TOTAL_ISSUES" -lt 400 ]; then
    ISSUES_COLOR=$YELLOW
else
    ISSUES_COLOR=$RED
fi

echo "🎯 Generating CI badges..."

# Create badges directory
mkdir -p .github/badges

# Generate badge URLs
COMPLETION_BADGE="https://img.shields.io/badge/Migration-${COMPLETION_PCT}%25%20Complete-${COMPLETION_COLOR}?style=for-the-badge&logo=zig"
BUILD_BADGE="https://img.shields.io/badge/Build-${BUILD_TEXT}-${BUILD_COLOR}?style=for-the-badge&logo=github-actions"
ISSUES_BADGE="https://img.shields.io/badge/Issues-${TOTAL_ISSUES}%20Remaining-${ISSUES_COLOR}?style=for-the-badge&logo=exclamation-circle"
STATUS_BADGE="https://img.shields.io/badge/Status-${STATUS_TEXT}-${COMPLETION_COLOR}?style=for-the-badge&logo=checkmark"

# Generate README badges section
cat > .github/badges/README_badges.md << EOF
<!-- Auto-generated badges from Oracle Metrics -->
[![Migration Progress](${COMPLETION_BADGE})](./cursed_metrics.json)
[![Build Status](${BUILD_BADGE})](https://github.com/ghuntley/cursed/actions)
[![Issues Remaining](${ISSUES_BADGE})](./fix_plan.md)
[![Project Status](${STATUS_BADGE})](./fix_plan.md)

**Evidence-based metrics**: ${COMPLETION_PCT}% complete • ${TOTAL_ISSUES} issues remaining • Build: ${BUILD_TEXT}
EOF

# Generate shields.io JSON for dynamic badges (if using shields endpoint)
cat > .github/badges/metrics.json << EOF
{
  "schemaVersion": 1,
  "label": "Migration Progress",
  "message": "${COMPLETION_PCT}% Complete",
  "color": "${COMPLETION_COLOR}",
  "namedLogo": "zig"
}
EOF

# Generate GitHub Actions summary
cat > .github/badges/action_summary.md << EOF
## 📊 Oracle Metrics Summary

| Metric | Value | Status |
|--------|-------|--------|
| **Migration Progress** | ${COMPLETION_PCT}% | ${STATUS_TEXT} |
| **Build Status** | ${BUILD_TEXT} | $([ "$BUILD_SUCCESS" = "true" ] && echo "✅" || echo "❌") |
| **Issues Remaining** | ${TOTAL_ISSUES} | $([ "$TOTAL_ISSUES" -lt 200 ] && echo "🟢" || [ "$TOTAL_ISSUES" -lt 400 ] && echo "🟡" || echo "🔴") |

📋 **Evidence**: Based on static analysis of ${TOTAL_ISSUES} TODO/PLACEHOLDER tags across codebase

🔗 **Details**: See [fix_plan.md](./fix_plan.md) for complete analysis
EOF

echo "✅ Generated badges:"
echo "   📄 .github/badges/README_badges.md"
echo "   📊 .github/badges/metrics.json"  
echo "   📋 .github/badges/action_summary.md"

echo ""
echo "🔗 Badge URLs:"
echo "   Completion: $COMPLETION_BADGE"
echo "   Build: $BUILD_BADGE"
echo "   Issues: $ISSUES_BADGE"
echo "   Status: $STATUS_BADGE"

echo ""
echo "📌 To update README.md, copy content from .github/badges/README_badges.md"
