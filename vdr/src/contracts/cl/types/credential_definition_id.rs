use crate::{contracts::did::types::did::DID, types::ContractParam, VdrError};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialDefinitionId(String);

impl CredentialDefinitionId {
    const ID_PATH: &'static str = "anoncreds/v0/CLAIM_DEF";

    pub fn build(issuer_id: &DID, schema_id: &str, tag: &str) -> CredentialDefinitionId {
        CredentialDefinitionId::from(
            format!(
                "{}/{}/{}/{}",
                issuer_id.as_ref(),
                Self::ID_PATH,
                schema_id,
                tag
            )
            .as_str(),
        )
    }
}

impl TryFrom<&CredentialDefinitionId> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &CredentialDefinitionId) -> Result<Self, Self::Error> {
        Ok(ContractParam::String(value.to_string()))
    }
}

impl From<&str> for CredentialDefinitionId {
    fn from(id: &str) -> Self {
        CredentialDefinitionId(id.to_string())
    }
}

impl AsRef<str> for CredentialDefinitionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToString for CredentialDefinitionId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
