# Example of new doctor error output format

Old format:
```
1. Field "from" does not exist on action "transfer" (evm::send_eth)
   The send_eth action only outputs: tx_hash
   Documentation: https://docs.txtx.sh/addons/evm/actions#send-eth
```

New format (clickable in IDEs):
```
runbooks/transfer.tx: error[1]: Field "from" does not exist on action "transfer" (evm::send_eth)
   The send_eth action only outputs: tx_hash
   Documentation: https://docs.txtx.sh/addons/evm/actions#send-eth

runbooks/deploy.tx: warning: Unused input variable "OLD_CONTRACT"
   Suggestion: Remove unused variable or use it in the runbook
```

When line/column info becomes available:
```
runbooks/transfer.tx:15:8: error[1]: Field "from" does not exist on action "transfer"
runbooks/deploy.tx:3:5: warning: Unused input variable "OLD_CONTRACT"
```
