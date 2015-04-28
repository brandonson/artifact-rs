extern crate artifact;

use std::path::PathBuf;
use artifact::{Logger, LoggerOutput, ArtifactGlobalLib, DEBUG};

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo", LoggerOutput::FileLog(PathBuf::from("foolog.log")));
  let logger_two = Logger::new_with_level("Bar", LoggerOutput::FileLog(PathBuf::from("foolog.log")), DEBUG);
  logger.warning("This is a file log.");
  logger_two.debug("And we can have two loggers to one file at once.");

  artifact_global.stop();
}
