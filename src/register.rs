use std::fmt;

use crate::{Data, Signed};

/// Represents a register with a 12-bit value and a sign bit
pub struct Register {
  data: u16,
}

impl Register {
  /// Mask for the data portion (first 12 bits)
  const DATA_MASK: u16 = 0b0000_1111_1111_1111;

  /// Mask for the sign bit (13th bit)
  const SIGN_MASK: u16 = 0b0001_0000_0000_0000;

  /// Mask for the value with the sign bit
  const VALUE_MASK: u16 = 0b0001_1111_1111_1111;

  /// Creates a new register with the given value and sign
  pub fn new(number: u16, sign: Option<bool>) -> Self {
    let mut data = number & Self::DATA_MASK;
    if let Some(sign) = sign {
      if sign {
        data |= Self::SIGN_MASK;
      }
    }

    Self { data }
  }
}

impl Default for Register {
  fn default() -> Self {
    Self::new(0, None)
  }
}

impl Data<u16> for Register {
  fn read(&self) -> u16 {
    self.data & Self::VALUE_MASK
  }

  fn read_data(&self) -> u16 {
    self.data & Self::DATA_MASK
  }

  fn read_with_modifier(&self, modifier: u16) -> u16 {
    let (left, right) = Self::split_modifier(modifier as u32);

    assert!(left <= right && right <= 2);

    match (left, right) {
      (0, 0) => (self.data & Self::SIGN_MASK),
      (0, 2) => self.read(),
      _ => {
        let mut result: u16 = 0;

        for index in left..=right {
          result <<= 6;
          result |= self.get_byte(index as usize) as u16;
        }
  
        result
      }
    }
  }

  fn write(&mut self, number: u16, sign: bool) {
    self.data = (number & Self::DATA_MASK) | if sign { Self::SIGN_MASK } else { 0 };
  }

  fn write_data(&mut self, number: u16) {
    self.data = (number & Self::DATA_MASK) | (self.data & Self::SIGN_MASK);
  }

  fn get_byte(&self, index: usize) -> u8 {
    assert!(index <= 2);

    ((self.data >> ((2 - index) * 6)) & 0b111111) as u8
  }
}

impl Signed for Register {
  fn read_sign(&self) -> bool {
    (self.data & Self::SIGN_MASK) != 0
  }

  fn write_sign(&mut self, sign: bool) {
    if sign {
      self.data |= Self::SIGN_MASK;
    } else {
      self.data &= !Self::SIGN_MASK;
    }
  }
}

impl fmt::Display for Register {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.read_sign() {
      write!(f, "+")?
    } else {
      write!(f, "-")?
    }

