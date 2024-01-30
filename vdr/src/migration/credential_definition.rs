use crate::{
    contracts::{
        cl::types::credential_definition::CredentialDefinitionTypes, did::types::did::DID,
    },
    error::{VdrError, VdrResult},
    migration::{DID_METHOD, NETWORK},
    CredentialDefinition, CredentialDefinitionId, SchemaId,
};
use log::warn;
use log_derive::{logfn, logfn_inputs};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndyCredentialDefinitionFormat {
    pub id: String,
    #[serde(rename = "schemaId")]
    pub schema_id: String,
    #[serde(rename = "type")]
    pub type_: CredentialDefinitionTypes,
    pub tag: String,
    pub value: serde_json::Value,
    #[serde(default)]
    pub ver: String,
}

impl CredentialDefinitionId {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn from_indy_format(id: &str) -> VdrResult<CredentialDefinitionId> {
        let parts: Vec<&str> = id.split(':').collect();
        let id = parts.get(0).ok_or_else(|| {
            let vdr_error = VdrError::CommonInvalidData("Invalid indy cred def id".to_string());

            warn!(
                "Error: {:?} during converting CredentialDefinitionId from indy format",
                vdr_error
            );

            vdr_error
        })?;
        let schema_id = parts.get(3).ok_or_else(|| {
            let vdr_error =
                VdrError::CommonInvalidData("Invalid indy cred def schema id".to_string());

            warn!(
                "Error: {:?} during converting CredentialDefinitionId from indy format",
                vdr_error
            );

            vdr_error
        })?;
        let tag = parts.get(4).ok_or_else(|| {
            let vdr_error = VdrError::CommonInvalidData("Invalid indy cred def tag".to_string());

            warn!(
                "Error: {:?} during converting CredentialDefinitionId from indy format",
                vdr_error
            );

            vdr_error
        })?;
        let issuer_did = DID::build(DID_METHOD, NETWORK, id);

        let cred_def_id = CredentialDefinitionId::build(&issuer_did, schema_id, tag);
        Ok(cred_def_id)
    }
}

impl CredentialDefinition {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn from_indy_format(credential_definition: &str) -> VdrResult<CredentialDefinition> {
        let indy_cred_def: IndyCredentialDefinitionFormat =
            serde_json::from_str(&credential_definition)
                .map_err(|_err| VdrError::CommonInvalidData("Invalid indy cred def".to_string()))?;
        let besu_cred_def = CredentialDefinition::try_from(indy_cred_def);
        besu_cred_def
    }
}

impl TryFrom<IndyCredentialDefinitionFormat> for CredentialDefinition {
    type Error = VdrError;

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn try_from(cred_def: IndyCredentialDefinitionFormat) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = cred_def.id.split(':').collect();
        let id = parts.get(0).ok_or_else(|| {
            let vdr_error = VdrError::CommonInvalidData("Invalid indy cred def id".to_string());

            warn!("Error: {:?} during converting CredentialDefinition from IndyCredentialDefinitionFormat", vdr_error);

            vdr_error
        })?;
        let issuer_id = DID::build(DID_METHOD, NETWORK, id);
        // TODO: How to deal with schema_id - now it's just sequence number?
        let schema_id = cred_def.schema_id.to_string();

        let besu_cred_def = CredentialDefinition {
            issuer_id,
            schema_id: SchemaId::from(schema_id.as_str()),
            cred_def_type: cred_def.type_.clone(),
            tag: cred_def.tag.to_string(),
            value: cred_def.value.clone(),
        };
        Ok(besu_cred_def)
    }
}

impl Into<IndyCredentialDefinitionFormat> for &CredentialDefinition {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn into(self) -> IndyCredentialDefinitionFormat {
        IndyCredentialDefinitionFormat {
            id: format!(
                "{}:3:{}:{}:{}",
                self.issuer_id.as_ref(),
                self.cred_def_type.as_ref(),
                self.schema_id.as_ref(),
                self.tag
            ),
            schema_id: self.schema_id.to_string(),
            type_: self.cred_def_type.clone(),
            tag: self.tag.to_string(),
            value: self.value.clone(),
            ver: "1.0".to_string(),
        }
    }
}