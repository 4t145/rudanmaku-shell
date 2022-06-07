
use tauri::api::process::{Command, CommandChild, CommandEvent};

// use tokio::task::JoinHandle;
// use crate::event::*;
#[derive(Debug)]
pub enum ErrorKind {
    TaskExist,
    SidecarError(String),
    SpawnFail(String),
}

// pub enum Exception {
//     TaskDoesNotExist
// }

pub struct Container {
    pub cmd: String,
    pub task: Option<CommandChild>,
}

pub type OutStream = tokio::sync::mpsc::Receiver<CommandEvent>;
impl Container {
    pub fn new(cmd: impl Into<String>) -> Self {
        Self {
            cmd: cmd.into(),
            task: None,
        }
    }

    pub fn start(&mut self, args:&[&str]) -> Result<OutStream, ErrorKind> {
        match &self.task {
            Some(_) => {
                return Err(ErrorKind::TaskExist);
            },
            None => {
                let child_cmd= Command::new_sidecar(self.cmd.clone())
                    .map_err(|e|{ErrorKind::SidecarError(e.to_string())})?;
                let (rx, child) = child_cmd.args(args).spawn()
                    .map_err(|e|{ErrorKind::SpawnFail(e.to_string())})?;
                self.task = Some(child);
                Ok(rx)
            },
        }
    }

    pub fn kill(&mut self) {
        self.task.take().map(|task|task.kill().unwrap_or_default());
    }
}

use crate::event::*;
pub async fn bridge(mut inbound: OutStream, outbound: tauri::Window, id: impl Into<String>) {
    let id:String = id.into();
    while let Some(evt) = inbound.recv().await {
        let out = match evt {
            CommandEvent::Stderr(e) => {
                ConsoleOut {
                    tag: ConsoleOutType::StdErr,
                    id: id.clone(), 
                    output: e
                }
            },
            CommandEvent::Stdout(line) => {
                ConsoleOut {
                    tag: ConsoleOutType::StdOut,
                    id: id.clone(), 
                    output: line
                }
            },
            CommandEvent::Error(e) => {
                ConsoleOut {
                    tag: ConsoleOutType::Error,
                    id: id.clone(), 
                    output: e
                }
            },
            CommandEvent::Terminated(pld) => {
                let code = pld.code.map(|code|code.to_string()).unwrap_or_default();
                let signal = pld.signal.map(|code|code.to_string()).unwrap_or_default();
                let output = format!("进程已退出，CODE=[{}]，SIGNAL=[{}]", code, signal);
                ConsoleOut {
                    tag: ConsoleOutType::Terminated,
                    id: id.clone(), 
                    output
                }
            },
            _ => {
                ConsoleOut {
                    tag: ConsoleOutType::Error,
                    id: id.clone(), 
                    output: "<神秘的意外输出...>".into()
                }
            },
        };
        outbound.emit(CONSOLE_OUT, out).unwrap_or_default();
    }
}