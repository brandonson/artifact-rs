#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(plugin)]
extern crate lazy_static;

pub use level::LogLevel;
pub use level::{WTF, CRITICAL, SEVERE, WARNING, DEBUG, INFO, VERBOSE};
pub use logger::{Logger, LoggerType};

pub mod level;
pub mod logger;
mod internal;

pub struct ArtifactGlobalLib {
  #[allow(dead_code)]
  x: ()
}

impl ArtifactGlobalLib{
  pub fn init() -> ArtifactGlobalLib {
    internal::comm::init_global_task();
    ArtifactGlobalLib{x : ()}
  }
}

impl Drop for ArtifactGlobalLib{
  fn drop(&mut self){
    internal::comm::stop_global_task();
  }
}
