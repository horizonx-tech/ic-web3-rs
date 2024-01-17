use core::fmt;

use ethabi::Token;
use ethereum_types::U256;
use num_bigint::BigInt;

use crate::contract::{tokens::Tokenizable, Error};

pub struct I256(pub BigInt);

impl Tokenizable for I256 {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match token {
            Token::Int(val) => {
                let bigint = to_bigint_from_u256(val);
                Ok(I256(bigint))
            },
            _ => Err(Error::InvalidOutputType("Expected int".to_owned())),
        }
    }
    #[inline(always)]
    fn into_token(self) -> Token {
        unimplemented!()
        // todo: ref https://github.com/horizonx-tech/ic-web3-rs/blob/4c01ebbe7fccdda733d170a65aa4c8bac5649c87/src/contract/tokens.rs#L243-L255
    }
}

impl fmt::Debug for I256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I256({})", self.0)
    }
}

fn to_bigint_from_u256(val: U256) -> BigInt {
    let mut bytes = [0u8; 32];
    val.to_big_endian(&mut bytes);
    BigInt::from_signed_bytes_be(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_max_bigint_from_u256() {
        let val = U256::MAX / U256::from("2"); // = max int128
        let bigint = to_bigint_from_u256(val);
        assert_eq!(
            format!("{:?}", bigint),
            "57896044618658097711785492504343953926634992332820282019728792003956564819967"
        )
    }

    #[test]
    fn test_to_min_bigint_from_u256() {
        let val = U256::MAX / U256::from("2") + U256::one(); // = max int128 + 1 = min int128
        let bigint = to_bigint_from_u256(val);
        assert_eq!(
            format!("{:?}", bigint),
            "-57896044618658097711785492504343953926634992332820282019728792003956564819968"
        )
    }
}
