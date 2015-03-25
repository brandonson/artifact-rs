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

use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::thread::{Thread, JoinGuard};
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::fs::File;
use std::io::{Write, stderr};

use logger::LoggerOutput;
use level;
use level::LogLevel;

use std::cell::RefCell;

#[derive(Clone)]
pub enum LoggerMessage{
  PoisonPill,
  LogMessage(String, LogLevel, String),
  NewLogger(String, LogLevel, LoggerOutput),
  RegisterLevelString(LogLevel, String)
}

enum LoggerInstance{
  FileLoggerInst(Rc<RefCell<File>>, PathBuf),
  StdoutLoggerInst,
  StderrLoggerInst,
  MultiLoggerInst(Vec<String>),
}

struct LoggerTaskInfo{
  loggers: HashMap<String, (LogLevel, LoggerInstance)>,
  level_strings: HashMap<LogLevel, String>
}

impl LoggerInstance{
  fn write(&self, message:String, level:LogLevel, task_info:&LoggerTaskInfo) {
    match self {
      &LoggerInstance::StdoutLoggerInst => {
        println!("{}", message);
      }
      &LoggerInstance::StderrLoggerInst => {
        // discard failures.  What are we going to do, log it?
        let _ = writeln!(&mut stderr(), "{}", message.as_slice());
      }
      &LoggerInstance::FileLoggerInst(ref file_writer, _) => {
        let _ = writeln!(file_writer.borrow_mut(), "{}", message.as_slice());
      }
      &LoggerInstance::MultiLoggerInst(ref other_loggers) => {
        for logger in other_loggers.iter() {
          let formatted_msg = format!("[{}] -- {}", logger, message);
          task_info.write_formatted_message(logger, level, formatted_msg);
        }
      }
    }
  }
}

impl LoggerTaskInfo{
  fn write_message(&self, logger_name: &str, msg_level: LogLevel, msg: String) {
    self.write_formatted_message(
      logger_name,
      msg_level,
      format!("[{}] {}: {}", logger_name, self.level_string(msg_level), msg));
  }

  fn write_formatted_message(&self, logger_name: &str, msg_level: LogLevel, msg: String) {
    match self.loggers.get(logger_name) {
      Some(&(logger_level, ref logger)) => {
        if msg_level <= logger_level {
          logger.write(msg, msg_level, &self);
        }
      }
      None => self.handle_nonexistant_logger(logger_name)
    }
  }

  fn handle_nonexistant_logger(&self, logger: &str){
    for existing_logger in self.loggers.keys() {
      self.write_message(existing_logger.as_slice(),
                         level::WTF,
                         format!("Can't log to the {} logger, it doesn't exist.", logger))
    }
    println!("Can't log to the {} logger, it doesn't exist.", logger);
  }

  fn level_string(&self, level: LogLevel) -> String {
    match self.level_strings.get(&level) {
      Some(ref strval) => strval.to_string(),
      None => level.to_string()
    }
  }

  fn add_file_logger(&mut self, logger:String, level:LogLevel, path:PathBuf) {
    if !self.loggers.get(logger.as_slice()).is_none() {
      //TODO add an internal event logger so we can log things like this
      return;
    }

    let mut previous_file_logger:Option<LoggerInstance> = None;

    for &(_, ref known_logger) in self.loggers.values() {
      match known_logger {
        &LoggerInstance::FileLoggerInst(ref cell, ref prev_path) => {
          if *prev_path == path {
            previous_file_logger = Some(LoggerInstance::FileLoggerInst(cell.clone(), prev_path.clone()));
            break;
          }
        }
        _ => {}
      }
    }

    if let Some(prev_logger) = previous_file_logger {
      self.loggers.insert(logger,
                          (level, prev_logger));
      return;
    }

    let file = match File::create(&path) {
      Ok(x) => x,
      Err(_) => {
        if let Some(path_str) = path.as_os_str().to_str() {
          panic!("Could not create log file {}", path_str);
        } else {
          panic!("Could not create a log file (name is not printable)");
        }
      }
    };

    self.loggers.insert(logger,
                        (level, LoggerInstance::FileLoggerInst(Rc::new(RefCell::new(file)), path)));
  }

  fn add_multi_logger(&mut self, logger:String, level:LogLevel, direct_to:Vec<String>){
    if !self.loggers.get(logger.as_slice()).is_none() {
      //TODO add an internal event logger so we can log things like this
      return;
    }

    let instance = LoggerInstance::MultiLoggerInst(direct_to);
    self.loggers.insert(logger,
                        (level, instance));
  }

  fn add_simple_logger(&mut self, logger:String, level: LogLevel, log_ty: LoggerOutput){
    if !self.loggers.get(logger.as_slice()).is_none() {
      //TODO add an internal event logger so we can log things like this
      return;
    }

    let simple_inst = match log_ty {
      LoggerOutput::StdoutLog => LoggerInstance::StdoutLoggerInst,
      LoggerOutput::StderrLog => LoggerInstance::StderrLoggerInst,
      _ => panic!("Unsupported logger type for add_simple_logger.")
    };
    
    self.loggers.insert(logger,
                        (level, simple_inst));
  }
}

pub fn spawn_logger(rx: Receiver<LoggerMessage>) -> JoinGuard<'static, ()>{
  //! Spawns the main logger task
  Thread::scoped(move | | logger_main(rx))
}

fn logger_main(rx: Receiver<LoggerMessage>){
  let mut task_info = LoggerTaskInfo{loggers: HashMap::new(), level_strings: HashMap::new()};
  loop {
    match rx.recv() {
      Ok(LoggerMessage::LogMessage(logger, level, message)) => {
        task_info.write_message(logger.as_slice(), level, message);
      }

      Ok(LoggerMessage::NewLogger(logger, level, LoggerOutput::FileLog(path))) =>
        task_info.add_file_logger(logger, level, path),

      Ok(LoggerMessage::NewLogger(logger, level, LoggerOutput::MultiLog(others))) =>
        task_info.add_multi_logger(logger, level, others),

      Ok(LoggerMessage::NewLogger(logger, level, simple_logger_type)) =>
        task_info.add_simple_logger(logger, level, simple_logger_type),

      Ok(LoggerMessage::PoisonPill) => {
        break;
      }

      Ok(LoggerMessage::RegisterLevelString(level, string)) => {
        task_info.level_strings.insert(level, string);
      }

      Err(_) => break,
    }
  }
}
