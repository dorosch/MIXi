use std::fmt;

use crate::{Data, Signed};

/// Represents a word with a 30-bit value and a sign bit
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Word {
  data: u32,
}

impl Word {
  const BYTES: usize = 5;

  #[rustfmt::skip]
  const SIGN_MASK:  u32 = 0b0100_0000_0000_0000_0000_0000_0000_0000;

  #[rustfmt::skip]
  const DATA_MASK:  u32 = 0b0011_1111_1111_1111_1111_1111_1111_1111;

  #[rustfmt::skip]
  const VALUE_MASK: u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111;

  pub fn new(number: u32, sign: Option<bool>) -> Self {
    let mut data = number & Self::DATA_MASK;

    if let Some(sign) = sign {
      if sign {
        data |= Self::SIGN_MASK;
      }
    }

    Self { data }
  }
}

impl Default for Word {
  fn default() -> Self {
    Self::new(0, None)
  }
}

impl From<u32> for Word {
  fn from(value: u32) -> Self {
    Self {
      data: value & Word::VALUE_MASK,
    }
  }
}

impl Data<u32> for Word {
  fn read(&self) -> u32 {
    self.data & Self::VALUE_MASK
  }

  fn read_data(&self) -> u32 {
    self.data & Self::DATA_MASK
  }

  fn read_with_modifier(&self, modifier: u32) -> u32 {
    let mut result: u32 = 0;
    let (left, right) = Self::split_modifier(modifier);

    assert!(right <= Self::BYTES as u32);

    for index in left..=right {
      result <<= 6;
      result |= self.get_byte(index as usize) as u32;
    }

    result
  }

  fn write(&mut self, number: u32, sign: bool) {
    self.data = (number & Self::DATA_MASK) | if sign { Self::SIGN_MASK } else { 0 };
  }

  fn write_data(&mut self, number: u32) {
    self.data = (number & Self::DATA_MASK) | (self.data & Self::SIGN_MASK);
  }

  fn get_byte(&self, index: usize) -> u8 {
    assert!(index <= Self::BYTES);

    ((self.data >> ((Self::BYTES - index) * 6)) & 0b111111) as u8
  }
}

impl Signed for Word {
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

impl fmt::Display for Word {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.read_sign() {
      write!(f, "+")?
    } else {
      write!(f, "-")?
    }

    write!(
      f,
      "{:06} {:06} {:06} {:06}",
      self.read_with_modifier(12),
      self.read_with_modifier(33),
      self.read_with_modifier(44),
      self.read_with_modifier(55),
    )
  }
}

#[cfg(test)]
mod tests {
  use rstest::rstest;
  use rstest_reuse::{self, *};

