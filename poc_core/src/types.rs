use std::num::IntErrorKind;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NullableU8(pub Option<u8>);

impl NullableU8 {
    pub fn into_inner(self) -> Option<u8> {
        self.into()
    }
}

impl From<NullableU8> for Option<u8> {
    fn from(value: NullableU8) -> Self {
        value.0
    }
}

impl TryFrom<Option<i16>> for NullableU8 {
    type Error = IntErrorKind;

    fn try_from(value: Option<i16>) -> Result<Self, Self::Error> {
        Ok(match value {
            None => Self(None),
            Some(value) => {
                if value < 0 || value >= u8::MAX as i16 {
                    return Err(IntErrorKind::InvalidDigit);
                }

                Self(Some(value as u8))
            }
        })
    }
}
