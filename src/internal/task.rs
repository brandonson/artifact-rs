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
use std::thread::{JoinGuard};
use std::thread;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::fs::File;
use std::io::{Write, stderr};
use std::borrow::Borrow;

use logger::LoggerOutput;
use level;
use level::LogLevel;

use std::cell::RefCell;

const INTERNAL_LOGGER_NAME:&'static str = "Artifact Internal";

#[derive(Clone)]
pub enum LoggerMessage{
  PoisonPill,
  LogMessage(String, LogLevel, String),
  NewLogger(String, LogLevel, LoggerOutput),
  RedirectLogger(String, Option<LogLevel>, LoggerOutput),
  RegisterLevelString(LogLevel, String),
  Disable(String, bool)
}

enum LoggerInstance{
  FileLoggerInst(Rc<RefCell<File>>, PathBuf),
  StdoutLoggerInst,
  StderrLoggerInst,
  MultiLoggerInst(Vec<String>),
}

struct LoggerTaskInfo{
  loggers: HashMap<String, (LogLevel, LoggerInstance)>,
  level_strings: HashMap<LogLevel, String>,
  disabled: HashMap<String, bool>,
}

impl LoggerInstance{
  fn write(&self, message:&str, level:LogLevel, task_info:&LoggerTaskInfo) {
    match *self {
      LoggerInstance::StdoutLoggerInst => {
        println!("{}", message);
      }
      LoggerInstance::StderrLoggerInst => {
        // discard failures.  What are we going to do, log it?
        let _ = writeln!(&mut stderr(), "{}", message);
      }
      LoggerInstance::FileLoggerInst(ref file_writer, _) => {
        let _ = writeln!(file_writer.borrow_mut(), "{}", message);
      }
      LoggerInstance::MultiLoggerInst(ref other_loggers) => {
        for logger in other_loggers.iter() {
          let formatted_msg = format!("[{}] -- {}", logger, message);
          task_info.write_formatted_message(logger, level, &formatted_msg);
        }
      }
    }
  }

  fn logger_type_name(&self) -> &'static str {
    match *self {
      LoggerInstance::StdoutLoggerInst => "StdoutLogger",
      LoggerInstance::StderrLoggerInst => "StderrLogger",
      LoggerInstance::FileLoggerInst(_,_) => "FileLogger",
      LoggerInstance::MultiLoggerInst(_) => "MultiLogger"
    }
  }
}

impl LoggerTaskInfo{
  fn new() -> LoggerTaskInfo {
    let mut task =
      LoggerTaskInfo{
        loggers: HashMap::new(),
        level_strings: HashMap::new(),
        disabled: HashMap::new()};
    task.add_logger(
      INTERNAL_LOGGER_NAME.to_string(),
      level::DEFAULT,
      LoggerOutput::StdoutLog);
    task
  }
  fn write_message<MsgTy:Borrow<str>>(&self, logger_name: &str, msg_level: LogLevel, msg: MsgTy) {
    self.write_formatted_message(
      logger_name,
      msg_level,
      &format!("[{}] {}: {}", logger_name, self.level_string(msg_level), msg.borrow()));
  }

