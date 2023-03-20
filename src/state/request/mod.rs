mod error;

use super::{Name, State};
use crate::role::Role;
use crate::Permission;
pub use error::Error;

use serde::{Deserialize, Serialize};

/// 送受信されるリクエスト
#[derive(Serialize, Deserialize)]
pub enum Request {
    /// 投票
    // 生存者・日中・候補者(独自変数)
    Vote(Name),
    /// 殺害
    // 役職[人狼]・夜間・ターゲット[生存者]・夜間1回のみ
    Kill(Name),
    /// 占い
    // 役職[占い師]・夜間・ターゲット[生存者、占っていない人]・夜間1回のみ
    Divine(Name),
}

impl<'state> Request {
    /// Stateを更新する。
    pub fn execute(&self, permission: Permission<'state>) -> Result<(), Error> {
        let Permission { name, state, config } = permission;
        /// 状況の確認をする
        macro_rules! assert_state {
            ($expected:pat) => {
                let $expected = state else {
                    return Err(Error::InvalidState {
                        found: Box::new(state.to_owned()),
                        expected: stringify!($expected).to_owned(),
                    });
                };
            };
        }
        /// 役職の確認をする
        macro_rules! assert_role {
            ($role: expr, $expected:pat) => {
                let $expected = $role else {
                    return Err(Error::InvalidRole {
                        found: Box::new($role.to_owned()), expected: stringify!($expected).to_owned(),
                    });
                };
            };
        }
        match self {
            Self::Vote(target) => {
                assert_state!(State::Day {
                    waiting,
                    survivors,
                    votes,
                    candidates,
                    ..
                });
                if !candidates.contains(target) {
                    return Err(Error::InvalidTarget(target.to_owned()));
                }
                if !survivors.contains(name) {
                    return Err(Error::SurvivorsOnly);
                }
                if !waiting.contains(name) {
                    return Err(Error::MultipleActions);
                }
                votes.insert(name.to_owned(), target.to_owned());
                waiting.remove(name);
            }
            Self::Kill(target) => {
                assert_state!(State::Night {
                    role,
                    waiting,
                    survivors,
                    next_survivors,
                    ..
                });
                assert_role!(role.get(name).unwrap(), Role::Wolf);
                if !waiting.contains(name) {
                    return Err(Error::MultipleActions);
                }
                if !survivors.contains(name) {
                    return Err(Error::SurvivorsOnly);
                }
                if !survivors.contains(target) {
                    return Err(Error::InvalidTarget(target.to_owned()));
                }
                next_survivors.remove(target);
            }
            Self::Divine(target) => {
                assert_state!(State::Night {
                    role,
                    waiting,
                    survivors,
                    ..
                });
                let target_is_wolf = matches!(role.get(target).unwrap(), Role::Wolf);
                assert_role!(role.get_mut(name).unwrap(), Role::Seer(prediction));
                if !waiting.contains(name) {
                    return Err(Error::MultipleActions);
                }
                if !survivors.contains(name) {
                    return Err(Error::SurvivorsOnly);
                }
                prediction.insert(target.to_owned(), target_is_wolf);
            }
        }
        Ok(())
    }
}
