pub mod role;
pub mod state;
pub mod master;

mod error;
pub use error::Error;
pub use master::Master;
pub use state::request::Request;

use crate::state::{Name, State};
use std::collections::HashMap;

/// リクエストを処理する権限
pub struct Permission<'master> {
    name: &'master Name,
    state: &'master mut State,
    client_states: &'master mut HashMap<Name, State>,
}
