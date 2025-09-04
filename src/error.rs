use std::fmt::Display;

use x11_dl::error::OpenError;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    XlibOpen(OpenError),
    IO(std::io::Error),
    Lua(mlua::Error),
    ConfigCouldNotBeCreated,
    HomeEnvNotSet,
    ReadLockError,
    _WriteLockError,
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
            AppError::HomeEnvNotSet => {
                f.write_str("Home env variable not set. Could not determen config location")
            }
            AppError::ReadLockError => f.write_str("Could not get RLock"),
            AppError::_WriteLockError => f.write_str("Could not get WLock"),
        }
    }
}

crate::from!({
    mlua::Error => Self::Lua,
    std::io::Error => Self::IO,
    OpenError => Self::XlibOpen
} => AppError);
