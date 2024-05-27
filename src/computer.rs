use std::fmt;

use crate::{instruction::Instruction, program::Program, register::Register, word::Word, Data};

#[derive(Debug)]
pub enum Compare {
  None,
  Less,
  Equal,
  Greater,
}

pub struct Computer {
  pub overflow: bool,
  pub comparison: Compare,
  pub memory: [Word; 4000],
  pub a: Word,
  pub x: Word,
  pub i1: Register,
  pub i2: Register,
  pub i3: Register,
  pub i4: Register,
  pub i5: Register,
  pub i6: Register,
}

impl Computer {
  pub fn new() -> Self {
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

  pub fn load(&mut self, program: Program) {
    for (index, instruction) in program.instructions.iter().enumerate() {
      self.memory[index] = Word::new(instruction.pack(), None);
    }
  }

  pub fn execute(&mut self) {
    for (index, word) in self.memory.iter().enumerate() {
      let instruction = Instruction::unpack(word.read());

      match instruction.command {
        0 => continue,
        8 => {
          let word = self.memory[instruction.address as usize];
          // self.a = word.read_part(instruction.modifier);
        }
        _ => unimplemented!("Unknown command"),
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
