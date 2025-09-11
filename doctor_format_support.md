# Doctor Command Format Support

The doctor command now supports multiple output formats for better integration with editors and tooling.

## Usage

```bash
txtx doctor --format <format>
```

## Available Formats

### `auto` (default)
Automatically detects the appropriate format:
- Uses `quickfix` when output is piped or in CI environments
- Uses `pretty` when outputting to terminal

### `pretty`
Human-readable format with colors and detailed context:
```
runbook.tx:10:5: error[1]: Field 'from' does not exist on action
   context line here
   Documentation: https://docs.txtx.dev/...
```

### `quickfix`
Single-line format for editor integration (Vim/Neovim quickfix):
```
runbook.tx:10:5: error: Field 'from' does not exist (see: https://docs.txtx.dev/...)
runbook.tx:15:3: warning: Unused variable 'x' (hint: remove or use it)
```

Note: When specific line/column information is not available (e.g., for manifest-level errors), 
the format defaults to `file:1:` to ensure editor navigation still works.

### `json`
Machine-readable JSON format for tooling integration:
```json
{
  "errors": [
    {
      "file": "runbook.tx",
      "line": 10,
      "column": 5,
      "level": "error",
      "message": "Field 'from' does not exist",
      "documentation": "https://docs.txtx.dev/..."
    }
  ],
  "warnings": [...],
  "suggestions": [...]
}
```

## Editor Integration

### Neovim/Vim
```vim
" In your vimrc/init.vim
set makeprg=txtx\ doctor\ --format=quickfix
set errorformat=%f:%l:%c:\ %t%*[^:]:\ %m

" Run doctor on current file
:make

" Or use as a command
command! TxtxCheck cgetexpr system('txtx doctor --format=quickfix')
```

### VS Code
Create a task in `.vscode/tasks.json`:
```json
{
  "label": "txtx doctor",
  "type": "shell",
  "command": "txtx doctor --format=quickfix",
  "problemMatcher": {
    "owner": "txtx",
    "fileLocation": ["relative", "${workspaceFolder}"],
    "pattern": {
      "regexp": "^(.*):(\\d+):(\\d+): (error|warning): (.*)$",
      "file": 1,
      "line": 2,
      "column": 3,
      "severity": 4,
      "message": 5
    }
  }
}
```

## Environment Variables

Set default format via environment variable:
```bash
export TXTX_DOCTOR_FORMAT=quickfix
```

## Examples

```bash
# Pretty output in terminal
txtx doctor

# Quickfix format for editor
txtx doctor --format=quickfix

# JSON for tooling
txtx doctor --format=json > results.json

# Auto-detect (quickfix when piped)
txtx doctor | vim -q -
```