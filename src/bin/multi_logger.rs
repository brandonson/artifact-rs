extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib, MessageFormatter, SimpleMessageFormatter};
use artifact::level;

struct MultiForm;

impl MessageFormatter for MultiForm {
  fn format_message(&self, logger_name:&str, level_string:&str, message: &str) -> String {
    let def = SimpleMessageFormatter;

    def.format_message(logger_name, level_string, message)
  }

  fn add_logger_name_to_multi_message(&self, logger_name: &str, message: &str) -> String {
    format!("[{}] FROM {}", logger_name, message)
  }

  fn add_defaulting_name_to_message(&self, default_logger_name: &str, formatted_msg:&str) -> String {
    format!("[{}] defaulting from {}", default_logger_name, formatted_msg)
  }
}

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new_with_level("Foo",
                                      LoggerOutput::StdoutLog,
                                      level::DEBUG);
  let _stderr = Logger::new_with_level("STDERRLOG",
                                       LoggerOutput::StderrLog,
                                       level::SEVERE);
  let multi_logger =
        Logger::new_with_level(
          "Multi",
          LoggerOutput::MultiLog(vec!("Foo".to_string(), "STDERRLOG".to_string())),
          level::WARNING);

  multi_logger.set_format(Box::new(MultiForm) as Box<MessageFormatter>);

  logger.debug("This will print");
  multi_logger.debug("This won't");
  multi_logger.warning("This will go to stdout");
  multi_logger.critical("This will print to both.");

  artifact_global.stop();
}
