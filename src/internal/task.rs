use std::io::stderr;
use std::comm::Receiver;
use std::task::spawn;
use std::io::fs::File;
use std::collections::hash_map::HashMap;

use logger::LoggerType;
use level;
use level::LogLevel;

use std::cell::RefCell;

#[deriving(Clone)]
pub enum LoggerMessage{
  PoisonPill,
  LogMessage(String, LogLevel, String),
  NewLogger(String, LogLevel, LoggerType)
}

enum LoggerInstance{
  FileLoggerInst(RefCell<File>),
  StdoutLoggerInst,
  StderrLoggerInst
}

struct LoggerTaskInfo{
  loggers: HashMap<String, (LogLevel, LoggerInstance)>
}

impl LoggerInstance{
  fn write(&self, message:String) {
    match self {
      &LoggerInstance::StdoutLoggerInst => {
        println!("{}", message);
      }
      &LoggerInstance::StderrLoggerInst => {
        // discard failures.  What are we going to do, log it?
        let _ = stderr().write_str(message.as_slice());
      }
      &LoggerInstance::FileLoggerInst(ref file_writer) => {
        let _ = file_writer.borrow_mut().write_str(message.as_slice());
      }
    }
  }
}

impl LoggerTaskInfo{
  fn write_message(&self, logger_name: &str, msg_level: LogLevel, msg: String) {
    match self.loggers.get(logger_name) {
      Some(&(logger_level, ref logger)) => {
        if msg_level <= logger_level {
          logger.write(format!("[{}] {}: {}", logger_name, self.level_string(msg_level), msg));
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
  }

  fn level_string(&self, level: LogLevel) -> String {
    //TODO allow registering level strings, instead of just printing the level number
    format!("{}", level)
  }

  fn add_file_logger(&mut self, logger:String, level:LogLevel, path:Path) {
    let file = match File::create(&path) {
      Ok(x) => x,
      Err(_) => panic!("Could not create log file {}", path.as_str())
    };

    self.loggers.insert(logger,
                        (level, LoggerInstance::FileLoggerInst(RefCell::new(file))));
  }

  fn add_simple_logger(&mut self, logger:String, level: LogLevel, log_ty: LoggerType){
    let simple_inst = match log_ty {
      LoggerType::StdoutLogger => LoggerInstance::StdoutLoggerInst,
      LoggerType::StderrLogger => LoggerInstance::StderrLoggerInst,
      _ => panic!("Unsupported logger type for add_simple_logger.")
    };
    
    self.loggers.insert(logger,
                        (level, simple_inst));
  }
}

pub fn spawn_logger(rx: Receiver<LoggerMessage>){
  //! Spawns the main logger task
  spawn(proc() logger_main(rx));
}

fn logger_main(rx: Receiver<LoggerMessage>){
  let mut task_info = LoggerTaskInfo{loggers: HashMap::new()};
  loop {
    match rx.recv_opt() {
      Ok(LoggerMessage::LogMessage(logger, level, message)) =>
        task_info.write_message(logger.as_slice(), level, message),

      Ok(LoggerMessage::NewLogger(logger, level, LoggerType::FileLogger(path))) =>
        task_info.add_file_logger(logger, level, path),

      Ok(LoggerMessage::NewLogger(logger, level, simple_logger_type)) =>
        task_info.add_simple_logger(logger, level, simple_logger_type),
      Ok(LoggerMessage::PoisonPill) => break,

      Err(_) => break,
    }
  }
}
