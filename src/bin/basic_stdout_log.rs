extern crate artifact;

use artifact::{Logger, LoggerType, ArtifactGlobalLib};

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo".to_string(), LoggerType::StdoutLogger);
  logger.wtf("WHAT IS HAPPENING!".to_string());

  artifact_global.stop();
}
