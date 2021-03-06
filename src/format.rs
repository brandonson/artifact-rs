/*
 * Copyright (c) 2015 Brandon Sanderson
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

#[cfg(feature = "time")]
use time;

/// Trait for things that can format logging messages
pub trait MessageFormatter : Send + Sync {
  /// Format a standard message.
  fn format_message(&self, logger_name:&str, level_string: &str, message: &str) -> String;
  /// Format for messages being sent onward by a multi-logger.
  /// logger_name is the name of the next logger.  formatted_multi_msg is the
  /// message as formatted by the multi-logger's format_message method.
  fn add_logger_name_to_multi_message(&self, logger_name: &str, formatted_multi_msg:&str) -> String;

  fn add_defaulting_name_to_message(&self, default_logger_name: &str, formatted_msg:&str) -> String;
}

/// Default formatter for logging messages
/// if time info isn't available.
#[derive(Clone)]
pub struct SimpleMessageFormatter;

/// Default formatter when time info is available.
/// Formats according to RFC 822, with Zulu time.
#[cfg(feature = "time")]
#[derive(Clone)]
pub struct ZuluTimeMessageFormatter;

impl MessageFormatter for SimpleMessageFormatter {
  fn format_message(&self, logger_name: &str, level_string: &str, message: &str) -> String {
    format!("[{}] -- {}: {}", logger_name, level_string, message)
  }

  fn add_logger_name_to_multi_message(&self, logger_name: &str, formatted_multi_msg:&str) -> String {
    format!("[{}] from {}", logger_name, formatted_multi_msg)
  }

  fn add_defaulting_name_to_message(&self, default_logger_name: &str, formatted_msg:&str) -> String {
    format!("[{}] defaulting from {}", default_logger_name, formatted_msg)
  }
}

#[cfg(feature = "time")]
impl MessageFormatter for ZuluTimeMessageFormatter {
  fn format_message(&self, logger_name: &str, level_string: &str, message: &str) -> String {
    let time = time::now();
    format!("[{}] ({}) -- {}: {}", logger_name, time.rfc822z(), level_string, message)
  }

  fn add_logger_name_to_multi_message(&self, logger_name: &str, formatted_multi_msg:&str) -> String {
    format!("[{}] from {}", logger_name, formatted_multi_msg)
  }

  fn add_defaulting_name_to_message(&self, default_logger_name: &str, formatted_msg:&str) -> String {
    format!("[{}] defaulting from {}", default_logger_name, formatted_msg)
  }
}

#[cfg(not(feature = "time"))]
pub fn new_basic_format_instance() -> Box<MessageFormatter> {
  Box::new(SimpleMessageFormatter)
}

#[cfg(feature = "time")]
pub fn new_basic_format_instance() -> Box<MessageFormatter> {
  Box::new(ZuluTimeMessageFormatter)
}

pub struct NoForwardingIndicationFormatter<T>(pub T) where T: MessageFormatter;

impl<F> MessageFormatter for NoForwardingIndicationFormatter<F> where F: MessageFormatter{
  fn format_message(&self, logger_name: &str, level_string: &str, message: &str) -> String {
    self.0.format_message(logger_name, level_string, message)
  }
  fn add_logger_name_to_multi_message(&self, _: &str, msg: &str) -> String {
    msg.to_string()
  }
  fn add_defaulting_name_to_message(&self, _: &str, msg: &str) -> String {
    msg.to_string()
  }
}
