use std::fmt;

use crate::{Data, Signed};

#[derive(Clone, Copy)]
pub struct Word {
  data: u32,
  positive: bool,
}

impl Word {
  const MASK: u32 = 0b0011_1111_1111_1111_1111_1111_1111_1111;

  pub fn new(number: u32) -> Self {
    Self {
      positive: true,
      data: number & Self::MASK,
    }
  }

  pub fn get_byte(&self, index: usize) -> u8 {
    assert!(index <= 5);

    ((self.data >> (6 * index)) & 0b111111) as u8
  }

  pub fn read_part(&self, modifier: u32) -> Self {
    let left = modifier / 8;
    let right = modifier - left * 8;
    let mut result = 0;

    for index in left..right {
      result <<= 6;
      result |= self.get_byte(index as usize) as u32;
    }

    Self {
      data: result,
      positive: if left == 0 { self.positive } else { true },
    }
  }
}

impl Default for Word {
  fn default() -> Self {
    Self::new(0)
  }
}

impl Data<u32> for Word {
  fn read(&self) -> u32 {
    self.data & Self::MASK
  }

  fn read_data(&self) -> u32 {
    todo!("Unimplemented")
  }

  fn write(&mut self, number: u32, sign: bool) {
    // TODO: Use sign
    self.data = number & Self::MASK;
  }

  fn write_data(&mut self, number: u32) {
    self.data = number & Self::MASK;
  }
}

impl Signed for Word {
  fn read_sign(&self) -> bool {
    self.positive
  }

  fn write_sign(&mut self, sign: bool) {
    self.positive = sign;
  }
}

impl fmt::Display for Word {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}{:06} {:06} {:06} {:06} {:06}",
      if self.positive { "+" } else { "-" },
      self.data & 0b0011_1111,
      self.data & 0b1111_1100_0000,
      self.data & 0b0011_1111_0000_0000_0000,
      self.data & 0b1111_1100_0000_0000_0000_0000,
      self.data & 0b0011_1111_0000_0000_0000_0000_0000_0000,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_sign() {
    let word = Word::default();
    assert!(word.read_sign());
  }

  #[test]
  fn test_write_sign() {
    let mut word = Word::default();
    word.write_sign(false);
    assert!(!word.read_sign());
  }

  #[test]
  fn test_read_number() {
    let word = Word::new(1073741824);
    assert_eq!(word.read(), 1073741824);
  }
}
