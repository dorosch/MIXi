use std::fmt;

trait Data<T> {
  fn read(&self) -> T;
  fn write(&mut self, number: T);
}

trait Signed {
  fn read_sign(&self) -> bool;
  fn write_sign(&mut self, sign: bool);
}

struct Register {
  data: u16,
  positive: bool,
}

impl Register {
  const MASK: u16 = 0b0000_1111_1111_1111;

  fn new(number: u16) -> Self {
    Self {
      positive: true,
      data: number & Self::MASK,
    }
  }
}

impl Default for Register {
  fn default() -> Self {
    Self::new(0)
  }
}

impl Data<u16> for Register {
  fn read(&self) -> u16 {
    self.data & Self::MASK
  }

  fn write(&mut self, number: u16) {
    self.data = number & Self::MASK;
  }
}

impl Signed for Register {
  fn read_sign(&self) -> bool {
    self.positive
  }

  fn write_sign(&mut self, sign: bool) {
    self.positive = sign;
  }
}

impl fmt::Display for Register {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.positive {
      write!(f, "+{}", self.data)
    } else {
      write!(f, "-{}", self.data)
    }
  }
}

#[derive(Clone, Copy)]
struct Word {
  data: u32,
  positive: bool,
}

impl Word {
  const MASK: u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111;

  fn new(number: u32) -> Self {
    Self {
      positive: true,
      data: number & Self::MASK,
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

  fn write(&mut self, number: u32) {
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

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
  command: u32,
  modifier: u32,
  index: u32,
  address: u32,
  sign: u32,
}

impl Instruction {
  const COMMAND_MASK: u32 = 0b0000_0000_0011_1111;
  const MODIFIER_MASK: u32 = 0b0000_1111_1100_0000;
  const INDEX_MASK: u32 = 0b1111_1100_0000_0000;
  const ADDRESS_MASK: u32 = 0b0011_1111_1111_1100_0000_0000_0000_0000;

  fn new(command: u32, modifier: u32, index: u32, address: u32, sign: u32) -> Self {
    Self {
      sign,
      command,
      modifier,
      index,
      address,
    }
  }

  fn pack(&self) -> u32 {
    (self.command & 0b111111)
      | ((self.modifier & 0b111111) << 6)
      | ((self.index & 0b111111) << 12)
      | ((self.address & 0b1111111111111) << 18)
      | (self.sign << 31)
  }

  fn unpack(word: u32) -> Self {
    Self {
      command: word & Self::COMMAND_MASK,
      modifier: (word & Self::MODIFIER_MASK) >> 6,
      index: (word & Self::INDEX_MASK) >> 12,
      address: (word & Self::ADDRESS_MASK) >> 18,
      sign: (word >> 31) & 1
    }
  }
}

struct Program {
  instructions: Vec<Instruction>
}

impl Program {
  fn new() -> Self {
    Self {
      instructions: Vec::new()
    }
  }

  fn add(&mut self, instruction: Instruction) {
    self.instructions.push(instruction);
  }
}

#[derive(Debug)]
enum Compare {
  None,
  Less,
  Equal,
  Greater,
}

struct Computer {
  overflow: bool,
  comparison: Compare,
  memory: [Word; 4000],
  a: Word,
  x: Word,
  i1: Register,
  i2: Register,
  i3: Register,
  i4: Register,
  i5: Register,
  i6: Register,
}

impl Computer {
  fn new() -> Self {
    Self {
      overflow: false,
      comparison: Compare::None,
      memory: [Word::default(); 4000],
      a: Word::default(),
      x: Word::default(),
      i1: Register::default(),
      i2: Register::default(),
      i3: Register::default(),
      i4: Register::default(),
      i5: Register::default(),
      i6: Register::default(),
    }
  }

  fn load(&mut self, program: Program) {
    for (index, instruction) in program.instructions.iter().enumerate() {
      self.memory[index] = Word::new(instruction.pack());
    }
  }

  fn execute(&mut self) {
    for (index, word) in self.memory.iter().enumerate() {
      let instruction = Instruction::unpack(word.read());

      match instruction.command {
        0 => {
          // NOOP
          continue
        },
        8 => {
          // LDA
          self.a = Word::new(instruction.modifier)
        },
        15 => {
          // LDX
          self.x = Word::new(instruction.modifier)
        },
        _ => unimplemented!("Unknown command")
      }
    }
  }
}

impl fmt::Display for Computer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "Memory:")?;
    for (i, word) in self.memory.iter().enumerate().rev() {
      write!(f, "{:04X}: ", i)?;
      writeln!(f, "{}", word)?;
    }

    writeln!(f, "Overflow: {}", self.overflow)?;
    writeln!(f, "Comparison: {:?}", self.comparison)?;
    writeln!(f, "A: {}", self.a)?;
    writeln!(f, "X: {}", self.x)?;
    writeln!(f, "I1: {}", self.i1)?;
    writeln!(f, "I2: {}", self.i2)?;
    writeln!(f, "I3: {}", self.i3)?;
    writeln!(f, "I4: {}", self.i4)?;
    writeln!(f, "I5: {}", self.i5)?;
    write!(f, "I6: {}", self.i6)
  }
}

fn main() {
  let mut computer = Computer::new();
  let mut program = Program::new();
  program.add(Instruction::new(8, 3, 2, 0, 1));
  program.add(Instruction::new(8, 11, 2, 0, 1));
  program.add(Instruction::new(8, 11, 0, 0, 1));
  program.add(Instruction::new(8, 5, 0, 0, 1));
  program.add(Instruction::new(8, 5, 4, 0, 0));

  computer.load(program);
  println!("{}", computer);

  computer.execute();
}

#[cfg(test)]
mod tests {
  use super::*;

  // Word tests
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

  #[test]
  fn test_write_number() {
    let mut word = Word::default();
    word.write(1073741824);
    assert_eq!(word.read(), 1073741824);
  }

  // Register tests
  #[test]
  fn test_read_write_register() {
    let register = Register::new(4095);

    assert_eq!(register.read(), 4095);
  }

  #[test]
  fn test_read_sign_register() {
    let mut register = Register::default();
    let expected_sign = true;

    register.write_sign(expected_sign);
    assert_eq!(register.read_sign(), expected_sign);
  }

  #[test]
  fn test_default_sign_register() {
    let register = Register::default();
    let expected_sign = true;

    assert_eq!(register.read_sign(), expected_sign);
  }
}
