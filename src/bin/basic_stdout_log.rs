extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};

fn main() {
  let _artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo", LoggerOutput::StdoutLog);
  logger.wtf("WHAT IS HAPPENING!");
}
