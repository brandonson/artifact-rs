extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};
use artifact::level;

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

  logger.debug("This will print");
  multi_logger.debug("This won't");
  multi_logger.warning("This will go to stdout");
  multi_logger.critical("This will print to both.");

  artifact_global.stop();
}
