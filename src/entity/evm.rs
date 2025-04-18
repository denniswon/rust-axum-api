
use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;

use sqlx::error::BoxDynError;
use sqlx::{Decode, Encode, Type};

use ethereum_types::H160;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvmAddress(pub H160);

impl std::fmt::Display for EvmAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl From<String> for EvmAddress {
    fn from(s: String) -> Self {
        Self(H160::from_str(&s).unwrap())
    }
}

impl From<EvmAddress> for String {
    fn from(evm_addr: EvmAddress) -> Self {
        evm_addr.0.to_string()
    }
}

impl From<H160> for EvmAddress {
    fn from(addr: H160) -> Self {
        Self(addr)
    }
}

impl From<EvmAddress> for H160 {
    fn from(evm_addr: EvmAddress) -> Self {
        evm_addr.0
    }
}

impl<'r, DB> Decode<'r, DB> for EvmAddress
where
    DB: sqlx::Database,
    &'r str: Decode<'r, DB>,
{
    fn decode(value: <DB as sqlx::Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <&str as Decode<DB>>::decode(value)?;
        Ok(EvmAddress(H160::from_str(s)?))
    }
}

impl<'a, DB> Encode<'a, DB> for EvmAddress
where
    DB: sqlx::Database,
    String: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        <String as Encode<DB>>::encode_by_ref(&format!("{:#x}", self.0), buf)
    }

    fn size_hint(&self) -> usize {
        <String as Encode<DB>>::size_hint(&format!("{:#x}", self.0))
    }
}

impl<DB> Type<DB> for EvmAddress
where
    DB: sqlx::Database,
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}
