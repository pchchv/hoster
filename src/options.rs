//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Write everything
    All,
    /// No serving messages
    NoServeStatus,
    /// No startup messages, but yes auth data
    NoStartup,
    /// No auth data
    NoAuth,
}

impl From<u64> for LogLevel {
    fn from(raw: u64) -> LogLevel {
        match raw {
            0 => LogLevel::All,
            1 => LogLevel::NoServeStatus,
            2 => LogLevel::NoStartup,
            _ => LogLevel::NoAuth,
        }
    }
}