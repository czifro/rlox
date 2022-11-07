mod interpreter;
mod prelude;

use interpreter::*;

fn main() -> std::io::Result<()> {
  let lox = LoxInterpreter::new();
  
  lox.launch()
}
