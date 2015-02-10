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

use level;
use level::LogLevel;
use internal::comm::send_logger_message;
use internal::task::LoggerMessage;

fn feature_based_log_level() -> LogLevel{
  level::WARNING
}

/// A logger within the Artifact logging library.
/// The struct itself only stores the name of the
/// logger, however, the initialization functions tell
/// the backend what kind of logger it is, what level it logs
/// at, and other interesting information.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Logger{
  name:String
}

/// Indicates what kind of output stream the logger will use.
#[derive(Clone, PartialEq, Eq)]
pub enum LoggerOutput{
  FileLog(Path),
  StdoutLog,
  StderrLog,
  /// Log to various other loggers.
  /// Any messages sent to this logger will be forwarded
  /// on to the loggers with the names given.
  /// Note that messages are filtered both by the level
  /// of this logger and the level of the sub loggers
  /// assigned to it.
  MultiLog(Vec<String>),
}

impl Logger{

  /// Creates a Logger instance, but does not tell the
  /// backend to initialize the logger.
  pub fn access(name: &str) -> Logger {
    Logger{name: name.to_string()}
  }

  /// Creates a logger which will log to the given output.
  /// This tells the backend logger task to initialize the logger.
  #[inline(always)]
  pub fn new(name: &str, ty: LoggerOutput) -> Logger{
    Logger::new_with_level(name, ty, feature_based_log_level())
  }

  /// Creates a logger for the given output which logs messages at or above the given level.
  /// This also initializes the logger by telling the backend task.
  #[inline(always)]
  #[cfg(not(feature = "disable"))]
  pub fn new_with_level(name: &str, ty: LoggerOutput, level:LogLevel) -> Logger {
    send_logger_message(LoggerMessage::NewLogger(name.to_string(),
                                                 level,
                                                 ty));
    Logger{name: name.to_string()}
  }

  #[inline(always)]
  #[cfg(feature = "disable")]
  pub fn new_with_level(name: &str, ty: LoggerOutput, level: LogLevel) -> Logger {
    Logger{name : name.to_string() }
  }

  /// Creates a new log message.  This just sends a message across
  /// the backend channel to the actual logger task.
  #[inline(always)]
  #[cfg(not(feature = "disable"))]
  pub fn log(&self, level: LogLevel, message:&str){
    send_logger_message(
      LoggerMessage::LogMessage(
        self.name.clone(),
        level,
        message.to_string()));
  }

  #[inline(always)]
  #[cfg(feature = "disable")]
  pub fn log(&self, level: LogLevel, message:&str){

  }

  #[inline(always)]
  pub fn wtf(&self, message:&str){
    self.log(level::WTF, message);
  }

  #[inline(always)]
  pub fn critical(&self, message:&str){
    self.log(level::CRITICAL, message);
  }

  #[inline(always)]
  pub fn severe(&self, message:&str){
    self.log(level::SEVERE, message);
  }

  #[inline(always)]
  pub fn warning(&self, message:&str){
    self.log(level::WARNING, message)
  }

  #[inline(always)]
  pub fn debug(&self, message:&str){
    self.log(level::DEBUG, message);
  }

  #[inline(always)]
  pub fn info(&self, message:&str){
    self.log(level::INFO, message);
  }

  #[inline(always)]
  pub fn verbose(&self, message:&str){
    self.log(level::VERBOSE, message);
  }
}


