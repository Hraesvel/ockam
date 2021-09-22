use ockam_core::Result;
use ockam_vault_core::{Hasher, Secret, SecretAttributes, SmallBuffer};

// use ockam_node::block_future;

use crate::{VaultRequestMessage, VaultResponseMessage, VaultSync, VaultSyncCoreError};

impl Hasher for VaultSync {
    fn sha256(&mut self, data: &[u8]) -> Result<[u8; 32]> {
        let resp: VaultResponseMessage = self.handle.call(
            VaultRequestMessage::Sha256 { data: data.into() }
        )?;

        if let VaultResponseMessage::Sha256(s) = resp {
            Ok(s)
        } else {
            Err(VaultSyncCoreError::InvalidResponseType.into())
        }

        // Todo: Remove after tests pass
        // block_future(&self.ctx.runtime(), async move {
        //
        //     self.send_message(VaultRequestMessage::Sha256 { data: data.into() })
        //         .await?;
        //
        //     let resp = self.receive_message().await?;
        //
        //     if let VaultResponseMessage::Sha256(s) = resp {
        //         Ok(s)
        //     } else {
        //         Err(VaultSyncCoreError::InvalidResponseType.into())
        //     }
        // })
    }

    fn hkdf_sha256(
        &mut self,
        salt: &Secret,
        info: &[u8],
        ikm: Option<&Secret>,
        output_attributes: SmallBuffer<SecretAttributes>,
    ) -> Result<SmallBuffer<Secret>> {
        let resp: VaultResponseMessage = self.handle.call(
            VaultRequestMessage::HkdfSha256 {
                salt: salt.clone(),
                info: info.into(),
                ikm: ikm.cloned(),
                output_attributes,
            }
        )?;

        if let VaultResponseMessage::HkdfSha256(s) = resp {
            Ok(s)
        } else {
            Err(VaultSyncCoreError::InvalidResponseType.into())
        }

        // block_future(&self.ctx.runtime(), async move {
        //
        //     self.send_message(VaultRequestMessage::HkdfSha256 {
        //         salt: salt.clone(),
        //         info: info.into(),
        //         ikm: ikm.cloned(),
        //         output_attributes,
        //     })
        //     .await?;
        //
        //     let resp = self.receive_message().await?;
        //
        //     if let VaultResponseMessage::HkdfSha256(s) = resp {
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
    fn sha256() {}

    #[vault_test_sync]
    fn hkdf() {}
}
