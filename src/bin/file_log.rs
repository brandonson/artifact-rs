extern crate artifact;

use artifact::{Logger, LoggerType, ArtifactGlobalLib};

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo", LoggerType::FileLogger(Path::new("foolog.log")));
  logger.warning("This is a file log.");

  artifact_global.stop();
}
