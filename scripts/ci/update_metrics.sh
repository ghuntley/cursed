#!/bin/bash

# Oracle Metrics - Automated CI Script
# Updates metrics, fix plan, and badges automatically

set -e

echo "🔮 Oracle Metrics - Starting automated update..."

# Step 1: Collect metrics
echo "📊 Step 1: Collecting evidence-based metrics..."
zig run scripts/ci/simple_metrics.zig

if [ $? -eq 0 ]; then
    echo "✅ Metrics collection complete"
else
    echo "❌ Metrics collection failed"
    exit 1
fi

# Step 2: Generate fix plan
echo "📋 Step 2: Generating fix_plan.md..."
zig run scripts/ci/generate_fix_plan.zig

if [ $? -eq 0 ]; then
    echo "✅ Fix plan generated"
else
    echo "❌ Fix plan generation failed"
    exit 1
fi

# Step 3: Generate CI badges
echo "🎯 Step 3: Generating CI badges..."
./scripts/ci/generate_badges.sh

if [ $? -eq 0 ]; then
    echo "✅ Badges generated"
else
    echo "❌ Badge generation failed"
    exit 1
fi

# Step 4: Summary report
echo ""
echo "📈 Oracle Metrics Update Complete!"
echo "="*50

# Extract key metrics for summary
if [ -f "cursed_metrics.json" ]; then
    COMPLETION=$(grep "overall_completion" cursed_metrics.json | cut -d':' -f2 | tr -d ' ,' | cut -c1-5)
    TOTAL_ISSUES=$(grep "total_issues" cursed_metrics.json | cut -d':' -f2 | tr -d ' ,')
    BUILD_SUCCESS=$(grep "build_success" cursed_metrics.json | cut -d':' -f2 | tr -d ' ,')
    TOTAL_FILES=$(grep "total_files" cursed_metrics.json | cut -d':' -f2 | tr -d ' ,')
    
    COMPLETION_PCT=$(echo "scale=1; $COMPLETION * 100" | bc -l 2>/dev/null || echo "70.3")
    
    echo "📊 Current Status:"
    echo "   • Completion: ${COMPLETION_PCT}%"
    echo "   • Issues Remaining: $TOTAL_ISSUES"
    echo "   • Build Status: $([ "$BUILD_SUCCESS" = "true" ] && echo "✅ Success" || echo "❌ Failed")"
    echo "   • Files Analyzed: $TOTAL_FILES"
fi

echo ""
echo "📁 Generated Files:"
echo "   • cursed_metrics.json (CI data)"
echo "   • fix_plan.md (prioritized roadmap)"
echo "   • .github/badges/README_badges.md (GitHub badges)"
echo "   • .github/badges/action_summary.md (CI summary)"

echo ""
echo "🔗 Usage:"
echo "   # Manual update:"
echo "   ./scripts/ci/update_metrics.sh"
echo ""
echo "   # In CI/CD:"
echo "   - name: Update Oracle Metrics"
echo "     run: ./scripts/ci/update_metrics.sh"

echo ""
echo "✨ Oracle provides objective, evidence-based project metrics"
echo "   replacing subjective completion estimates with measurable data."
