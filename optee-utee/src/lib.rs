pub use error::{Error, ErrorKind, Result};
pub use optee_utee_macros::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session,
};
pub use parameter::{ParamTypeFlags, Parameters};
pub use session::Session;
pub use trace::Trace;

#[macro_use]
mod macros;
mod error;
mod parameter;
mod session;
mod trace;
