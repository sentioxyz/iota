// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Navigate, useNavigate, useParams } from 'react-router-dom';
import { HideShowDisplayBox, Loading, Overlay } from '_components';
import { useAccounts } from '../../hooks';
import { Ed25519PublicKey } from '@iota/iota-sdk/keypairs/ed25519';

export function ExportPublicKeyPage() {
    const { accountID } = useParams();
    const { data: allAccounts, isPending } = useAccounts();
    const account = allAccounts?.find(({ id }) => accountID === id) || null;
    const navigate = useNavigate();

    if (!account && !isPending) {
        return <Navigate to="/accounts/manage" replace />;
    }

    const publicKey = account?.publicKey ? new Ed25519PublicKey(account.publicKey) : null;

    return (
        <Overlay title="Export Public Key" closeOverlay={() => navigate(-1)} showModal>
            <Loading loading={isPending}>
                <div className="flex flex-col gap-md">
                    <HideShowDisplayBox
                        value={publicKey ? publicKey?.toIotaPublicKey() : ''}
                            copiedMessage="Public Key copied"
                        />
                </div>
            </Loading>
        </Overlay>
    );
}