  use super::*;
  use crate::tests::split_modifier_cases;

  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000_0000_0000_0000_0000, false)]
  #[case(0b0100_0000_0000_0000_0000_0000_0000_0000, true)]
  #[case(0b0011_1111_1111_1111_1111_1111_1111_1111, false)]
  #[case(0b0111_1111_1111_1111_1111_1111_1111_1111, true)]
  #[case(0b1011_1111_1111_1111_1111_1111_1111_1111, false)]
  #[case(0b1111_1111_1111_1111_1111_1111_1111_1111, true)]
  fn sign_cases(#[case] number: u32, #[case] sign: bool) {}

  #[rustfmt::skip]
  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0000_0000)]
  #[case(0b0100_0000_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0000_0000)]
  #[case(0b1100_0000_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0000_0000)]
  #[case(0b0011_1111_1111_1111_1111_1111_1111_1111, 0b0011_1111_1111_1111_1111_1111_1111_1111)]
  #[case(0b0111_1111_1111_1111_1111_1111_1111_1111, 0b0011_1111_1111_1111_1111_1111_1111_1111)]
  #[case(0b1111_1111_1111_1111_1111_1111_1111_1111, 0b0011_1111_1111_1111_1111_1111_1111_1111)]
  fn data_without_sign_cases(#[case] number: u32, #[case] expected: u32) {}

  #[rustfmt::skip]
  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0000_0000, false)]
  #[case(0b0000_0000_0000_0000_0000_0000_0000_0000, 0b0100_0000_0000_0000_0000_0000_0000_0000, true)]
  #[case(0b0010_0000_0000_0000_0000_0000_0000_0000, 0b0010_0000_0000_0000_0000_0000_0000_0000, false)]
  #[case(0b0010_0000_0000_0000_0000_0000_0000_0000, 0b0110_0000_0000_0000_0000_0000_0000_0000, true)]
  #[case(0b0111_1111_1111_1111_1111_1111_1111_1111, 0b0011_1111_1111_1111_1111_1111_1111_1111, false)]
  #[case(0b0111_1111_1111_1111_1111_1111_1111_1111, 0b0111_1111_1111_1111_1111_1111_1111_1111, true)]
  #[case(0b1111_1111_1111_1111_1111_1111_1111_1111, 0b0011_1111_1111_1111_1111_1111_1111_1111, false)]
  #[case(0b1111_1111_1111_1111_1111_1111_1111_1111, 0b0111_1111_1111_1111_1111_1111_1111_1111, true)]
  fn data_with_sign_cases(#[case] number: u32, #[case] expected: u32, #[case] sign: bool) {}

  #[rustfmt::skip]
  #[template]
  #[rstest]
  #[case(0b0100_0000_0000_0000_0000_0000_0000_0000, 0b0000_0001, true, 0)]
  #[case(0b0000_0000_0000_0000_0000_0000_0000_0000, 0b0000_0000, false, 0)]
  #[case(0b0111_1111_0000_0000_0000_0000_0000_0000, 0b0011_1111, true, 1)]
  #[case(0b0011_1111_1100_0000_0000_0000_0000_0000, 0b0011_1111, false, 1)]
  #[case(0b0100_0000_1111_1100_0000_0000_0000_0000, 0b0011_1111, true, 2)]
  #[case(0b0000_0000_1111_1111_0000_0000_0000_0000, 0b0011_1111, false, 2)]
  #[case(0b0100_0000_0000_0011_1111_0000_0000_0000, 0b0011_1111, true, 3)]
  #[case(0b0000_0000_0000_0011_1111_0000_0000_0000, 0b0011_1111, false, 3)]
  #[case(0b0100_0000_0000_0000_0000_1111_1100_0000, 0b0011_1111, true, 4)]
  #[case(0b0000_0000_0000_0000_0011_1111_1100_0000, 0b0011_1111, false, 4)]
  #[case(0b0100_0000_0000_0000_0000_0000_0011_1111, 0b0011_1111, true, 5)]
  #[case(0b0000_0000_0000_0000_0000_0000_1111_1111, 0b0011_1111, false, 5)]
  fn get_byte_cases(
    #[case] number: u32,
    #[case] expected: u8,
    #[case] sign: bool,
    #[case] index: usize,
  ) {
  }

  #[rustfmt::skip]
  #[template]
  #[rstest]
  #[case(0b0000_0000_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0000_0000, false, 0)]
  #[case(0b0100_0000_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0000_0001, true, 0)]
  #[case(0b0011_1111_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0011_1111, false, 1)]
  #[case(0b0011_1111_1111_1100_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0011_1111, false, 1)]
  #[case(0b0010_1010_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0110_1010, true, 1)]
  #[case(0b0110_1010_0000_0000_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0110_1010, true, 1)]
  #[case(0b0011_1111_1111_1100_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_1111_1111_1111, false, 2)]
  #[case(0b0111_1111_1111_1100_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_1111_1111_1111, false, 2)]
  #[case(0b0011_1111_1111_1100_0000_0000_0000_0000, 0b0000_0000_0000_0000_0001_1111_1111_1111, true, 2)]
  #[case(0b0111_1111_1111_1100_0000_0000_0000_0000, 0b0000_0000_0000_0000_0001_1111_1111_1111, true, 2)]
  #[case(0b0011_1111_1111_1111_1111_1111_1111_1111, 0b0011_1111_1111_1111_1111_1111_1111_1111, false, 5)]
  #[case(0b0111_1111_1111_1111_1111_1111_1111_1111, 0b0011_1111_1111_1111_1111_1111_1111_1111, false, 5)]
  #[case(0b0111_1111_1111_1111_1111_1111_1111_1111, 0b0111_1111_1111_1111_1111_1111_1111_1111, true, 5)]
  #[case(0b1111_1111_1111_1111_1111_1111_1111_1111, 0b0111_1111_1111_1111_1111_1111_1111_1111, true, 5)]
  #[case(0b0011_1111_1111_1100_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_1111_1111_1111, false, 12)]
  #[case(0b0111_1111_1111_1100_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_1111_1111_1111, true, 12)]
  #[case(0b0000_0001_1111_1110_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0011_1111, false, 22)]
  #[case(0b0000_0001_1111_1110_0000_0000_0000_0000, 0b0000_0000_0000_0000_0000_0000_0011_1111, true, 22)]
  #[case(0b0000_0000_0000_0011_1111_1111_1100_0000, 0b0000_0000_0000_0000_0000_1111_1111_1111, false, 34)]
  #[case(0b0000_0000_0000_0111_1111_1111_1110_0000, 0b0000_0000_0000_0000_0000_1111_1111_1111, true, 34)]
  #[case(0b0000_0000_0000_0000_0000_1111_1111_1111, 0b0000_0000_0000_0000_0000_1111_1111_1111, false, 45)]
  #[case(0b0000_0000_0000_0000_0001_1111_1111_1111, 0b0000_0000_0000_0000_0000_1111_1111_1111, true, 45)]
  #[case(0b0000_0000_0000_0000_0000_0000_0011_1111, 0b0000_0000_0000_0000_0000_0000_0011_1111, false, 55)]
  #[case(0b0000_0000_0000_0000_0000_0000_0111_1111, 0b0000_0000_0000_0000_0000_0000_0011_1111, true, 55)]
  fn read_with_modifier_cases(
    #[case] number: u32,
    #[case] expected: u32,
    #[case] sign: bool,
    #[case] modifier: u32,
  ) {
  }

  #[test]
  fn test_default() {
    assert!(!Word::default().read_sign());
    assert_eq!(Word::default().read_data(), 0);
  }

  #[apply(data_with_sign_cases)]
  fn test_word_from_u32(number: u32, expected: u32, sign: bool) {
    assert_eq!(Word::from(expected), Word::new(number, Some(sign)));
  }

  #[apply(data_with_sign_cases)]
  fn test_read(number: u32, expected: u32, sign: bool) {
    assert_eq!(Word::new(number, Some(sign)).read(), expected);
  }

  #[apply(data_without_sign_cases)]
  fn test_read_data(number: u32, expected: u32) {
    assert_eq!(Word::new(number, Some(true)).read_data(), expected);
  }

  #[apply(read_with_modifier_cases)]
  fn test_read_with_modifier(number: u32, expected: u32, sign: bool, modifier: u32) {
    assert_eq!(
      Word::new(number, Some(sign)).read_with_modifier(modifier),
      expected
    );
  }

  #[apply(data_with_sign_cases)]
  fn test_write(number: u32, expected: u32, sign: bool) {
    let mut word = Word::default();
    word.write(number, sign);

    assert_eq!(word.read(), expected);
  }

  #[apply(data_without_sign_cases)]
  fn test_write_data(number: u32, expected: u32) {
    let mut word = Word::default();
    word.write_data(number);

    assert_eq!(word.read(), expected);
  }

  #[apply(sign_cases)]
  fn test_read_sign(number: u32, sign: bool) {
    assert_eq!(Word::new(number, Some(sign)).read_sign(), sign);
  }

  #[apply(data_with_sign_cases)]
  fn test_write_sign(number: u32, expected: u32, sign: bool) {
    let mut word = Word::new(number, Some(sign));
    word.write_sign(!sign);

    assert_eq!(word.read_sign(), !sign);
    assert_eq!(word.read_data(), expected & Word::DATA_MASK);
  }

  #[apply(get_byte_cases)]
  fn test_get_byte(number: u32, expected: u8, sign: bool, index: usize) {
    assert_eq!(Word::new(number, Some(sign)).get_byte(index), expected);
  }

  #[apply(split_modifier_cases)]
  fn test_split_modifier(modifier: u32, expected: (u32, u32)) {
    assert_eq!(Word::split_modifier(modifier), expected);
  }
}
