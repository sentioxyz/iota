// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use iota_open_rpc::Module;
use iota_open_rpc_macros::open_rpc;
use iota_types::base_types::IotaAddress;

use crate::{context::Context, error::InternalContext as _};

use super::rpc_module::RpcModule;

use self::error::Error;

mod error;
mod response;

#[open_rpc(namespace = "iotax", tag = "Name Service API")]
#[rpc(server, namespace = "iotax")]
trait NameServiceApi {
    /// Resolve a IotaNS name to its address
    #[method(name = "resolveNameServiceAddress")]
    async fn resolve_name_service_address(
        &self,
        /// The name to resolve
        name: String,
    ) -> RpcResult<Option<IotaAddress>>;
}

pub(crate) struct NameService(pub Context);

#[async_trait::async_trait]
impl NameServiceApiServer for NameService {
    async fn resolve_name_service_address(&self, name: String) -> RpcResult<Option<IotaAddress>> {
        let Self(ctx) = self;
        Ok(response::resolved_address(ctx, &name)
            .await
            .with_internal_context(|| format!("Resolving IotaNS name {name:?}"))?)
    }
}

impl RpcModule for NameService {
    fn schema(&self) -> Module {
        NameServiceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}
