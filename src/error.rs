use std::fmt::Display;

use x11_dl::error::OpenError;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    XlibOpen(OpenError),
    IO(std::io::Error),
    Lua(mlua::Error),
    ConfigCouldNotBeCreated,
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::XlibOpen(e) => f.write_fmt(format_args!(
                "Could not open Xlib: {:#?} {}",
                e.kind(),
                e.detail()
            )),
            AppError::IO(error) => f.write_fmt(format_args!("IO Error: {error}")),
            AppError::Lua(error) => f.write_fmt(format_args!("Lua Error: {error}")),
            AppError::ConfigCouldNotBeCreated => f.write_str("Unable to create config file."),
        }
    }
}

impl From<OpenError> for AppError {
    fn from(value: OpenError) -> Self {
        Self::XlibOpen(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<mlua::Error> for AppError {
    fn from(value: mlua::Error) -> Self {
        Self::Lua(value)
    }
}
