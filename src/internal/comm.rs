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

use std::sync::Mutex;
use std::cell::RefCell;
use internal::task;
use level;

lazy_static!(
  static ref GLOBAL_LOGGER_ACCESS: Mutex<Option<Artifact>> = Mutex::new(None);
)

thread_local!(static LOCAL_LOGGER_CELL: RefCell<Option<Artifact>> = RefCell::new(None))

#[deriving(Clone)]
pub struct Artifact{
  msg_tx: Sender<task::LoggerMessage>
}

pub fn init_global_task(){
  let mut g_logger = GLOBAL_LOGGER_ACCESS.lock();
  if g_logger.is_none() {
    let logger_task_sender = spawn_logger_task();
    register_level(&logger_task_sender, "TERRIBLE FAILURE", level::WTF);
    register_level(&logger_task_sender, "CRITICAL", level::CRITICAL);
    register_level(&logger_task_sender, "SEVERE", level::SEVERE);
    register_level(&logger_task_sender, "WARNING", level::WARNING);
    register_level(&logger_task_sender, "DEBUG", level::DEBUG);
    register_level(&logger_task_sender, "INFO", level::INFO);
    register_level(&logger_task_sender, "VERBOSE", level::VERBOSE);

    *g_logger = Some(logger_task_sender);
  }
}

fn register_level(artifact_state: &Artifact,
                  name: &str,
                  level: level::LogLevel) {
  artifact_state.msg_tx.send(task::LoggerMessage::RegisterLevelString(level, name.to_string()))
}

pub fn stop_global_task(){
  match *GLOBAL_LOGGER_ACCESS.lock() {
    Some(Artifact{ref msg_tx}) => msg_tx.send(task::LoggerMessage::PoisonPill),
    None => {}
  }
}

pub fn send_logger_message(message: task::LoggerMessage){
  LOCAL_LOGGER_CELL.with(|logger_cell:&RefCell<Option<Artifact>>| {
    let mut mut_cell_internal = logger_cell.borrow_mut();
    let tls_initialized = mut_cell_internal.is_some();

    if tls_initialized {
      send_to_logger(&mut_cell_internal.as_ref().unwrap().msg_tx, message.clone())
    } else {
      send_logger_message_with_uninit_tls(&mut *mut_cell_internal, message.clone())
    }
  })
}

fn send_logger_message_with_uninit_tls(tls_ref:&mut Option<Artifact>, message: task::LoggerMessage){
  let local_sender_opt = GLOBAL_LOGGER_ACCESS.lock();
  match *local_sender_opt {
    None => panic!("Global logger task info not initialized.  Try a GlobalArtifactLib instance?"),
    Some(ref local_sender) => {
      send_to_logger(&local_sender.msg_tx, message);
      *tls_ref = Some(local_sender.clone());
    }
  }
}

#[cfg(not(feature = "no-failure-logs"))]
fn send_to_logger(logger:&Sender<task::LoggerMessage>, message: task::LoggerMessage){
  match logger.send_opt(message) {
    Err(_) => println!("Logger task is down, could not send message."),
    _ => {}
  }
}

#[cfg(feature = "no-failure-logs")]
fn send_to_logger(logger:&Sender<task::LoggerMessage>, message: task::LoggerMessage){
  let _ = logger.send_opt(message);
}

fn spawn_logger_task() -> Artifact {
  let (tx, rx) = channel();
  task::spawn_logger(rx);
  Artifact{msg_tx: tx}
}
