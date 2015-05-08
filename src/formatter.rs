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

use std::borrow::Borrow;

pub trait MessageFormatter : Send + Sync {
  fn format_message(&self, logger_name:&str, level_string: &str, message: &str) -> String;
  fn add_logger_name_to_multi_message(&self, logger_name: &str, formatted_multi_msg:&str) -> String;
}

#[derive(Clone)]
pub struct DefaultMessageFormatter;

impl MessageFormatter for DefaultMessageFormatter {
  fn format_message(&self, logger_name: &str, level_string: &str, message: &str) -> String {
    format!("[{}] -- {}: {}", logger_name, level_string, message)
  }

  fn add_logger_name_to_multi_message(&self, logger_name: &str, formatted_multi_msg:&str) -> String {
    format!("[{}] from {}", logger_name, formatted_multi_msg)
  }
}

impl<MF : Borrow<Box<MessageFormatter>> + Sync + Send> MessageFormatter for Option<MF> {
  fn format_message(&self, logger_name: &str, level_string: &str, message: &str) -> String {
    match *self {
      Some(ref mf) => mf.borrow().format_message(logger_name, level_string, message),
      None => {
        let dmf = DefaultMessageFormatter;
        dmf.format_message(logger_name, level_string, message)
      }
    }
  }

  fn add_logger_name_to_multi_message(&self, logger_name: &str, message: &str) -> String {
    match *self {
      Some(ref mf) => mf.borrow().add_logger_name_to_multi_message(logger_name, message),
      None => {
        let dmf = DefaultMessageFormatter;
        dmf.add_logger_name_to_multi_message(logger_name, message)
      }
    }
  }
}
