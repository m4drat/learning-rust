use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{ToPrimitive, Zero};
use std::ops;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    mantissa: BigInt,
    exponent: BigInt,
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x10E{}", self.mantissa, self.exponent)
    }
}

impl ops::Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, _rhs: Decimal) -> Decimal {
        let (lhs, rhs) = Decimal::aligned_mantissas(self.clone(), _rhs.clone());
        Decimal::new(lhs + rhs, self.exponent.max(_rhs.exponent))
    }
}

impl ops::Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, _rhs: Decimal) -> Decimal {
        self + Decimal::new(-_rhs.mantissa, _rhs.exponent)
    }
}

impl ops::Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, _rhs: Decimal) -> Decimal {
        Decimal::new(self.mantissa * _rhs.mantissa, self.exponent + _rhs.exponent)
    }
}

// impl PartialEq for Decimal {
//     fn eq(&self, other: &Self) -> bool {
//         self.mantissa == other.mantissa && self.exponent == other.exponent
//     }
// }

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (lhs, rhs) = Decimal::aligned_mantissas(self.clone(), other.clone());
        Some(lhs.cmp(&rhs))
    }
}

impl Decimal {
    pub fn new(mut mantissa: BigInt, mut exponent: BigInt) -> Decimal {
        while exponent > Zero::zero() && &mantissa % 10 == Zero::zero() {
            mantissa /= 10;
            exponent -= 1;
        }

        Self { mantissa, exponent }
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        // use itertools::Itertools;
        // let (integer, fraction) = match {
        //     input
        //         .split(".")
        //         .map(|part| BigInt::parse_bytes(part.as_bytes(), 10).unwrap())
        //         .collect::<Vec<BigInt>>()[..]
        // } {
        //     [var1, var2] => Some((var1, var2)),
        //     _ => None,
        // }
        // .unwrap();

        // if let Some((integer, fraction)) = input
        //     .split(".")
        //     .map(|num| BigInt::parse_bytes(num.as_bytes(), 10).unwrap())
        //     .tuples()
        //     .next()
        // {
        //     return Some(Decimal::new(integer, fraction));
        // }

        let mantissa = input.replace('.', "").parse::<BigInt>().ok()?;

        if let Some(index) = input.find('.') {
            Some(Decimal::new(
                mantissa,
                (input.len() - index - 1).to_bigint().unwrap(),
            ))
        } else {
            Some(Decimal::new(mantissa, 0.to_bigint().unwrap()))
        }
    }

    // pub fn aligned_mantissas(lhs: Decimal, rhs: Decimal) -> (BigInt, BigInt) {
    //     return match (lhs, rhs) {
    //         (lhs, rhs) if rhs.exponent > lhs.exponent => {
    //             let diff = rhs.exponent - lhs.exponent;
    //             (lhs.mantissa * diff * 10, rhs.mantissa)
    //         }
    //         (lhs, rhs) if lhs.exponent > rhs.exponent => {
    //             let diff = lhs.exponent - rhs.exponent;
    //             (lhs.mantissa, rhs.mantissa * diff * 10)
    //         }
    //         (lhs, rhs) => (lhs.mantissa, rhs.mantissa),
    //     };
    // }

    // fn aligned_mantissas(mut lhs: Decimal, mut rhs: Decimal) -> (BigInt, BigInt) {
    //     while lhs.exponent < rhs.exponent {
    //         lhs.mantissa *= 10;
    //         lhs.exponent += 1;
    //     }
    //     while rhs.exponent < lhs.exponent {
    //         rhs.mantissa *= 10;
    //         rhs.exponent += 1;
    //     }
    //     (lhs.mantissa, rhs.mantissa)
    // }

    pub fn aligned_mantissas(mut lhs: Decimal, mut rhs: Decimal) -> (BigInt, BigInt) {
        if rhs.exponent > lhs.exponent {
            let diff = &rhs.exponent - &lhs.exponent;
            lhs.mantissa *= 10.to_bigint().unwrap().pow(truncate_bigint_to_u32(&diff));
            lhs.exponent += diff;
        }

        if lhs.exponent > rhs.exponent {
            let diff = lhs.exponent - &rhs.exponent;
            rhs.mantissa *= 10.to_bigint().unwrap().pow(truncate_bigint_to_u32(&diff));
            rhs.exponent += diff;
        }

        (lhs.mantissa, rhs.mantissa)
    }
}

fn truncate_biguint_to_u32(a: &BigUint) -> u32 {
    use std::u32;
    let mask = BigUint::from(u32::MAX);
    (a & mask).to_u32().unwrap()
}

fn truncate_bigint_to_u32(a: &BigInt) -> u32 {
    use num_traits::Signed;

    let was_negative = a.is_negative();
    let abs = a.abs().to_biguint().unwrap();
    let truncated = truncate_biguint_to_u32(&abs);
    if was_negative {
        truncated.wrapping_neg()
    } else {
        truncated
    }
}

#[test]
fn test_aligned_mantissas() {
    let mut a = Decimal::try_from("13").unwrap();
    let mut b = Decimal::try_from("37").unwrap();
    assert!(Decimal::aligned_mantissas(a, b) == (13.to_bigint().unwrap(), 37.to_bigint().unwrap()));

    a = Decimal::try_from("1.1").unwrap();
    b = Decimal::try_from("1").unwrap();

    println!("{:?}", Decimal::aligned_mantissas(a.clone(), b.clone()));

    assert!(
        Decimal::aligned_mantissas(a.clone(), b.clone())
            == (11.to_bigint().unwrap(), 10.to_bigint().unwrap())
    );
}
