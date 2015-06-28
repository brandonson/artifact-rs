extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};
use artifact::level;

fn main() {
  let _artifact_global = ArtifactGlobalLib::init();

  let logger1 = Logger::new("Foo", LoggerOutput::StdoutLog);
  logger1.debug("This should be there");
  logger1.redirect_set_level(LoggerOutput::StdoutLog, level::CRITICAL);
  logger1.debug("This should not print");
  logger1.critical("You should see this, and one message before this.");
}
