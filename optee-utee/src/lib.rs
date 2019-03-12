pub use error::{Error, ErrorKind, Result};
pub use parameter::{ParamTypeFlags, Parameters};
pub use trace::Trace;
pub use optee_utee_macros::{ta_create,ta_destroy,ta_open_session,ta_close_session,ta_invoke_command};

#[macro_use]
mod macros;
mod error;
mod parameter;
mod trace;
