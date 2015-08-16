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

#[cfg(feature = "log")]
extern crate log;

use std::thread::JoinHandle;

pub use level::LogLevel;
pub use level::{WTF, CRITICAL, SEVERE, WARNING, DEBUG, INFO, TRACE, VERBOSE};
pub use logger::{Logger, LoggerOutput};
pub use format::{MessageFormatter, SimpleMessageFormatter, NoForwardingIndicationFormatter};
#[cfg(feature = "time")]
pub use format::ZuluTimeMessageFormatter;

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
}

impl Drop for ArtifactGlobalLib {
  fn drop(&mut self) {
    if let Some(thread_handle) = self.handle.take() {
      internal::comm::stop_global_task();
      let _ = thread_handle.join();
    }
  }
}

#[cfg(feature = "log")]
pub fn setup_log_crate_support() -> Result<(), log::SetLoggerError> {
  log::set_logger(|max_log_level| {
    max_log_level.set(log::LogLevelFilter::Trace);
    Box::new(logger::ArtifactDelegateLog)
  })
}
