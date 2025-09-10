# Doctor Command - Environment Handling

## Overview

The doctor command now properly validates input references against the txtx manifest's environment configuration, including support for the special `global` environment and inheritance rules.

## Environment Inheritance

### The `global` Environment

The `environments.global` entry in txtx.yml is special:
- It serves as the default/base environment for all other environments
- Values in `global` are inherited by all other environments
- Other environments can override specific values from `global`

### Example

```yaml
# txtx.yml
environments:
  global:
    CHAIN_ID: "1"
    RPC_URL: "https://mainnet.infura.io/v3/YOUR_KEY"
    GAS_PRICE: "20"
  
  dev:
    CHAIN_ID: "5"  # Overrides global
    # Inherits RPC_URL and GAS_PRICE from global
    
  prod:
    RPC_URL: "https://mainnet.alchemy.com/v2/YOUR_KEY"  # Overrides global
    # Inherits CHAIN_ID and GAS_PRICE from global
```

### How Doctor Validates

1. **Default Environment**: If no environment is specified with `--env`, doctor defaults to `global` (if it exists)

2. **Environment Building**: Doctor builds an "effective environment" by:
   - Starting with all values from `global` (if it exists)
   - Overlaying values from the specific environment
   - This gives you the complete set of available inputs

3. **Validation**: When checking `input.VARIABLE` references:
   - Doctor checks against the effective environment (merged values)
   - Errors show which values are missing
   - Suggestions indicate whether to add to `global` or the specific environment

## Input Precedence

The order of precedence for input values (highest to lowest):

1. **CLI Arguments** (`--input KEY=value`)
2. **Environment-specific values** (e.g., `environments.dev.KEY`)
3. **Global values** (`environments.global.KEY`)

Doctor now adds a note about this precedence when inputs are used in a runbook.

## Example Doctor Output

For a runbook using `input.DATABASE_URL` that's missing:

```
❌ Input 'input.DATABASE_URL' is not defined in environment 'dev' (including inherited values)
   Add 'DATABASE_URL' to your txtx.yml file (consider adding to 'global' if used across environments)
   Documentation: https://docs.txtx.sh/concepts/manifest#environments

Suggestions:
• Environment 'dev' inherits from 'global'. Values in 'dev' override those in 'global'.
• Add the missing input to your environment
  environments:
    dev:
      DATABASE_URL: "<value>"
  
  # Or add to global for all environments:
  environments:
    global:
      DATABASE_URL: "<value>"
• Note: Values passed via --input on CLI take precedence over environment values
  txtx run myrunbook --input DATABASE_URL=override_value
```

## Benefits

1. **DRY Configuration**: Define common values once in `global`
2. **Environment-Specific Overrides**: Only override what's different
3. **Clear Error Messages**: Doctor explains inheritance and suggests where to add values
4. **CLI Flexibility**: Override any value at runtime with `--input`

## Testing

The doctor command includes tests for:
- Global environment inheritance
- Environment-specific overrides
- Missing input detection with inheritance
- CLI precedence documentation

This ensures the validation logic correctly handles all the inheritance scenarios.