extern crate artifact;

use artifact::{Logger, LoggerType, ArtifactGlobalLib};
use artifact::level;

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new_with_level("Foo".to_string(),
                                      LoggerType::StdoutLogger,
                                      level::DEBUG);
  logger.debug("This will print".to_string());
  logger.info("But this won't".to_string());
  logger.critical("This definitely will.".to_string());

  artifact_global.stop();
}
