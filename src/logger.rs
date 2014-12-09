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