  fn write_formatted_message(&self, logger_name: &str, msg_level: LogLevel, msg: &str) {
    if self.disabled.contains_key(logger_name) {
      return;
    }
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
      self.write_message(existing_logger.as_ref(),
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

    let file_logger_instance =
      if let Some(prev_logger) = previous_file_logger {
        Some(prev_logger)
      } else {
        match File::create(&path) {
          Ok(new_file) =>
            Some(LoggerInstance::FileLoggerInst(Rc::new(RefCell::new(new_file)), path.clone())),
          Err(_) => {
            None
          }
        }
      };
    match file_logger_instance {
      Some(instance) => {
        self.loggers.insert(
          logger,
          (level, instance));
      }
      None =>
        if let Some(path_str) = path.as_os_str().to_str() {
          self.log_internal(
            format!("Could not create log file {}", path_str),
            level::INTERNAL_EXTREME_FAIL);
        } else {
          self.log_internal(
            "Could not create a log file.  Name is not printable.",
            level::INTERNAL_EXTREME_FAIL);
        }
    }
  }

  fn add_multi_logger(&mut self, logger:String, level:LogLevel, direct_to:Vec<String>){
    let instance = LoggerInstance::MultiLoggerInst(direct_to);
    self.loggers.insert(logger,
                        (level, instance));
  }

  fn add_logger(&mut self, logger:String, level: LogLevel, log_ty: LoggerOutput) {
    let disabled_status = self.disabled.get(&logger).map(|b| *b);
    if !self.loggers.get(&logger).is_none() {
      self.log_internal(
        format!(
          "Cannot re-register the {} logger.",
          logger),
        level::INFO);
    } else if let Some(log_attempt) = disabled_status {
      if log_attempt {
        self.log_internal(
          format!(
            "An attempt to register a logger for name {} was rejected as that name is disabled.",
            logger),
          level::DEBUG);
      }
    } else {
      match log_ty {
        LoggerOutput::StdoutLog => {
          self.loggers.insert(logger, (level, LoggerInstance::StdoutLoggerInst));
        }
        LoggerOutput::StderrLog => {
          self.loggers.insert(logger, (level, LoggerInstance::StderrLoggerInst));
        }
        LoggerOutput::FileLog(path) => {
          self.add_file_logger(logger, level, path);
        }
        LoggerOutput::MultiLog(others) => {
          self.add_multi_logger(logger, level, others);
        }
      };
    }
  }

  fn redirect_logger(&mut self, logger_name:String, level:Option<LogLevel>, log_ty: LoggerOutput) {
    let logger = self.loggers.remove(&logger_name);
    match logger {
      None => {
        self.add_logger(logger_name, level.unwrap_or(level::DEFAULT), log_ty);
        self.log_internal("Attempted to redirect non-existant logger", level::WARNING);
      }
      Some((old_level, _)) => {
        let new_level = level.unwrap_or(old_level);
        self.add_logger(logger_name, new_level, log_ty);
      }
    }
  }

  fn disable_logger(&mut self, logger:String, log: bool) {
    let removed = self.loggers.remove(&logger);

    if log {
      if let Some((_, log_inst)) = removed {
        self.log_internal(
          format!(
            "{} {} has been removed and disabled. Logger was in use.",
            log_inst.logger_type_name(),
            logger),
          level::DEBUG);
      } else {
        self.log_internal(
          format!(
            "Logger name {} has been disabled. Logger was not in use.",
            logger),
          level::DEBUG);
      }
    }
    self.disabled.insert(logger, log);
  }

  fn log_internal<MsgTy: Borrow<str>>(&mut self, message: MsgTy, level: LogLevel) {
    self.write_message(
      INTERNAL_LOGGER_NAME,
      level,
      message);
  }
}

pub fn spawn_logger(rx: Receiver<LoggerMessage>) -> JoinGuard<'static, ()>{
  //! Spawns the main logger task
  thread::scoped(move | | logger_main(rx))
}

fn logger_main(rx: Receiver<LoggerMessage>){
  let mut task_info = LoggerTaskInfo::new();
  loop {
    match rx.recv() {
      Ok(LoggerMessage::LogMessage(logger, level, message)) => {
        task_info.write_message(logger.as_ref(), level, message);
      }

      Ok(LoggerMessage::NewLogger(logger, level, output)) =>
        task_info.add_logger(logger, level, output),

      Ok(LoggerMessage::PoisonPill) => {
        break;
      }

      Ok(LoggerMessage::RegisterLevelString(level, string)) => {
        task_info.level_strings.insert(level, string);
      }

      Ok(LoggerMessage::Disable(name, log)) => {
        task_info.disable_logger(name, log);
      }

      Ok(LoggerMessage::RedirectLogger(logger, level_opt, output)) => {
        task_info.redirect_logger(logger, level_opt, output);
      }

      Err(_) => break,
    }
  }
}
