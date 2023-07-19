mod error;

use super::{Name, State};
use crate::{
    master::Config,
    role::{Role, Team},
};
pub use error::Error;

use serde::{Deserialize, Serialize};

/// 状況の確認をする
macro_rules! assert_state {
    ($expected: pat, $state: expr) => {
        let $expected = $state else {
            return Err(
                Error::InvalidState { found: Box::new($state.to_owned()), expected: stringify!($expected).to_owned() }
            );
        };
    };
}
/// 役職の確認をする
macro_rules! assert_role {
    ($expected:pat, $role: expr) => {
        let $expected = $role else {
            return Err(Error::InvalidRole {
                found: Box::new($role.to_owned()), expected: stringify!($expected).to_owned(),
            });
        };
    };
}

/// リクエストが満たすべきトレイト要件
pub trait Request<'req>: Serialize + Deserialize<'req> {
    /// リクエストの挙動を規定する動作
    fn modify(self, name: &Name, state: &mut State, config: &Config) -> Result<(), Error>;
}

/// 待機中にゲーム設定を更新する
#[derive(Serialize, Deserialize)]
pub struct UpdateConfig {
    /// 新しいゲーム設定
    pub config: Config,
}

impl Request<'_> for UpdateConfig {
    fn modify(self, _: &Name, state: &mut State, _: &Config) -> Result<(), Error> {
        assert_state!(State::Waiting(ref mut config), state);
        *config = self.config;
        Ok(())
    }
}

/// 昼に通報者に投票する
#[derive(Serialize, Deserialize)]
pub struct Vote {
    /// 投票先
    pub target: Name,
}

impl Request<'_> for Vote {
    fn modify(self, name: &Name, state: &mut State, _: &Config) -> Result<(), Error> {
        assert_state!(
            State::Day {
                waiting,
                survivors,
                votes,
                candidates,
                ..
            },
            state
        );
        if !candidates.contains(&self.target) {
            return Err(Error::InvalidTarget(self.target));
        }
        if !survivors.contains(name) {
            return Err(Error::SurvivorsOnly);
        }
        if !waiting.contains(name) {
            return Err(Error::MultipleActions);
        }
        votes.insert(name.to_owned(), self.target);
        waiting.remove(name);
        Ok(())
    }
}

/// 夜に住民を殺害する
#[derive(Serialize, Deserialize)]
pub struct Kill {
    /// 殺害先
    pub target: Name,
}

impl Request<'_> for Kill {
    fn modify(self, name: &Name, state: &mut State, _: &Config) -> Result<(), Error> {
        assert_state!(
            State::Night {
                role,
                waiting,
                survivors,
                ..
            },
            state
        );
        assert_role!(Role::Wolf { ref mut killing }, role.get_mut(name).unwrap());
        if !waiting.contains(name) {
            return Err(Error::MultipleActions);
        }
        if !survivors.contains(name) {
            return Err(Error::SurvivorsOnly);
        }
        if !survivors.contains(&self.target) {
            return Err(Error::InvalidTarget(self.target));
        }
        *killing = Some(self.target);
        waiting.remove(name);
        Ok(())
    }
}

/// 夜に住民を占う
#[derive(Serialize, Deserialize)]
pub struct Divine {
    /// 占い先
    pub target: Name,
}

impl Request<'_> for Divine {
    fn modify(self, name: &Name, state: &mut State, _: &Config) -> Result<(), Error> {
        assert_state!(
            State::Night {
                role,
                waiting,
                survivors,
                ..
            },
            state
        );
        let target_is_wolf = matches!(role.get(&self.target).unwrap(), Role::Wolf { .. });
        assert_role!(Role::Seer { prediction }, role.get_mut(name).unwrap());
        if !waiting.contains(name) {
            return Err(Error::MultipleActions);
        }
        if !survivors.contains(name) {
            return Err(Error::SurvivorsOnly);
        }
        prediction.insert(
            self.target,
            if target_is_wolf {
                Team::Wolf
            } else {
                Team::Citizen
            },
        );
        waiting.remove(name);
        Ok(())
    }
}

/// 夜に住民を防護する
#[derive(Serialize, Deserialize)]
pub struct Save {
    /// 防護先
    pub target: Name,
}

impl Request<'_> for Save {
    fn modify(self, name: &Name, state: &mut State, _: &Config) -> Result<(), Error> {
        assert_state!(
            State::Night {
                role,
                waiting,
                survivors,
                ..
            },
            state
        );
        assert_role!(Role::Hunter { ref mut saving }, role.get_mut(name).unwrap());
        if !waiting.contains(name) {
            return Err(Error::MultipleActions);
        }
        if !survivors.contains(name) {
            return Err(Error::SurvivorsOnly);
        }
        if !survivors.contains(&self.target) {
            return Err(Error::InvalidTarget(self.target));
        }
        *saving = Some(self.target);
        waiting.remove(name);
        Ok(())
    }
}
