use std::{error::Error, fmt};

use serenity::framework::standard::Reason;

#[derive(Clone, Debug)]
pub enum ButlerError {
    User(String),
    Log(String),
    UserAndLog { user: String, log: String },
}

impl ButlerError {
    pub fn user(s: &str) -> Self {
        Self::User(s.to_string())
    }

    pub fn log(s: &str) -> Self {
        Self::Log(s.to_string())
    }

    pub fn user_and_log(user: &str, log: &str) -> Self {
        Self::UserAndLog {
            user: user.to_string(),
            log: log.to_string(),
        }
    }
}

#[macro_export]
macro_rules! butler_log {
    ($err:expr, $ctx:expr, $msg:expr, $lvl:expr) => {
        use crate::emit;

        let error: &ButlerError = $err;
        let ctx: &Context = $ctx;
        let msg: &Message = $msg;
        match error {
            ButlerError::User(user) => emit!(msg.reply(&ctx.http, user).await, $lvl),
            ButlerError::Log(log) => event!($lvl, ?log),
            ButlerError::UserAndLog { user, log } => {
                emit!(msg.reply(&ctx.http, user).await, $lvl);
                event!($lvl, ?log);
            }
        }
    };
}

impl From<Reason> for ButlerError {
    fn from(r: Reason) -> ButlerError {
        match r {
            Reason::Log(s) => ButlerError::Log(s),
            Reason::User(s) => ButlerError::User(s),
            Reason::UserAndLog { user, log } => ButlerError::UserAndLog { user, log },
            _ => ButlerError::Log("Unknown reason".to_owned()),
        }
    }
}

impl From<ButlerError> for Reason {
    fn from(s: ButlerError) -> Reason {
        match s {
            ButlerError::Log(s) => Reason::Log(s),
            ButlerError::User(s) => Reason::User(s),
            ButlerError::UserAndLog { user, log } => Reason::UserAndLog { user, log },
        }
    }
}

impl fmt::Display for ButlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User(s) => write!(f, "user: {}", s),
            Self::Log(s) => write!(f, "log: {}", s),
            Self::UserAndLog { user, log } => write!(f, "user: {}, log: {}", user, log),
        }
    }
}

impl Error for ButlerError {}

#[macro_export]
macro_rules! emit {
    ($res:expr, $lvl:expr) => {
        if let Err(e) = $res {
            event!($lvl, %e, "Emit error")
        }
    };
}