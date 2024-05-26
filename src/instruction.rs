#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
  pub command: u32,
  pub modifier: u32,
  pub index: u32,
  pub address: u32,
  pub sign: u32,
}

impl Instruction {
  const COMMAND_MASK: u32 = 0b0000_0000_0011_1111;
  const MODIFIER_MASK: u32 = 0b0000_1111_1100_0000;
  const INDEX_MASK: u32 = 0b1111_1100_0000_0000;
  const ADDRESS_MASK: u32 = 0b0011_1111_1111_1100_0000_0000_0000_0000;

  pub fn new(command: u32, modifier: u32, index: u32, address: u32, sign: u32) -> Self {
    Self {
      sign,
      command,
      modifier,
      index,
      address,
    }
  }

  pub fn pack(&self) -> u32 {
    (self.command & 0b111111)
      | ((self.modifier & 0b111111) << 6)
      | ((self.index & 0b111111) << 12)
      | ((self.address & 0b1111111111111) << 18)
      | (self.sign << 31)
  }

  pub fn unpack(word: u32) -> Self {
    Self {
      command: word & Self::COMMAND_MASK,
      modifier: (word & Self::MODIFIER_MASK) >> 6,
      index: (word & Self::INDEX_MASK) >> 12,
      address: (word & Self::ADDRESS_MASK) >> 18,
      sign: (word >> 31) & 1,
    }
  }
}
