use crate::{
    error::{VdrError, VdrResult},
    Address,
};

use crate::utils::format_bytes32;
use ethabi::{Log, Token};
use log::warn;
use log_derive::{logfn, logfn_inputs};
use serde::{Deserialize, Serialize};

/// Contract configuration
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContractConfig {
    /// Address of deployed contract
    pub address: String,
    /// Contract ABI specification
    pub spec_path: Option<String>,
    /// Contract ABI specification
    pub spec: Option<ContractSpec>,
}

/// Contract ABI specification
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractSpec {
    /// Name of contract
    #[serde(rename = "contractName")]
    pub name: String,
    /// Contract ABI itself
    pub abi: serde_json::Value,
}

impl ContractSpec {
    /// Read and parse contract specification from a JSON file
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn from_file(spec_path: &str) -> VdrResult<Self> {
        let contract_spec = std::fs::read_to_string(spec_path).map_err(|err| {
            let vdr_error = VdrError::ContractInvalidSpec(format!(
                "Unable to read contract spec file. Err: {:?}",
                err
            ));

            warn!(
                "Error: {:?} during reading contract spec from file",
                vdr_error
            );

            vdr_error
        })?;
        let contract_spec = serde_json::from_str(&contract_spec).map_err(|err| {
            let vdr_error = VdrError::ContractInvalidSpec(format!(
                "Unable to parse contract specification. Err: {:?}",
                err.to_string()
            ));

            warn!(
                "Error: {:?} during paring contract specification",
                vdr_error
            );

            vdr_error
        });

        contract_spec
    }
}

/// Contract parameters representation (ethereum ABI)
pub type ContractParam = Token;

/// Helper wrapper for more convenient parsing of the contract execution results
#[derive(Debug)]
pub(crate) struct ContractOutput(Vec<ContractParam>);

impl ContractOutput {
    #[allow(unused)]
    pub fn new(data: Vec<ContractParam>) -> ContractOutput {
        ContractOutput(data)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_tuple(&self, index: usize) -> VdrResult<ContractOutput> {
        self.0
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing tuple value".to_string())
            })?
            .clone()
            .into_tuple()
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing tuple value".to_string()))
            .map(ContractOutput)
    }

    pub fn get_string(&self, index: usize) -> VdrResult<String> {
        self.0
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing string value".to_string())
            })?
            .clone()
            .into_string()
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing string value".to_string())
            })
    }

    pub fn get_address(&self, index: usize) -> VdrResult<Address> {
        let address_str = self
            .0
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })?
            .clone()
            .to_string();

        Ok(Address::from(address_str.as_str()))
    }

    pub fn get_bool(&self, index: usize) -> VdrResult<bool> {
        self.0
            .get(index)
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing bool value".to_string()))?
            .clone()
            .into_bool()
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing bool value".to_string()))
    }

    pub fn get_u8(&self, index: usize) -> VdrResult<u8> {
        Ok(self
            .0
            .get(index)
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing uint value".to_string()))?
            .clone()
            .into_uint()
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing uint value".to_string()))?
            .as_u32() as u8)
    }

    pub fn get_u64(&self, index: usize) -> VdrResult<u64> {
        Ok(self
            .0
            .get(index)
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing uint value".to_string()))?
            .clone()
            .into_uint()
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing uint value".to_string()))?
            .as_u64())
    }

    pub fn get_u128(&self, index: usize) -> VdrResult<u128> {
        Ok(self
            .0
            .get(index)
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing uint value".to_string()))?
            .clone()
            .into_uint()
            .ok_or_else(|| VdrError::ContractInvalidResponseData("Missing uint value".to_string()))?
            .as_u128())
    }

    pub fn get_address_array(&self, index: usize) -> VdrResult<Vec<Address>> {
        Ok(self
            .0
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData(
                    "Missing address string array value".to_string(),
                )
            })?
            .clone()
            .into_array()
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData(
                    "Missing address string array value".to_string(),
                )
            })?
            .into_iter()
            .map(|token| Address::from(token.to_string().as_str()))
            .collect())
    }
}

impl From<Vec<Token>> for ContractOutput {
    fn from(value: Vec<Token>) -> Self {
        ContractOutput(value)
    }
}

/// Helper wrapper for more convenient parsing of the contract event logs
#[derive(Debug)]
pub(crate) struct ContractEvent(Log);

impl ContractEvent {
    pub fn is_empty(&self) -> bool {
        self.0.params.is_empty()
    }

    pub fn get_address(&self, index: usize) -> VdrResult<Address> {
        let address_str = self
            .0
            .params
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })?
            .clone()
            .value
            .into_address()
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })?
            .to_string();

        Ok(Address::from(address_str.as_str()))
    }

    pub fn get_fixed_bytes(&self, index: usize) -> VdrResult<Vec<u8>> {
        self.0
            .params
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })?
            .clone()
            .value
            .into_fixed_bytes()
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })
    }

    pub fn get_bytes(&self, index: usize) -> VdrResult<Vec<u8>> {
        self.0
            .params
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })?
            .clone()
            .value
            .into_bytes()
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })
    }

    pub fn get_uint(&self, index: usize) -> VdrResult<u64> {
        self.0
            .params
            .get(index)
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })?
            .clone()
            .value
            .into_uint()
            .ok_or_else(|| {
                VdrError::ContractInvalidResponseData("Missing address value".to_string())
            })
            .map(|uint| uint.as_u64())
    }
}

impl From<Log> for ContractEvent {
    fn from(value: Log) -> Self {
        ContractEvent(value)
    }
}

#[derive(Debug)]
pub(crate) struct UintBytesParam(u64);

impl From<u64> for UintBytesParam {
    fn from(value: u64) -> Self {
        UintBytesParam(value)
    }
}

impl TryFrom<UintBytesParam> for ContractParam {
    type Error = VdrError;

    fn try_from(value: UintBytesParam) -> Result<Self, Self::Error> {
        Ok(ContractParam::FixedBytes(
            format_bytes32(value.0.to_be_bytes().as_slice())?.to_vec(),
        ))
    }
}

#[derive(Debug)]
pub(crate) struct MethodParam(&'static str);

impl From<&'static str> for MethodParam {
    fn from(value: &'static str) -> Self {
        MethodParam(value)
    }
}

impl TryFrom<MethodParam> for ContractParam {
    type Error = VdrError;

    fn try_from(value: MethodParam) -> Result<Self, Self::Error> {
        Ok(ContractParam::String(value.0.to_string()))
    }
}
