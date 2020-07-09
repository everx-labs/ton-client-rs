# Release Notes
All notable changes to this project will be documented in this file.

## 0.25.0 - Jul 09, 2020
### Featured
- New transaction wait mechanism. All account's shard blocks are checked for transaction to 
guarantee message expiration

### New
- `wait_transaction` function for awaiting previously sent message processing
- `send_message` returns message processing state for `wait_transaction` function

## 0.24.0 - Jun 01, 2020
### Featured
- Error resolving after message rejection
- All transaction producing functions return transaction fees

### New
- `run_local_msg` function for processing given message locally
- `run_local` funtion now take flag `emulate_transaction` to run contract in transaction executor
which processes all transaction phases to emulate processing on node. Transaction fees and updated
contract state are returnedi f this flag is `true`.

## 0.23.0 - May 18, 2020
### Featured
- Message creating and processing functions added: `create_run_message`, `create_deploy_message`,
`send_message`, `process_message`
- Link core as a regular rust dependency.
- `run_get` function added for running FunC get methods
- `request_core` function added for calling cor functions which are not yet added to client lib

## 0.21.0 - Apr 5, 2020
### Featured
- ABI version 2 supported. See specification at https://docs.ton.dev
- Message expiration implemented. Check [usage guide](https://docs.ton.dev/86757ecb2/p/88321a-message-expiration-time)

### New
- `get_deploy_data` function added

## 0.20.100 - Feb 17, 2020
### New
- `deploy` function now checks the account state before sending message and returns `alreadyDeployed = true` if account is already active.
- Messages are sent to node via GraphQL requests, not REST API requests.

### Breaking Compatibility
- `deploy` returns structure `ResultOfDeploy` with account address and `alreadyDeployed` flag instead of just address.
