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
use std::path::PathBuf;

use MessageFormatter;

/// A logger within the Artifact logging library.
/// This struct is somewhat similar to an address.
/// The struct itself only stores the name of the
/// logger. However, the initialization functions tell
/// the backend what kind of logger it is, what level it logs
/// at, and other interesting information.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Logger{
  name:String
}

/// Indicates what kind of output stream a logger will use.
#[derive(Clone, PartialEq, Eq)]
pub enum LoggerOutput{
  FileLog(PathBuf),
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

  /// Sets the default formatter.  This formatter will be
  /// used by any logger which does not have a formatter
  /// set for it.
  pub fn set_default_formatter(fmtr:Box<MessageFormatter>) {
    send_logger_message(LoggerMessage::SetDefaultFormatter(fmtr));
  }

  /// Creates a Logger instance, but does not tell the
  /// backend to initialize the logger.
  pub fn access(name: &str) -> Logger {
    Logger{name: name.to_string()}
  }

  /// Creates a logger which will log to the given output.
  /// This tells the backend logger task to initialize the logger.
  pub fn new(name: &str, ty: LoggerOutput) -> Logger{
    Logger::new_with_level(name, ty, level::DEFAULT)
  }

  /// Creates a logger for the given output which logs messages at or above the given level.
  /// This also initializes the logger by telling the backend task.
  pub fn new_with_level(name: &str, ty: LoggerOutput, level:LogLevel) -> Logger {
    send_logger_message(LoggerMessage::NewLogger(name.to_string(),
                                                 level,
                                                 ty));
    Logger::access(name)
  }

  /// Redirects a logger to a new output location.
  /// Returns the logger as well
  pub fn redirect(&self, ty: LoggerOutput) {
    send_logger_message(
      LoggerMessage::RedirectLogger(
        self.name.to_string(),
        None,
        ty));
  }

  /// Redirects a logger and changes its level
  /// Returns the logger
  pub fn redirect_set_level(&self, ty: LoggerOutput, level: LogLevel) {
    send_logger_message(
      LoggerMessage::RedirectLogger(
        self.name.to_string(),
        Some(level),
        ty));
  }

  ///Prevents use of a logger name, and kills off any existing
  ///logger instances with that name
  pub fn disable(self) {
    send_logger_message(LoggerMessage::Disable(self.name, true));
  }

  ///Prevents use of a logger name, kills off any existing loggers
  ///with that name, and disables logging info about that logger
  ///entirely.
  pub fn disable_without_logs(self) {
    send_logger_message(LoggerMessage::Disable(self.name, false));
  }

  ///Sets the logger's format
  pub fn set_format(&self, formatter: Box<MessageFormatter>) {
    send_logger_message(LoggerMessage::SetFormatter(self.name.to_string(), formatter));
  }

  /// Creates a new log message.  This just sends a message across
  /// the backend channel to the actual logger task.
  pub fn log(&self, level: LogLevel, message:&str){
    send_logger_message(
      LoggerMessage::LogMessage(
        self.name.clone(),
        level,
        message.to_string()));
  }

  pub fn wtf(&self, message:&str){
    self.log(level::WTF, message);
  }

  pub fn critical(&self, message:&str){
    self.log(level::CRITICAL, message);
  }

  pub fn severe(&self, message:&str){
    self.log(level::SEVERE, message);
  }

  pub fn warning(&self, message:&str){
    self.log(level::WARNING, message)
  }

  pub fn debug(&self, message:&str){
    self.log(level::DEBUG, message);
  }

  pub fn info(&self, message:&str){
    self.log(level::INFO, message);
  }

  pub fn verbose(&self, message:&str){
    self.log(level::VERBOSE, message);
  }
}


