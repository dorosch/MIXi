mod computer;
mod instruction;
mod program;
mod register;
mod word;

#[cfg(test)]
use rstest_reuse;

use computer::Computer;
use instruction::Instruction;
use program::Program;

/// Trait for reading and writing data
trait Data<T> {
  /// Reads the value including the sign
  fn read(&self) -> T;

  /// Reads the value without the sign
  fn read_data(&self) -> T;

  /// Reads the value by modifier
  fn read_with_modifier(&self, modifier: T) -> T;

  /// Writes the value, including the sign
  fn write(&mut self, number: T, sign: bool);

  /// Writes the value, without the sign
  fn write_data(&mut self, number: T);

  fn get_byte(&self, index: usize) -> u8;

  /// Get left and right parts from modifier
  fn split_modifier(modifier: u32) -> (u32, u32) {
    let (left, right) = (modifier / 10, modifier % 10);

    assert!(left <= right && right <= 5);

    (left, right)
  }
}

/// Trait for reading and writing the sign
trait Signed {
  /// Reads the sign (true if positive, false if negative)
  fn read_sign(&self) -> bool;

  /// Writes the sign (true for positive, false for negative)
  fn write_sign(&mut self, sign: bool);
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
  computer.execute();

  println!("{}", computer);
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest_reuse::{self, *};

  #[template]
  #[rstest]
  #[case(0, (0, 0))]
  #[case(1, (0, 1))]
  #[case(5, (0, 5))]
  #[case(13, (1, 3))]
  #[case(15, (1, 5))]
  #[case(24, (2, 4))]
  #[case(45, (4, 5))]
  #[case(55, (5, 5))]
  fn split_modifier_cases(#[case] modifier: u32, #[case] expected: (u32, u32)) {}
}
