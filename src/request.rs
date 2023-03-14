use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Error, Name, Phase, Role, State};

/// 送受信されるリクエスト
#[derive(Serialize, Deserialize)]
pub enum Request {
    /// 投票: 生存者・日中・候補者(独自変数)
    Vote(Name),
    /// 殺害: 役職[人狼]・夜間・ターゲット[生存者]
    Kill(Name),
}
impl<'state> Request {
    /// Stateを更新する。
    pub fn apply_to(&self, state: &'state mut State, sender: &Name) -> Result<(), Error> {
        /// フェーズの確認をする
        macro_rules! assert_phase {
            ($expected:pat) => {
                let $expected = state.phase else {
                    return Err(Error::InvalidPhase {
                        found: state.phase.to_owned(), expected: stringify!($expected).to_string(),
                    });
                };
            };
        }
        /// 役職の確認をする
        macro_rules! assert_role {
            ($expected:pat) => {
                let $expected = state.role.get(sender).unwrap() else {
                    return Err(Error::InvalidPhase {
                        found: state.phase.to_owned(), expected: stringify!($expected).to_string(),
                    });
                };
            };
        }
        /// 生存者の確認をする。
        /// 生存確認が本人に対するものであるならば引数を省略する
        macro_rules! assert_survive {
            ($name: expr) => {
                if !state.survivors.contains($name) {
                    // 指定された名前が候補者に含まれていない場合。
                    return Err(Error::TargedExiledOrKilled($name.to_owned()));
                }
            };
            () => {
                if !state.survivors.contains(sender) {
                    return Err(Error::SurvivorsOnly);
                }
            };
        }

        match self {
            Self::Vote(vote_to) => {
                // 日中に限る
                assert_phase!(Phase::Day { ref mut votes, ref mut candidates });
                // 生存者に限る
                assert_survive!();

                if !candidates.contains(vote_to) {
                    // 指定された名前が候補者に含まれていない場合。
                    return Err(Error::CannotVoteToThisPlayer(vote_to.to_owned()));
                }
                // 投票リストの更新
                votes.insert(sender.to_owned(), vote_to.to_owned());
                if votes.len() == state.survivors.len() {
                    // 全員投票が終わったら
                    // 投票の集計
                    let mut max = 0; // 最大の得票数
                    candidates.clear(); // 最大得票の候補者を洗いだす
                    let mut votes_count = HashMap::new(); // 得票数
                    for candidate in votes.values() {
                        let counter = votes_count.entry(candidate).or_insert(0);
                        *counter += 1;
                        if max <= *counter {
                            if max < *counter {
                                max = *counter;
                                candidates.clear();
                            }
                            candidates.insert(candidate.to_owned());
                        }
                    }
                    if candidates.len() != 1 {
                        // 追放者が決まらなかった場合
                        // 決戦投票
                        votes.clear();
                        return Ok(());
                    }
                    let exiled_player = candidates.iter().next().unwrap(); // 追放される人
                    state.survivors.remove(exiled_player);
                    // 勝敗確認
                    if !state.judge() {
                        // 勝敗が決まらなかった場合
                        state.phase = Phase::Night;
                    }
                };
                Ok(())
            }
            Request::Kill(name) => {
                // 夜間に限る
                assert_phase!(Phase::Night);
                // 人狼に限る
                assert_role!(Role::Wolf);
                // ユーザーが生存しているか確認する
                assert_survive!();
                // ターゲットが生存しているか確認する
                assert_survive!(name);
                state.survivors.remove(name);
                Ok(())
            }
        }
    }
}
