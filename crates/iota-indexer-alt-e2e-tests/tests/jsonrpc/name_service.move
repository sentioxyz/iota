// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 70 --simulator

// Testing various input errors for the IotaNS name resolution:
// 1. Not enough labels (need at least two)
// 2. Too long
// 3. Bad (inconsistent) use of separators
// 4. Indvidual label too short
// 5. Individual label too long
// 6, 7, 8, 9. Bad characters (non-alphanumeric, or leading/trailing hyphen)

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["foo"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.iota"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["foo*bar.iota"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["foo..iota"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["toolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolong.iota"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["-foo.iota"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["foo-.iota"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["foo_bar.iota"]
}

//# run-jsonrpc
{
  "method": "iotax_resolveNameServiceAddress",
  "params": ["🫠.iota"]
}
