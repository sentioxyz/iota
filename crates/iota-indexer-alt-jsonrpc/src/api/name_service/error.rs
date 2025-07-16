// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Domain not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    NameService(iota_name_service::NameServiceError),
}
