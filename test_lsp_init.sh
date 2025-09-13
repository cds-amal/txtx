#!/bin/bash

# Test LSP initialization
INIT_MSG='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file:///tmp","capabilities":{}}}'
CONTENT_LENGTH=${#INIT_MSG}

{
  echo "Content-Length: $CONTENT_LENGTH"
  echo ""
  echo -n "$INIT_MSG"
  sleep 1
  # Send shutdown
  SHUTDOWN_MSG='{"jsonrpc":"2.0","id":2,"method":"shutdown"}'
  SHUTDOWN_LENGTH=${#SHUTDOWN_MSG}
  echo "Content-Length: $SHUTDOWN_LENGTH"
  echo ""
  echo -n "$SHUTDOWN_MSG"
  sleep 1
  # Send exit
  EXIT_MSG='{"jsonrpc":"2.0","method":"exit"}'
  EXIT_LENGTH=${#EXIT_MSG}
  echo "Content-Length: $EXIT_LENGTH"
  echo ""
  echo -n "$EXIT_MSG"
} | ./target/release/txtx lsp 2>&1