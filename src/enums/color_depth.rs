use strum_macros::{AsRefStr, Display};

use crate::Error;

/// Screen color depth in bits per pixel.
///
/// Valid values per EMVCo 3D Secure specification.
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
/// Color depth is a generic device characteristic requiring no security protection.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum ColorDepth {
    Bits1 = 1,
    Bits4 = 4,
    Bits8 = 8,
    Bits15 = 15,
    Bits16 = 16,
    Bits24 = 24,
    Bits32 = 32,
    Bits48 = 48,
}

impl TryFrom<u8> for ColorDepth {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Bits1),
            4 => Ok(Self::Bits4),
            8 => Ok(Self::Bits8),
            15 => Ok(Self::Bits15),
            16 => Ok(Self::Bits16),
            24 => Ok(Self::Bits24),
            32 => Ok(Self::Bits32),
            48 => Ok(Self::Bits48),
            _ => Err(Error::InvalidInput(
                "color depth must be 1, 4, 8, 15, 16, 24, 32, or 48".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_valid_values() {
        for (input, expected) in [
            (1, ColorDepth::Bits1),
            (4, ColorDepth::Bits4),
            (8, ColorDepth::Bits8),
            (15, ColorDepth::Bits15),
            (16, ColorDepth::Bits16),
            (24, ColorDepth::Bits24),
            (32, ColorDepth::Bits32),
            (48, ColorDepth::Bits48),
        ] {
            assert_eq!(ColorDepth::try_from(input).unwrap(), expected);
        }
    }

    #[test]
    fn rejects_invalid_values() {
        for input in [
            0, 2, 3, 5, 6, 7, 9, 10, 14, 17, 23, 25, 31, 33, 47, 49, 64, 255,
        ] {
            assert!(matches!(
                ColorDepth::try_from(input),
                Err(Error::InvalidInput(_))
            ));
        }
    }
}
