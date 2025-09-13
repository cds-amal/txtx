#!/bin/bash
# Test all doctor fixtures manually with both pretty and JSON output

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TXTX_BIN="${TXTX_BIN:-txtx}"

echo "=== Testing all doctor fixtures ==="
echo "Using txtx binary: $TXTX_BIN"
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test each fixture
for fixture in "$SCRIPT_DIR"/*.tx; do
    if [ -f "$fixture" ]; then
        filename=$(basename "$fixture")
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "Testing: $filename"
        echo "────────────────────────────────────────────────────────────"
        
        # Extract expected errors from comments
        expected=$(grep -E "^//\s*Expected errors?:" "$fixture" | sed 's/.*Expected errors\?: //')
        if [ -n "$expected" ]; then
            echo "Expected: $expected errors"
        fi
        
        # Run with pretty format
        echo ""
        echo "Pretty format:"
        $TXTX_BIN doctor "$fixture" --format pretty
        EXIT_CODE=$?
        
        if [ $EXIT_CODE -eq 0 ]; then
            echo -e "${GREEN}✓ Passed (no errors)${NC}"
        else
            echo -e "${RED}✗ Failed (found errors)${NC}"
        fi
        
        # Run with JSON format
        echo ""
        echo "JSON format:"
        JSON_OUTPUT=$($TXTX_BIN doctor "$fixture" --format json 2>&1)
        if [ -n "$JSON_OUTPUT" ]; then
            echo "$JSON_OUTPUT" | jq '.' 2>/dev/null || echo "$JSON_OUTPUT"
            
            # Count errors if valid JSON
            if echo "$JSON_OUTPUT" | jq '.' >/dev/null 2>&1; then
                ERROR_COUNT=$(echo "$JSON_OUTPUT" | jq '.errors | length')
                WARNING_COUNT=$(echo "$JSON_OUTPUT" | jq '.warnings | length')
                echo ""
                echo "Summary: $ERROR_COUNT errors, $WARNING_COUNT warnings"
            fi
        else
            echo "{}"
            echo "Summary: 0 errors, 0 warnings"
        fi
        
        echo ""
    fi
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Testing complete!"