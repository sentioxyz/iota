# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2025 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# tests that building a package that implicitly depends on `Bridge` can build
iota move build -p example 2> /dev/null
