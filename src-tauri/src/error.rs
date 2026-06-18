//! Error type for commands.
//!
//! The frontend's `bindings.ts` wraps every invoke in a try/catch that turns a
//! rejected promise into `{ status: "error", error: <string> }`. So commands
//! only need to return errors as plain strings — hence `Result<T, String>`.

pub type AppResult<T> = Result<T, String>;
