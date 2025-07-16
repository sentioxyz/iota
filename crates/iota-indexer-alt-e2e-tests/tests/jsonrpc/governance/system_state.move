// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 70 --simulator --accounts A

//# run-jsonrpc
{
  "method": "iotax_getLatestIotaSystemState",
  "params": []
}

//# programmable --sender A --inputs 1000000000 object(0x5) @validator_0
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: iota_system::iota_system::request_add_stake(Input(1), Result(0), Input(2))

//# create-checkpoint

//# run-jsonrpc
{
  "method": "iotax_getLatestIotaSystemState",
  "params": []
}

//# advance-clock --duration-ns 1000000

//# advance-epoch

//# run-jsonrpc
{
  "method": "iotax_getLatestIotaSystemState",
  "params": []
}

//# programmable --sender A --inputs object(0x5) object(2,1)
//> 0: iota_system::iota_system::request_withdraw_stake(Input(0), Input(1))

//# create-checkpoint

//# run-jsonrpc
{
  "method": "iotax_getLatestIotaSystemState",
  "params": []
}

//# advance-epoch

//# run-jsonrpc
{
  "method": "iotax_getLatestIotaSystemState",
  "params": []
}
