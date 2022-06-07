
macro_rules! declare_events {
    ($($evt:ident)+) => {
        $(pub const $evt:&'static str = stringify!($evt);)+
    };
}

declare_events! {
    START_CORE
    CONSOLE_OUT
    OPEN_WEBAPP
    SPLASHSCREEN_UPDATE
    SPLASHSCREEN_CLOSE
    SPLASHSCREEN_OPEN
    WEBAPP_LOCK
    WEBAPP_UNLOCK
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ConsoleOutType {
    StdOut,
    StdErr,
    Error,
    Terminated,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConsoleOut {
    pub tag: ConsoleOutType,
    pub id: String,
    pub output: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StartCore {
    pub config_path: String,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenWebapp {
    pub url: String,
    pub name: String,
}