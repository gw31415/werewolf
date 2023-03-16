pub mod error;
pub mod role;
pub mod state;

pub use error::Error;

mod master;
pub use master::{Master, Token, TOKEN_LENGTH};

use crate::state::{Name, State};
use std::collections::HashMap;

/// リクエストを処理する権限
pub struct Permission<'master> {
    name: &'master Name,
    state: &'master mut State,
    client_states: &'master mut HashMap<&'master Name, State>,
}
