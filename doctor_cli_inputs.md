# Doctor Command CLI Inputs

The doctor command now accepts command-line input overrides via the `--input` or `-i` flag. This allows you to provide input values directly from the command line, which take precedence over environment values.

## Usage

```bash
# Basic syntax
txtx doctor --input KEY=VALUE

# Multiple inputs
txtx doctor --input PRIVATE_KEY=0x123... --input RPC_URL=https://mainnet.infura.io

# Short form
txtx doctor -i PRIVATE_KEY=0x123... -i RPC_URL=https://mainnet.infura.io

# With specific environment
txtx doctor --env mainnet --input PRIVATE_KEY=0x123...

# With specific runbook
txtx doctor myrunbook --env mainnet --input PRIVATE_KEY=0x123...
```

## Precedence Order

Input values are resolved in the following order (highest to lowest precedence):

1. **CLI inputs** (`--input KEY=VALUE`) - Always take precedence
2. **Environment-specific values** (e.g., `environments.mainnet.KEY`)
3. **Global environment values** (`environments.global.KEY`)
4. **No value** - Results in validation error

## Examples

### Example 1: Override a single value

```yaml
# txtx.yml
environments:
  global:
    RPC_URL: "https://testnet.example.com"
```

```bash
# Override RPC_URL for this validation
txtx doctor --input RPC_URL=https://mainnet.example.com
```

### Example 2: Provide missing values

```yaml
# txtx.yml
environments:
  mainnet:
    RPC_URL: "https://mainnet.example.com"
    # PRIVATE_KEY not defined
```

```bash
# Provide the missing PRIVATE_KEY
txtx doctor --env mainnet --input PRIVATE_KEY=0x123...
```

### Example 3: Multiple overrides

```bash
# Override multiple values at once
txtx doctor \
  --env mainnet \
  --input PRIVATE_KEY=0x123... \
  --input CONTRACT_ADDRESS=0x456... \
  --input GAS_LIMIT=3000000
```

## Implementation Details

The CLI inputs are:
- Parsed as key-value pairs separated by `=`
- Applied after environment inheritance
- Shown in doctor output with a note about precedence
- Not persisted to the manifest file

## Error Handling

Invalid input formats will result in an error:
```bash
# Missing equals sign
txtx doctor --input KEYVALUE  # Error: invalid KEY=VALUE

# Empty key
txtx doctor --input =VALUE    # Error: invalid KEY=VALUE
```