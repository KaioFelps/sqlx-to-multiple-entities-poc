use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NullableU8(pub Option<u8>);

impl NullableU8 {
    pub fn some(v: u8) -> Self {
        Self(Some(v))
    }

    pub fn none() -> Self {
        Self(None)
    }

    pub fn into_inner(self) -> Option<u8> {
        self.into()
    }

    pub fn to_opt_i16(&self) -> Option<i16> {
        self.0.map(|n| n as i16)
    }

    pub fn into_opt_i16(self) -> Option<i16> {
        self.into()
    }
}

impl From<Option<u8>> for NullableU8 {
    fn from(value: Option<u8>) -> Self {
        Self(value)
    }
}

impl From<NullableU8> for Option<u8> {
    fn from(value: NullableU8) -> Self {
        value.0
    }
}

impl From<Option<i32>> for NullableU8 {
    fn from(value: Option<i32>) -> Self {
        value.map(|v| v as i16).into()
    }
}

impl From<Option<i16>> for NullableU8 {
    fn from(value: Option<i16>) -> Self {
        match value {
            None => Self(None),
            Some(value) => {
                if value < 0 || value >= u8::MAX as i16 {
                    panic!("Tried to parse an invalid u8 as a `NullabeU8`.");
                }

                Self(Some(value as u8))
            }
        }
    }
}

impl From<NullableU8> for Option<i16> {
    fn from(value: NullableU8) -> Self {
        value.0.map(|n| n as i16)
    }
}
