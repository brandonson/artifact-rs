extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};

fn main() {
  let artifact_global = ArtifactGlobalLib::init();
  let global2 = ArtifactGlobalLib::init();

  let logger1 = Logger::new("Foo", LoggerOutput::StdoutLog);
  logger1.critical("This should be there");
  logger1.disable();
  let logger1 = Logger::access("Foo"); //disabling consumes the logger, re-access it
  Logger::access("Bar").disable();
  Logger::access("ThisShouldPrintAMessage").disable();

  let logger2 = Logger::new("Bar", LoggerOutput::StderrLog);
  logger1.critical("This shouldn't appear");
  logger2.critical("Nor should this");
  global2.stop();

  let _ = Logger::new("ThisShouldPrintAMessage", LoggerOutput::StdoutLog);
  artifact_global.stop();
}
