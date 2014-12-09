/* 
 * Copyright (c) 2014 Brandon Sanderson
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

#[deriving(Show, Clone, Eq, PartialEq)]
pub struct Logger{
  pub name:String
}

#[deriving(Clone, PartialEq, Eq)]
pub enum LoggerType{
  FileLogger(Path),
  StdoutLogger,
  StderrLogger
}

impl Logger{

  pub fn new(name:String, ty:LoggerType) -> Logger{
    send_logger_message(LoggerMessage::NewLogger(name.clone(),
                                                 feature_based_log_level(),
                                                 ty));
    Logger{name:name}
  }

  pub fn log(&self, level: LogLevel, message:String){
    send_logger_message(
      LoggerMessage::LogMessage(
        self.name.clone(),
        level,
        message));
  }

  pub fn wtf(&self, message:String){
    self.log(level::WTF, message);
  }

  pub fn critical(&self, message:String){
    self.log(level::CRITICAL, message);
  }

  pub fn severe(&self, message:String){
    self.log(level::SEVERE, message);
  }

  pub fn warning(&self, message:String){
    self.log(level::WARNING, message)
  }

  pub fn debug(&self, message:String){
    self.log(level::DEBUG, message);
  }

  pub fn info(&self, message:String){
    self.log(level::INFO, message);
  }

  pub fn verbose(&self, message:String){
    self.log(level::VERBOSE, message);
  }
}


