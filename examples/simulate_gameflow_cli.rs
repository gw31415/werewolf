use std::io::Write;

use werewolf::{request, Error};

fn input(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        if std::io::stdin().read_line(&mut buf).is_ok() {
            break buf.trim().to_string();
        }
        println!("入力が壊れています。")
    }
}

fn main() -> Result<(), Error> {
    use werewolf::master::{config::*, *};
    let mut master = Master::from(Config {
        citizen: CitizenConfig { count: 1 },
        hunter: HunterConfig {
            count: 1,
            ..Default::default()
        },
        wolf: WolfConfig {
            count: 1,
            ..Default::default()
        },
        seer: SeerConfig {
            count: 1,
            ..Default::default()
        },
    });
    let tokens = ["はなこ", "たろう", "めじろ", "ぽち"]
        .iter()
        .map(|name| master.signup(name.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    master.start()?;
    for token in tokens.iter().cycle() {
        use werewolf::role::Role::*;
        use werewolf::state::State::*;

        let user = master.login(token)?;
        let name = user.name();

        println!("-------------------------------------------");

        match user.view_state() {
            End { winner: team } => {
                println!("Winner: {:?}", team);
                break;
            }
            Day {
                count,
                role,
                survivors,
                candidates,
                ..
            } => {
                println!("第{count}日目[昼] {name}");
                print!("生存者リスト: ");
                for target in survivors.iter() {
                    print!("{target}, ");
                }
                println!();

                println!(
                    "役職: {}",
                    match role.get(name).unwrap() {
                        Citizen => "市民".to_string(),
                        Hunter { .. } => "狩人".to_string(),
                        Wolf { .. } => "人狼".to_string(),
                        Seer { prediction } => {
                            let mut msg = "占い師 - ".to_string();
                            for (k, v) in prediction {
                                use werewolf::role::Team::*;
                                msg += &format!(
                                    "{k}:{}, ",
                                    match v {
                                        Wolf => "W",
                                        Citizen => "C",
                                    }
                                );
                            }
                            msg
                        }
                    }
                );

                if !survivors.contains(name) {
                    println!("{name}は死亡したため行動できません");
                    continue;
                }

                print!("候補者リスト: ");
                for target in candidates.iter() {
                    print!("{target}, ");
                }
                println!();
                loop {
                    let target = input("投票先 > ");
                    if candidates.contains(&target) {
                        user.execute(request::Vote { target })?;
                        break;
                    }
                    println!("{target} はリストに含まれていません。");
                }
            }
            Night {
                count,
                role,
                survivors,
                ..
            } => {
                println!("第{count}日目[夜]| {name}");
                print!("生存者リスト: ");
                for target in survivors.iter() {
                    print!("{target}, ");
                }
                println!();

                println!(
                    "役職: {}",
                    match role.get(name).unwrap() {
                        Citizen => "市民".to_string(),
                        Hunter { .. } => "狩人".to_string(),
                        Wolf { .. } => "人狼".to_string(),
                        Seer { prediction } => {
                            let mut msg = "占い師 - ".to_string();
                            for (k, v) in prediction {
                                use werewolf::role::Team::*;
                                msg += &format!(
                                    "{k}:{}, ",
                                    match v {
                                        Wolf => "W",
                                        Citizen => "C",
                                    }
                                );
                            }
                            msg
                        }
                    }
                );

                if !survivors.contains(name) {
                    println!("{name}は死亡したため行動できません");
                    continue;
                }

                match role.get(name).unwrap() {
                    Citizen => {
                        println!("あなたは何もすることがありません。");
                        user.execute(request::Skip())?;
                    }
                    Hunter { .. } => loop {
                        let target = input("守護先 > ");
                        if survivors.contains(&target) {
                            if &target != name {
                                user.execute(request::Guard { target })?;
                                break;
                            }
                            println!("自分自身を対象にできません。");
                            continue;
                        }
                        println!("{target} は生存者に含まれていません。");
                    },
                    Wolf { .. } => loop {
                        let target = input("殺害先 > ");
                        if survivors.contains(&target) {
                            if &target != name {
                                user.execute(request::Kill { target })?;
                                break;
                            }
                            println!("自分自身を対象にできません。");
                            continue;
                        }
                        println!("{target} は生存者に含まれていません。");
                    },
                    Seer { .. } => loop {
                        let target = input("占い先 > ");
                        if survivors.contains(&target) {
                            if &target != name {
                                user.execute(request::Divine { target })?;
                                break;
                            }
                            println!("自分自身を対象にできません。");
                            continue;
                        }
                        println!("{target} は生存者に含まれていません。");
                    },
                };
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
