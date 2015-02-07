#![feature(path)]
extern crate artifact;

use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};

fn main() {
  let artifact_global = ArtifactGlobalLib::init();

  let logger = Logger::new("Foo", LoggerOutput::FileLog(Path::new("foolog.log")));
  logger.warning("This is a file log.");

  artifact_global.stop();
}
