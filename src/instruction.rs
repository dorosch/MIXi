use crate::{word::Word, Data};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Command {
  Noop = 0,
  Lda = 8,
}

impl From<u32> for Command {
  fn from(value: u32) -> Self {
    match value {
      0 => Self::Noop,
      8 => Self::Lda,
      _ => unreachable!("Command not implemented"),
    }
  }
}

impl From<Command> for u32 {
  fn from(value: Command) -> Self {
    match value {
      Command::Noop => 0,
      Command::Lda => 8,
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Instruction {
  pub sign: bool,
  pub address: u32,
  pub index: u32,
  pub modifier: u32,
  pub command: Command,
}

impl Instruction {
  #[rustfmt::skip]
  const COMMAND_MASK:  u32 = 0b0000_0000_0000_0000_0000_0000_0011_1111;

  #[rustfmt::skip]
  const MODIFIER_MASK: u32 = 0b0000_0000_0000_0000_0000_1111_1100_0000;

  #[rustfmt::skip]
  const INDEX_MASK:    u32 = 0b0000_0000_0000_0011_1111_0000_0000_0000;

  #[rustfmt::skip]
  const ADDRESS_MASK:  u32 = 0b0011_1111_1111_1100_0000_0000_0000_0000;

  #[rustfmt::skip]
  const SIGN_MASK:     u32 = 0b0100_0000_0000_0000_0000_0000_0000_0000;

  pub fn new(sign: bool, address: u32, index: u32, modifier: u32, command: Command) -> Self {
    Self {
      sign,
      address,
      index,
      modifier,
      command,
    }
  }
}

impl From<Instruction> for u32 {
  fn from(value: Instruction) -> u32 {
    (u32::from(value.command) & 0b111111)
      | ((value.modifier & 0b111111) << 6)
      | ((value.index & 0b111111) << 12)
      | ((value.address & 0b1111111111111) << 18)
      | ((value.sign as u32) << 30)
  }
}

impl From<u32> for Instruction {
  fn from(value: u32) -> Self {
    Self {
      command: Command::from(value & Self::COMMAND_MASK),
      modifier: (value & Self::MODIFIER_MASK) >> 6,
      index: (value & Self::INDEX_MASK) >> 12,
      address: (value & Self::ADDRESS_MASK) >> 18,
      sign: (value & Self::SIGN_MASK) != 0,
    }
  }
}

impl From<Instruction> for Word {
  fn from(value: Instruction) -> Self {
    let sign = Some(value.sign);

    Word::new(u32::from(value), sign)
  }
}

// TODO: Add tests
impl From<&Instruction> for Word {
  fn from(value: &Instruction) -> Self {
    Word::from(*value)
  }
}

impl From<Word> for Instruction {
  fn from(value: Word) -> Self {
    Self {
      sign: value.read_with_modifier(0) != 0,
      address: value.read_with_modifier(12),
      index: value.read_with_modifier(33),
      modifier: value.read_with_modifier(44),
      command: Command::from(value.read_with_modifier(55)),
    }
  }
}

#[cfg(test)]
mod tests {
  use rstest::rstest;
  use rstest_reuse::{self, *};

  use crate::Data;

  use super::*;

  #[template]
  #[rstest]
  #[case(Command::Noop, 0)]
  #[case(Command::Lda, 8)]
  fn from_command_cases(#[case] command: Command, #[case] expected: u32) {}

  #[rustfmt::skip]
  #[template]
  #[rstest]
  #[case(true, 2000, 0, 0, Command::Lda, 0b0101_1111_0100_0000_0000_0000_0000_1000)]
  #[case(false, 2000, 0, 0, Command::Lda, 0b0001_1111_0100_0000_0000_0000_0000_1000)]
  fn from_instruction_cases(
    #[case] sign: bool,
    #[case] address: u32,
    #[case] index: u32,
    #[case] modifier: u32,
    #[case] command: Command,
    #[case] expected: u32,
  ) {
  }

  #[apply(from_command_cases)]
  fn test_u32_from_command(command: Command, expected: u32) {
    assert_eq!(Command::from(expected), command);
  }

  #[apply(from_command_cases)]
  fn test_command_from_u32(command: Command, expected: u32) {
    assert_eq!(u32::from(command), expected);
  }

  #[apply(from_instruction_cases)]
  fn test_u32_from_instruction(
    sign: bool,
    address: u32,
    index: u32,
    modifier: u32,
    command: Command,
    expected: u32,
  ) {
    assert_eq!(
      u32::from(Instruction::new(sign, address, index, modifier, command)),
      expected
    );
  }

  #[apply(from_instruction_cases)]
  fn test_instruction_from_u32(
    sign: bool,
    address: u32,
    index: u32,
    modifier: u32,
    command: Command,
    expected: u32,
  ) {
    assert_eq!(
      Instruction::from(expected),
      Instruction::new(sign, address, index, modifier, command)
    );
  }

  #[apply(from_instruction_cases)]
  fn test_validate_each_instruction_field(
    sign: bool,
    address: u32,
    index: u32,
    modifier: u32,
    command: Command,
    expected: u32,
  ) {
    let instruction = Instruction::from(expected);
    let word = Word::from(instruction);

    assert_eq!(word.read_with_modifier(0), sign as u32);
    assert_eq!(word.read_with_modifier(12), address);
    assert_eq!(word.read_with_modifier(33), index);
    assert_eq!(word.read_with_modifier(44), modifier);
    assert_eq!(word.read_with_modifier(55), u32::from(command));
  }

  #[apply(from_instruction_cases)]
  fn test_word_from_instruction(
    sign: bool,
    address: u32,
    index: u32,
    modifier: u32,
    command: Command,
    expected: u32,
  ) {
    let instruction = Instruction::new(sign, address, index, modifier, command);

    assert_eq!(Word::from(instruction).read(), expected);
  }

  #[apply(from_instruction_cases)]
  fn test_instruction_from_word(
    sign: bool,
    address: u32,
    index: u32,
    modifier: u32,
    command: Command,
    expected: u32,
  ) {
    let word = Word::new(expected, Some(sign));

    assert_eq!(
      Instruction::from(word),
      Instruction::new(sign, address, index, modifier, command)
    );
  }
}
