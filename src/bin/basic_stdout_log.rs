extern crate artifact;

use artifact::{Logger, LoggerType, ArtifactGlobalLib};

fn main() {
  let _ = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo".to_string(), LoggerType::StdoutLogger);
  logger.wtf("WHAT IS HAPPENING!".to_string());
}
