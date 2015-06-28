extern crate artifact;

use std::path::PathBuf;
use artifact::{Logger, LoggerOutput, ArtifactGlobalLib, DEBUG, MessageFormatter};

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

  let logger = Logger::new("Foo", LoggerOutput::FileLog(PathBuf::from("foolog.log")));
  let logger_two = Logger::new_with_level("Bar", LoggerOutput::FileLog(PathBuf::from("foolog.log")), DEBUG);
  logger.set_format(Box::new(FooFormat) as Box<MessageFormatter>);
  logger.warning("This is a file log.");
  logger_two.debug("And we can have two loggers to one file at once.");
  Logger::set_default_formatter(Box::new(FooFormat) as Box<MessageFormatter>);
  logger_two.severe("This format will be different, now matching Foo's.");
}
