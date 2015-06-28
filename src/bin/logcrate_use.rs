extern crate artifact;

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(feature = "log")]
fn main() {
  with_log::main();
}

#[cfg(not(feature = "log"))]
fn main() {
  println!("Logging feature not enabled.");
}


#[cfg(feature = "log")]
mod with_log {


use artifact::{Logger, LoggerOutput, ArtifactGlobalLib};
use artifact::level;
use artifact;

pub fn main() {
  let _global = ArtifactGlobalLib::init();
  artifact::setup_log_crate_support().unwrap();
  let default_log = Logger::new_with_level("Foo", LoggerOutput::StdoutLog, level::TRACE);
  default_log.set_as_default();

  debug!("This is a test log");

  default_log.set_as_silent_default();

  info!("This log will have no indication that it defaulted.");
}

}