    write!(f, "{}", self.read_data())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;
  use rstest_reuse::{self, *};
  use crate::tests::split_modifier_cases;

  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000, false)]
  #[case(0b0001_0000_0000_0000, true)]
  #[case(0b0000_1000_0000_0000, false)]
  #[case(0b0001_1000_0000_0000, true)]
  #[case(0b0000_1111_1111_1111, false)]
  #[case(0b0001_1111_1111_1111, true)]
  #[case(0b1110_1111_1111_1111, false)]
  #[case(0b1111_1111_1111_1111, true)]
  fn sign_cases(#[case] number: u16, #[case] sign: bool) {}

  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000, 0b0000_0000_0000_0000)]
  #[case(0b0001_0000_0000_0000, 0b0000_0000_0000_0000)]
  #[case(0b1111_0000_0000_0000, 0b0000_0000_0000_0000)]
  #[case(0b0000_1111_1111_1111, 0b0000_1111_1111_1111)]
  #[case(0b0001_1111_1111_1111, 0b0000_1111_1111_1111)]
  #[case(0b1111_1111_1111_1111, 0b0000_1111_1111_1111)]
  fn data_without_sign_cases(#[case] number: u16, #[case] expected: u16) {}

  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000, 0b0000_0000_0000_0000, false)]
  #[case(0b0000_0000_0000_0000, 0b0001_0000_0000_0000, true)]
  #[case(0b0000_1000_0000_0000, 0b0000_1000_0000_0000, false)]
  #[case(0b0000_1000_0000_0000, 0b0001_1000_0000_0000, true)]
  #[case(0b0001_1111_1111_1111, 0b0000_1111_1111_1111, false)]
  #[case(0b0001_1111_1111_1111, 0b0001_1111_1111_1111, true)]
  #[case(0b1111_1111_1111_1111, 0b0000_1111_1111_1111, false)]
  #[case(0b1111_1111_1111_1111, 0b0001_1111_1111_1111, true)]
  fn data_with_sign_cases(#[case] number: u16, #[case] expected: u16, #[case] sign: bool) {}

  #[template]
  #[rstest]
  #[case(0b0001_0000_0000_0000, 0b0000_0001, true, 0)]
  #[case(0b0000_0000_0000_0000, 0b0000_0000, false, 0)]
  #[case(0b0000_1111_1100_0000, 0b0011_1111, false, 1)]
  #[case(0b0000_1111_1100_0000, 0b0011_1111, false, 1)]
  #[case(0b0000_1111_1111_1111, 0b0011_1111, false, 1)]
  #[case(0b0000_1111_1111_1111, 0b0011_1111, false, 2)]
  #[case(0b0000_0000_0011_1111, 0b0011_1111, false, 2)]
  #[case(0b0000_0000_0000_1111, 0b0000_1111, false, 2)]
  fn get_byte_cases(#[case] number: u16, #[case] expected: u8, #[case] sign: bool, #[case] index: usize) {}

  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000, 0b0000_0000_0000_0000, false, 0)]
  #[case(0b0001_0000_0000_0000, 0b0001_0000_0000_0000, true, 0)]
  #[case(0b0000_1111_1100_0000, 0b0000_0000_0011_1111, false, 1)]
  #[case(0b0000_1111_1111_1111, 0b0000_0000_0011_1111, false, 1)]
  #[case(0b0000_1111_1111_1111, 0b0000_0000_0111_1111, true, 1)]
  #[case(0b0001_1111_1111_1111, 0b0000_0000_0111_1111, true, 1)]
  #[case(0b0000_1111_1111_1111, 0b0000_1111_1111_1111, false, 2)]
  #[case(0b0001_1111_1111_1111, 0b0000_1111_1111_1111, false, 2)]
  #[case(0b0000_1111_1111_1111, 0b0001_1111_1111_1111, true, 2)]
  #[case(0b0001_1111_1111_1111, 0b0001_1111_1111_1111, true, 2)]
  #[case(0b0000_1111_1111_1111, 0b0000_1111_1111_1111, false, 12)]
  #[case(0b0001_1111_1111_1111, 0b0000_1111_1111_1111, true, 12)]
  #[case(0b0000_0000_0011_1111, 0b0000_0000_0011_1111, false, 22)]
  #[case(0b0001_0000_0011_1111, 0b0000_0000_0011_1111, false, 22)]
  #[case(0b0001_1111_0011_0000, 0b0000_0000_0011_0000, true, 22)]
  #[case(0b0000_1111_0011_0001, 0b0000_0000_0011_0001, false, 22)]
  fn read_with_modifier_cases(#[case] number: u16, #[case] expected: u16, #[case] sign: bool, #[case] modifier: u16) {}

  #[test]
  fn test_default() {
    assert!(!Register::default().read_sign());
    assert_eq!(Register::default().read_data(), 0);
  }

  #[apply(data_with_sign_cases)]
  fn test_read(number: u16, expected: u16, sign: bool) {
    assert_eq!(Register::new(number, Some(sign)).read(), expected);
  }

  #[apply(data_without_sign_cases)]
  fn test_read_data(number: u16, expected: u16) {
    assert_eq!(Register::new(number, Some(true)).read_data(), expected);
  }

  #[apply(read_with_modifier_cases)]
  fn test_read_with_modifier(number: u16, expected: u16, sign: bool, modifier: u16) {
    assert_eq!(Register::new(number, Some(sign)).read_with_modifier(modifier), expected);
  }

  #[apply(data_with_sign_cases)]
  fn test_write(number: u16, expected: u16, sign: bool) {
    let mut register = Register::default();
    register.write(number, sign);

    assert_eq!(register.read(), expected);
  }

  #[apply(data_without_sign_cases)]
  fn test_write_data(number: u16, expected: u16) {
    let mut register = Register::default();
    register.write_data(number);

    assert_eq!(register.read(), expected);
  }

  #[apply(sign_cases)]
  fn test_read_sign(number: u16, sign: bool) {
    assert_eq!(Register::new(number, Some(sign)).read_sign(), sign);
  }

  #[apply(data_with_sign_cases)]
  fn test_write_sign(number: u16, expected: u16, sign: bool) {
    let mut register = Register::new(number, Some(sign));
    register.write_sign(!sign);

    assert_eq!(register.read_sign(), !sign);
    assert_eq!(register.read_data(), expected & Register::DATA_MASK);
  }

  #[apply(get_byte_cases)]
  fn test_get_byte(number: u16, expected: u8, sign: bool, index: usize) {
    assert_eq!(Register::new(number, Some(sign)).get_byte(index), expected);
  }

  #[apply(split_modifier_cases)]
  fn test_split_modifier(modifier: u32, expected: (u32, u32)) {
    assert_eq!(Register::split_modifier(modifier), expected);
  }
}
