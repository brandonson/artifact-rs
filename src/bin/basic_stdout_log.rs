extern crate artifact;

use artifact::{Logger, LoggerType, ArtifactGlobalLib};

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo", LoggerType::StdoutLogger);
  logger.wtf("WHAT IS HAPPENING!");

  artifact_global.stop();
}
