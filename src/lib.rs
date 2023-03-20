pub mod role;
pub mod state;
pub mod master;

mod error;
pub use error::Error;
use master::Config;
pub use master::Master;
pub use state::request::Request;

use crate::state::{Name, State};

/// リクエストを処理する権限
pub struct Permission<'master> {
    name: &'master Name,
    state: &'master mut State,
    config: &'master Config,
}
