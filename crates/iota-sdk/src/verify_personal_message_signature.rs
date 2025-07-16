// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::{error::Error, IotaClient};
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::traits::ToFromBytes;
use shared_crypto::intent::{Intent, IntentMessage, PersonalMessage};
use iota_json_rpc_types::ZkLoginIntentScope;
use iota_types::{
    base_types::IotaAddress,
    signature::{AuthenticatorTrait, GenericSignature, VerifyParams},
    signature_verification::VerifiedDigestCache,
};

/// Verify a signature against a personal message bytes and the iota address.
/// IotaClient is required to pass in if zkLogin signature is supplied.
pub async fn verify_personal_message_signature(
    signature: GenericSignature,
    message: &[u8],
    address: IotaAddress,
    client: Option<IotaClient>,
) -> Result<(), Error> {
    let intent_msg = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: message.to_vec(),
        },
    );
    match signature {
        GenericSignature::ZkLoginAuthenticator(ref _sig) => {
            if let Some(client) = client {
                let bytes = Base64::encode(message);
                let sig_string = Base64::encode(signature.as_bytes());
                let res = client
                    .read_api()
                    .verify_zklogin_signature(
                        bytes,
                        sig_string,
                        ZkLoginIntentScope::PersonalMessage,
                        address,
                    )
                    .await?;
                if res.success {
                    Ok(())
                } else {
                    Err(Error::InvalidSignature)
                }
            } else {
                Err(Error::InvalidSignature)
            }
        }
        _ => signature
            .verify_claims::<PersonalMessage>(
                &intent_msg,
                address,
                &VerifyParams::default(),
                Arc::new(VerifiedDigestCache::new_empty()),
            )
            .map_err(|_| Error::InvalidSignature),
    }
}
