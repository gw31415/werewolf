use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Name, Phase, Role, State};
use crate::{Error, Team};

/// 送受信されるリクエスト
#[derive(Serialize, Deserialize)]
pub enum Request {
    /// 投票: 生存者・日中・候補者(独自変数)
    Vote(Name),
    /// 殺害: 役職[人狼]・夜間・ターゲット[生存者]・夜間1回のみ
    Kill(Name),
    /// 占い: 役職[占い師]・夜間・ターゲット[生存者、占っていない人]・夜間1回のみ
    Divine(Name),
}
impl<'state> Request {
    /// Stateを更新する。
    pub(crate) fn apply_to(&self, state: &'state mut State, sender: &Name) -> Result<(), Error> {
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
                let $expected = state.role.get_mut(sender).unwrap() else {
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
                    return Err(Error::InvalidTarget($name.to_owned()));
                }
            };
            () => {
                if !state.survivors.contains(sender) {
                    return Err(Error::SurvivorsOnly);
                }
            };
        }

        // リクエスト固有の処理を行う。
        // Phaseの変更はリクエストに依存しないものが多いので、
        // そういうものは後のmatch移譲する。
        match self {
            Self::Vote(vote_to) => {
                // 日中に限る
                assert_phase!(Phase::Day { ref mut votes, ref mut candidates, .. });
                // 生存者に限る
                assert_survive!();

                if !candidates.contains(vote_to) {
                    // 指定された名前が候補者に含まれていない場合。
                    return Err(Error::InvalidTarget(vote_to.to_owned()));
                }
                // 投票リストの更新
                votes.insert(sender.to_owned(), vote_to.to_owned());
            }
            Self::Kill(name) => {
                // 夜間に限る
                assert_phase!(Phase::Night{ ref mut waiting, .. });
                // 人狼に限る
                assert_role!(Role::Wolf(ref mut option));
                // ユーザーが生存しているか確認する
                assert_survive!();
                // ターゲットが生存しているか確認する
                assert_survive!(name);
                // 行動済みの場合はエラー
                if !waiting.contains(sender) {
                    return Err(Error::MultipleActions);
                }
                *option = Some(name.to_owned());
                // タスク終了の通知
                waiting.remove(sender);
            }
            Self::Divine(name) => {
                // 夜間に限る
                assert_phase!(Phase::Night{ ref mut waiting, .. });
                // ユーザーが生存しているか確認する
                assert_survive!();
                // ターゲットが生存しているか確認する
                assert_survive!(name);
                // 行動済みの場合はエラー
                if !waiting.contains(sender) {
                    return Err(Error::MultipleActions);
                }
                // ターゲットが人狼か否か
                let target_is_wolf = matches!(state.role.get(name).unwrap(), Role::Wolf(_));
                // 占い師に限る
                assert_role!(Role::Seer(ref mut expected));
                // 既に占っていた場合はエラー
                if expected.contains_key(name) {
                    return Err(Error::InvalidTarget(name.to_owned()));
                }
                expected.insert(name.to_owned(), target_is_wolf);
                // タスク終了の通知
                waiting.remove(sender);
            }
        }

        // Phaseの推移
        match state.phase.clone() {
            Phase::Night { count, waiting } => {
                if waiting.is_empty() {
                    // 行動待ちがいない場合
                    // 夜のうちにキューされた殺害リストを適用する
                    let mut kill_list: Vec<&String> = Vec::new();
                    for survivor in state.survivors.iter() {
                        if let Role::Wolf(Some(target)) = state.role.get(survivor).unwrap() {
                            kill_list.push(target);
                        }
                    }
                    for name in kill_list {
                        state.survivors.remove(name);
                    }
                    for survivor in state.survivors.iter() {
                        if let Role::Wolf(option) = state.role.get_mut(survivor).unwrap() {
                            if option.is_some() {
                                *option = None;
                            }
                        }
                    }
                    if judge(state) {
                        // 勝敗確認
                        state.phase = Phase::Day {
                            count,
                            votes: HashMap::new(),
                            candidates: state.survivors.clone(),
                        }
                    }
                }
            }
            Phase::Day {
                count,
                ref mut candidates,
                ref mut votes,
            } => {
                // 全員投票が終わったら
                if votes.len() == state.survivors.len() {
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
                    if judge(state) {
                        // 勝敗が決まらなかった場合
                        state.phase = Phase::Night {
                            count: count + 1,
                            waiting: state.survivors.clone(),
                        };
                    }
                }
            }
            Phase::Waiting | Phase::End(_) => {}
        }
        Ok(())
    }
}

/// 勝敗を確認する。終了した場合はPhaseをEndにし、trueを返す。
/// 終了しなかった場合はfalseを返す。
fn judge(state: &mut State) -> bool {
    let State {
        ref survivors,
        ref role,
        ref mut phase,
        ..
    } = state;

    // 陣営の数を数える
    let mut iter = survivors.iter();
    let (mut wolf_count, mut citizen_count) = (0usize, 0usize);
    let mut check_wolf_win_after_increment = |survivor: &Name| {
        let role = role.get(survivor).unwrap();
        match role.team() {
            Team::Wolf => {
                wolf_count += 1;
            }
            Team::Citizen => {
                citizen_count += 1;
            }
        }
        if wolf_count >= citizen_count {
            *phase = Phase::End(Team::Wolf);
            return true;
        }
        false
    };

    'wolf_presence_check: {
        // 市民の勝利条件確認(人狼の存在の有無)
        for survivor in &mut iter {
            if check_wolf_win_after_increment(survivor) {
                return true;
            }
            if let Role::Wolf(_) = state.role.get(survivor).unwrap() {
                break 'wolf_presence_check;
            }
        }
        // 人狼が存在しなければ市民陣営の勝利
        *phase = Phase::End(Team::Citizen);
        return true;
    }
    for survivor in &mut iter {
        if check_wolf_win_after_increment(survivor) {
            return true;
        }
    }
    // この時点で、人狼の勝利条件確認の終了 (人狼>市民)
    false
}
