use crate::{VaultRequestMessage, VaultResponseMessage, VaultSync, VaultSyncCoreError};
use ockam_core::Result;
// Todo: Remove
// handle call method blocks
// use ockam_node::block_future;
use ockam_vault_core::{AsymmetricVault, PublicKey, Secret};

impl AsymmetricVault for VaultSync {
    fn ec_diffie_hellman(
        &mut self,
        context: &Secret,
        peer_public_key: &PublicKey,
    ) -> Result<Secret> {
        let resp: VaultResponseMessage = self.handle.call(VaultRequestMessage::EcDiffieHellman {
            context: context.clone(),
            peer_public_key: peer_public_key.clone(),
        })?;

        if let VaultResponseMessage::EcDiffieHellman(s) = resp {
            Ok(s)
        } else {
            Err(VaultSyncCoreError::InvalidResponseType.into())
        }

        // Todo: Remove after tests pass
        // block_future(&self.ctx.runtime(), async move {
        //
        //     self.send_message(VaultRequestMessage::EcDiffieHellman {
        //         context: context.clone(),
        //         peer_public_key: peer_public_key.clone(),
        //     })
        //     .await?;
        //
        //     let resp = self.receive_message().await?;
        //
        //     if let VaultResponseMessage::EcDiffieHellman(s) = resp {
        //         Ok(s)
        //     } else {
        //         Err(VaultSyncCoreError::InvalidResponseType.into())
        //     }
        // })
    }
}

#[cfg(test)]
mod tests {
    use ockam_vault::SoftwareVault;
    use ockam_vault_test_attribute::*;

    fn new_vault() -> SoftwareVault {
        SoftwareVault::default()
    }

    #[vault_test_sync]
    fn ec_diffie_hellman_curve25519() {}
}
