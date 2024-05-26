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

  /// Writes the value, including the sign
  fn write(&mut self, number: T, sign: bool);

  /// Writes the value, without the sign
  fn write_data(&mut self, number: T);
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
