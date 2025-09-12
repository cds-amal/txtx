#!/bin/bash

# Script to run VSCode extension tests in headless mode

echo "Running VSCode extension tests in headless mode..."

# Check if xvfb is installed
if ! command -v xvfb-run &> /dev/null; then
    echo "xvfb-run not found. Installing xvfb..."
    sudo apt-get update
    sudo apt-get install -y xvfb
fi

# Compile the extension and tests
echo "Compiling extension..."
npm run compile

echo "Compiling tests..."
npm run compile-tests

# Run tests with virtual display
echo "Starting tests with virtual display..."
xvfb-run -a npm test

# Capture exit code
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Tests passed successfully!"
else
    echo "❌ Tests failed with exit code $EXIT_CODE"
fi

exit $EXIT_CODE