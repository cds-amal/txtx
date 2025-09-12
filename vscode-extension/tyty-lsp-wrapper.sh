#!/bin/bash
# Wrapper script for tyty lsp that ignores --stdio flag

# Filter out --stdio from arguments
args=()
for arg in "$@"; do
    if [[ "$arg" != "--stdio" ]]; then
        args+=("$arg")
    fi
done

# Run tyty lsp with filtered arguments
exec tyty lsp "${args[@]}"