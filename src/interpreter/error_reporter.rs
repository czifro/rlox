

pub struct ErrorReporter {
  had_error: bool
}

impl ErrorReporter {
  pub fn error(&self, line: i32, message: String) {
    self.report(line, "".to_string(), message);
  }
  
  fn report(&self, line: i32, where: String, message: String) {
    println!("[line {line}] Error{where}: {message}");
    self.had_error = true;
  }
  
  pub fn errored(&self) -> bool {
    self.had_error
  }
}

impl Default for ErrorReporter {
  fn default() -> Self {
    Self {
      had_error: false
    }
  }
}