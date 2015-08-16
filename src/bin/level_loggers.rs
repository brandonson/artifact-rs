extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};
use artifact::level;

fn main() {
  let _artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new_with_level("Foo",
                                      LoggerOutput::StdoutLog,
                                      level::TRACE);
  logger.trace("This will print");
  logger.verbose("But this won't");
  logger.critical("This definitely will.");
}
