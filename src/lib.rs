/*
 * Copyright (c) 2014-2015 Brandon Sanderson
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 */

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "time")]
extern crate time;

#[cfg(test)]
extern crate codifyle;

use std::thread::JoinHandle;

pub use level::LogLevel;
pub use level::{WTF, CRITICAL, SEVERE, WARNING, DEBUG, INFO, VERBOSE};
pub use logger::{Logger, LoggerOutput};
pub use format::{MessageFormatter, SimpleMessageFormatter};

pub mod level;
pub mod logger;
pub mod format;
mod internal;

/// Used to initialize and clean up the logger library
pub struct ArtifactGlobalLib {
  handle: Option<JoinHandle<()>>
}

impl ArtifactGlobalLib{

  pub fn init() -> ArtifactGlobalLib {
    let handle = internal::comm::init_global_task();
    ArtifactGlobalLib{handle: handle}
  }

  pub fn stop(self){
    if let Some(thread_handle) = self.handle {
      internal::comm::stop_global_task();
      //TODO should we provide a way to handle this?
      let _ = thread_handle.join();
    }
  }

}

#[cfg(test)]
mod file_test{
  use std::path::PathBuf;
  use super::{Logger, LoggerOutput, ArtifactGlobalLib, DEBUG, MessageFormatter, SimpleMessageFormatter};
  use codifyle;

  struct FooFormat;

  impl MessageFormatter for FooFormat {
    fn format_message(&self, logger:&str, level_string:&str, message: &str) -> String {
      format!("Foobled from {} -- {}: {}", logger, level_string, message)
    }

    fn add_logger_name_to_multi_message(&self, _:&str, m:&str) -> String {
      m.to_string()
    }
  }

  #[test]
  fn test_multi_file_write() {
    let artifact_global = ArtifactGlobalLib::init();

    let logger = Logger::new("Foo", LoggerOutput::FileLog(PathBuf::from("foolog.log")));
    let logger_two = Logger::new_with_level("Bar", LoggerOutput::FileLog(PathBuf::from("foolog.log")), DEBUG);
    logger.set_format(Box::new(FooFormat) as Box<MessageFormatter>);
    logger.warning("This is a file log.");

    logger_two.set_format(Box::new(SimpleMessageFormatter) as Box<MessageFormatter>);
    logger_two.debug("And we can have two loggers to one file at once.");

    artifact_global.stop();
    codifyle::assert_file_contents_eq(
      "foolog.log",
      "Foobled from Foo -- WARNING: This is a file log.\
      \n[Bar] -- DEBUG: And we can have two loggers to one file at once.\n");
  }

  #[test]
  fn test_default_logger_set() {
    let artifact_global = ArtifactGlobalLib::init();

    let logger = Logger::new("DEFCHANGE", LoggerOutput::FileLog(PathBuf::from("foolog.log")));
    Logger::set_default_formatter(Box::new(FooFormat) as Box<MessageFormatter>);
    logger.warning("This will have the default format of FooFormat");

    artifact_global.stop();

    codifyle::assert_file_contents_eq(
      "foolog.log",
      "Foobled from DEFCHANGE -- WARNING: This will have the default format of FooFormat\n");
  }
}
