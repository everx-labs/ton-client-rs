# Release Notes
All notable changes to this project will be documented in this file.

## 0.20.100 - Feb 17, 2020
### New
- `deploy` function now checks the account state before sending message and returns `alreadyDeployed = true` if account is already active.
- Messages are sent to node via GraphQL requests, not REST API requests.

### Breaking Compatibility
- `deploy` returns structure `ResultOfDeploy` with account address and `alreadyDeployed` flag instead of just address.

## 0.21.0 - Apr 5, 2020
### Featured
- ABI version 2 supported. See specification at https://docs.ton.dev
- Message expiration implemented. Check [usage guide](https://docs.ton.dev/86757ecb2/p/88321a-message-expiration-time)
