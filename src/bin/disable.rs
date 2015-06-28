extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib, MessageFormatter};

struct FooFormat;

impl MessageFormatter for FooFormat {
  fn format_message(&self, logger:&str, level_string:&str, message: &str) -> String {
    format!("Foobled from {} -- {}: {}", logger, level_string, message)
  }

  fn add_logger_name_to_multi_message(&self, _:&str, m:&str) -> String {
    m.to_string()
  }

  fn add_defaulting_name_to_message(&self, default_logger_name: &str, formatted_msg:&str) -> String {
    format!("[{}] defaulting from {}", default_logger_name, formatted_msg)
  }
}

fn main() {
  let _artifact_global = ArtifactGlobalLib::init();

  let logger1 = Logger::new("Foo", LoggerOutput::StdoutLog);
  logger1.critical("This should be there");
  logger1.disable();
  let logger1 = Logger::access("Foo"); //disabling consumes the logger, re-access it
  Logger::access("Bar").disable();
  Logger::access("ThisShouldPrintAMessage").disable();

  let internal = Logger::access_internal_logger();
  internal.warning("Setting new output format");
  internal.set_format(Box::new(FooFormat));

  let logger2 = Logger::new("Bar", LoggerOutput::StderrLog);
  logger1.critical("This shouldn't appear");
  logger2.critical("Nor should this");

  let _ = Logger::new("ThisShouldPrintAMessage", LoggerOutput::StdoutLog);
}
