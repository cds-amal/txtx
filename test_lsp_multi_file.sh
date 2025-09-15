#!/bin/bash

# Test LSP with multi-file runbook (separate-flows-file)

echo "Testing LSP with multi-file runbook..."
echo "======================================="

# Change to the test fixture directory
cd tests/fixtures/doctor/separate-flows-file

# First, let's test with doctor to confirm it works
echo -e "\n1. Testing with txtx doctor (should work):"
txtx doctor

# Now let's test the LSP
echo -e "\n2. Testing with LSP:"

# Start the LSP and send initialization
echo "Starting LSP server..."

# Create a temporary file for LSP communication
TMPFILE=$(mktemp)

# Start LSP in background and capture PID
txtx lsp > $TMPFILE 2>&1 &
LSP_PID=$!

# Give it a moment to start
sleep 1

# Function to send LSP request
send_request() {
    local content="$1"
    local length=${#content}
    echo -e "Content-Length: $length\r\n\r\n$content"
}

# Initialize LSP
INIT_REQUEST='{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "processId": null,
    "rootUri": "file://'$(pwd)'",
    "capabilities": {
      "textDocument": {
        "publishDiagnostics": {
          "relatedInformation": true
        }
      }
    }
  }
}'

echo "Sending initialize request..."
send_request "$INIT_REQUEST" | txtx lsp 2>&1 &
LSP_PID=$!
sleep 2

# Open the main.tx file
OPEN_REQUEST='{
  "jsonrpc": "2.0",
  "method": "textDocument/didOpen",
  "params": {
    "textDocument": {
      "uri": "file://'$(pwd)'/deploy/main.tx",
      "languageId": "txtx",
      "version": 1,
      "text": "'$(cat deploy/main.tx | sed 's/"/\\"/g' | tr '\n' ' ')'"
    }
  }
}'

echo -e "\n3. Opening main.tx file in LSP..."
send_request "$OPEN_REQUEST"

# Give LSP time to process
sleep 2

# Kill the LSP server
kill $LSP_PID 2>/dev/null

# Show the output
echo -e "\n4. LSP Output:"
cat $TMPFILE

# Cleanup
rm -f $TMPFILE

echo -e "\n5. Let's also check if the LSP recognizes this as a multi-file runbook:"
echo "Directory structure:"
ls -la deploy/

echo -e "\nContent of txtx.yml:"
cat txtx.yml

echo -e "\nContent of deploy/main.tx:"
cat deploy/main.tx

echo -e "\nContent of deploy/flows.tx:"
cat deploy/flows.tx