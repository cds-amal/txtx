# Doctor Command - Clickable Error Messages

The doctor command now outputs error and warning messages in a format that's recognized by most IDEs and editors, making them clickable for quick navigation to the problem location.

## Output Format

Errors and warnings follow the standard format used by compilers:

```
file:line:column: severity: message
```

### Examples

```bash
# Error format
runbooks/deploy.tx: error[1]: Field 'from' does not exist on action 'transfer'
   The send_eth action only outputs: tx_hash
   Documentation: https://docs.txtx.sh/addons/evm/actions#send-eth

# Warning format  
runbooks/deploy.tx: warning: Unused variable 'old_balance'
   Suggestion: Remove unused variable or prefix with underscore

# When line/column info is available (future enhancement)
runbooks/deploy.tx:15:8: error[1]: Field 'from' does not exist on action 'transfer'
```

## IDE Integration

### VSCode
- In the integrated terminal, errors are automatically recognized and underlined
- Click on the file path to jump to that file
- When line/column info is available, jumps directly to the error location

### Vim/Neovim
- Use `:cexpr system('txtx doctor')` to populate the quickfix list
- Navigate errors with `:cnext` and `:cprev`
- Or use `:make` with `makeprg=txtx\ doctor`

### Emacs
- Run `M-x compile` with `txtx doctor`
- Use `M-g M-n` (next-error) to jump between errors
- Works with compilation-mode out of the box

### IntelliJ IDEA / Other JetBrains IDEs
- Errors in terminal are automatically hyperlinked
- Click to navigate to the file

## Future Enhancements

Currently, the doctor command shows file paths but not specific line/column numbers. This is because the parser's AST doesn't preserve location information. Future improvements could include:

1. **Enhanced Parser AST**: Modify the parser to include location information in the AST nodes
2. **Source Mapping**: Track source locations during AST construction
3. **More Precise Errors**: Show exact line/column for each validation error

## Implementation Notes

The clickable format is implemented in the `display_results` function in `doctor/mod.rs`. The format follows the de facto standard used by most compilers:

- GCC/Clang: `file:line:column: error: message`
- Rust: `file:line:column: error[E0001]: message`
- Go: `file:line:column: message`

This ensures maximum compatibility across different development environments.