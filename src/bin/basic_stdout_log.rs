extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo", LoggerOutput::StdoutLog);
  logger.wtf("WHAT IS HAPPENING!");

  artifact_global.stop();
}
