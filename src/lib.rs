pub mod master;
pub mod request;
pub mod role;
pub mod state;

mod error;

pub use error::Error;
pub use master::Master;

use crate::state::{Name, State};
use master::Config;
use request::Request;
use std::cell::Cell;

/// リクエストを処理する権限
/// Permissionが作成されると、Permissionがドロップされるまで
/// Masterはイミュータブルになることに留意。
pub struct Permission<'master> {
    name: &'master Name,
    state: &'master mut Cell<State>,
    config: &'master mut Config,
}

impl<'master> Permission<'master> {
    /// リクエストを実行する
    pub fn execute(self, req: impl Request<'master>) -> Result<(), Error> {
        let Self {
            name,
            state,
            config,
        } = self;
        req.modify(name, state.get_mut(), config)?;
        // 設定が変更されたら書きかえる
        if let State::Waiting(next_config) = state.get_mut().to_owned() {
            *config = next_config;
        }
        Ok(())
    }
}
