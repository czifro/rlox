mod interpreter;

use interpreter::*;

fn main() -> std::io::Result<()> {
  let lox = LoxInterpreter::new();
  
  lox.launch()
}
