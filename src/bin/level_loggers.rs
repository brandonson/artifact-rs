extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};
use artifact::level;

fn main() {
  let _artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new_with_level("Foo",
                                      LoggerOutput::StdoutLog,
                                      level::DEBUG);
  logger.debug("This will print");
  logger.info("But this won't");
  logger.critical("This definitely will.");
}
