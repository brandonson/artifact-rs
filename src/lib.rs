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

#![feature(unboxed_closures)]
#![feature(box_syntax)]
#![feature(scoped)]

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "time")]
extern crate time;

use std::thread::JoinGuard;

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
  _guard: Option<JoinGuard<'static, ()>>
}

impl ArtifactGlobalLib{
  #[inline(always)]
  #[cfg(not(feature = "disable"))]
  pub fn init() -> ArtifactGlobalLib {
    let guard = internal::comm::init_global_task();
    ArtifactGlobalLib{_guard: guard}
  }

  #[inline(always)]
  #[cfg(feature = "disable")]
  pub fn init() -> ArtifactGlobalLib {
    ArtifactGlobalLib{_guard : None}
  }

  #[inline(always)]
  pub fn stop(&self){
    if self._guard.is_some() {
      internal::comm::stop_global_task();
    }
  }

}
