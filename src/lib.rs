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
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

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

        /// 勝敗の決定
        macro_rules! judge {
            ($survivors: expr, $role: expr) => {
                let mut wolves = 0;
                for name in $survivors.iter() {
                    if let Some(role::Role::Wolf { .. }) = $role.get(name) {
                        wolves += 1;
                    }
                }
                use role::Team::{Citizen, Wolf};
                if wolves * 2 >= $survivors.len() {
                    *state = State::End(Wolf).into();
                    return Ok(());
                } else if wolves == 0 {
                    *state = State::End(Citizen).into();
                    return Ok(());
                }
            };
        }

        match state.get_mut().clone() {
            State::Waiting(next_config) => {
                // 設定が変更されたら書きかえる
                *config = next_config;
            }
            State::Day {
                count,
                role,
                waiting,
                mut survivors,
                votes,
                ..
            } => {
                if waiting.is_empty() {
                    let candidates: HashSet<_> = {
                        // 最大票数獲得者の絞りこみ
                        let mut freqs = HashMap::new();
                        for target in votes.values() {
                            *freqs.entry(target).or_insert(0) += 1;
                        }

                        let max_count = freqs.values().cloned().max().unwrap_or(0);
                        freqs
                            .into_iter()
                            .filter_map(|(candidate, count)| {
                                if count == max_count {
                                    Some(candidate.clone())
                                } else {
                                    None
                                }
                            })
                            .collect()
                    };

                    if candidates.len() == 1 {
                        // 候補者が一人に定まった場合

                        // 追放
                        survivors.remove(&candidates.iter().next().unwrap().to_owned());

                        // 勝敗判定
                        judge!(survivors, role);

                        // 次の夜がやってきました。
                        *state = State::Night {
                            count: count + 1,
                            role,
                            waiting: survivors.clone(),
                            survivors,
                        }
                        .into();
                    } else {
                        // 決選投票
                        *state = State::Day {
                            count,
                            role,
                            waiting: survivors.clone(),
                            survivors,
                            votes: HashMap::new(),
                            candidates,
                        }
                        .into();
                    }
                }
            }
            State::Night {
                count,
                role,
                waiting,
                mut survivors,
            } => {
                if waiting.is_empty() {
                    {
                        // 殺害
                        let (mut saves, mut targets) = (Vec::new(), Vec::new());
                        for name in survivors.iter() {
                            use role::Role::{Hunter, Wolf};
                            match role.get(name) {
                                Some(Hunter { saving: Some(save) }) => {
                                    saves.push(save);
                                }
                                Some(Wolf {
                                    killing: Some(kill),
                                }) => {
                                    targets.push(kill);
                                }
                                _ => (),
                            }
                        }
                        for kill in targets {
                            if !saves.contains(&kill) {
                                // 守られていない人
                                survivors.remove(kill);
                            }
                        }
                    }

                    // 勝敗判定
                    judge!(survivors, role);

                    // 次の夜がやってきました。
                    *state = State::Day {
                        count: count + 1,
                        role,
                        waiting: survivors.clone(),
                        candidates: survivors.clone(),
                        votes: HashMap::new(),
                        survivors,
                    }
                    .into();
                }
            }
            State::End(_) => {}
        }
        Ok(())
    }

    /// パーミッション元ユーザの名前を返す。
    pub fn name(&self) -> &Name {
        self.name
    }

    /// Stateをクローンし、そのユーザーが閲覧できる範囲にフィルターして返す
    pub fn view_state(&self) -> State {
        let state = unsafe { (*self.state.as_ptr()).clone() };
        use State::*;
        match state {
            Waiting(_) | End(_) => state,
            Day {
                count,
                mut role,
                waiting,
                survivors,
                votes,
                candidates,
            } => {

                // 自分のロールのみにフィルターする
                role = role.drain().filter(|(k, _)| k == self.name).collect();

                Day {
                    count,
                    role,
                    waiting,
                    survivors,
                    votes,
                    candidates,
                }
            }
            Night {
                count,
                mut role,
                waiting,
                survivors,
            } => {

                // 自分のロールのみにフィルターする
                role = role.drain().filter(|(k, _)| k == self.name).collect();

                Night {
                    count,
                    role,
                    waiting,
                    survivors,
                }
            }
        }
    }
}
