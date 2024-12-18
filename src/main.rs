use std::{thread, time::Duration};

mod cli;
mod logic;
mod models;
mod services;

use logic::rand::XorShift128;
use models::trainer::Trainer;
use services::battle::trainer::TrainerActions;

use crate::cli::print::*;
use crate::models::pokemon::*;
use crate::services::battle::pokemon::*;

/// トレーナーを選択
/// 
/// # 引数
/// * `trainers` - トレーナーのリスト
/// 
/// # 戻り値
/// * `&mut Trainer` - 選択したトレーナー
fn select_trainer(trainers: &mut [Trainer]) -> &mut Trainer {
    clear_screen();
    print_letter_with_delay("トレーナーを選んでください：");
    for (idx, trainer) in trainers.iter().enumerate() {
        println!("{}: {}", idx + 1, trainer.name);
    }

    loop {
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: usize = match choice.trim().parse() {
            Ok(num) if num > 0 && num <= trainers.len() => num,
            _ => {
                println!("もう一度選びなおしてください。");
                continue;
            }
        };

        return &mut trainers[choice.saturating_sub(1)]
    }
}

/// ポケモンを選択
/// 
/// # 引数
/// * `trainer` - トレーナー
fn select_pokemon(trainer: &mut Trainer) {
    clear_screen();
    print_letter_with_delay(&format!("選んだトレーナー: {}", trainer.name));
    print_letter_with_delay("所持ポケモン：");

    for (idx, pokemon) in trainer.pokemons.iter().enumerate() {
        println!("{}: {}", idx + 1, pokemon.name);
    }

    loop {
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: usize = match choice.trim().parse() {
            Ok(num) if num > 0 && num <= trainer.pokemons.len() => num,
            _ => {
                println!("もう一度選びなおしてください。");
                continue;
            }
        };

        trainer.set_active_pokemon(trainer.pokemons[choice.saturating_sub(1)].clone());
        return;
    }
}

/// 敵ポケモンをランダムに選択
/// 
/// # 戻り値
/// * `Pokemon` - 初期化したポケモン
fn select_random_enemy_pokemon() -> Pokemon {
    let pokemon_names = vec![
        "pikachu",
        "metagross",
        "pidgey",
    ];

    // 乱数生成
    let rand_idx = XorShift128::random_in_range(0, pokemon_names.len() as u64 - 1) as usize;

    // ランダムに選ばれたポケモン名でポケモンを生成
    let selected_name = pokemon_names[rand_idx];

    set_pokemon(selected_name)
}

/// 技の設定
/// 
/// # 引数
/// * `pokemon_name` - ポケモン名
/// 
/// # 戻り値
/// * `Vec<Skill>` - 初期化した技のリスト
fn set_skill_list(pokemon_name: &str) -> Vec<Skill> {
    match pokemon_name {
        "pikachu" => {
            let tailwind = Skill {
                name: "しっぽをふる",
                base_atk: 0,
                accuracy: 100,
                priority: 0,
                skill_effect: Some(SkillEffect {
                    status_effect: StatusEffect {
                        target: StatusType::Def,
                        effect_value: -1,
                    },
                    target: Target::Enemy,
                }),
                type_: SkillType::ChangeStatus,
            };
            let thundershock = Skill {
                name: "でんきショック",
                base_atk: 40,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                type_: SkillType::SpecialAttack,
            };
            let quick_attack = Skill {
                name: "でんこうせっか",
                base_atk: 40,
                accuracy: 100,
                priority: 2,
                skill_effect: None,
                type_: SkillType::PhysicalAttack,
            };
            let tackle = Skill {
                name: "たいあたり",
                base_atk: 40,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                type_: SkillType::PhysicalAttack,
            };
        
            vec![tailwind, thundershock, quick_attack, tackle]
        },
        "metagross" => {
            let bullet_punch = Skill {
                name: "バレットパンチ",
                base_atk: 40,
                accuracy: 100,
                priority: 2,
                skill_effect: None,
                type_: SkillType::PhysicalAttack,
            };
            let earthquake = Skill {
                name: "じしん",
                base_atk: 100,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                type_: SkillType::PhysicalAttack,
            };
            let ice_punch = Skill {
                name: "れいとうパンチ",
                base_atk: 75,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                type_: SkillType::PhysicalAttack,
            };
            let psychic_fangs = Skill {
                name: "サイコファング",
                base_atk: 85,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                type_: SkillType::PhysicalAttack,
            };
        
            vec![bullet_punch, earthquake, ice_punch, psychic_fangs]
        },
        "pidgey" => {
            let thundershock = Skill {
                name: "ゴッドバード",
                base_atk: 120,
                accuracy: 50,
                priority: 0,
                skill_effect: None,
                type_: SkillType::SpecialAttack,
            };
            let growl = Skill {
                name: "なきごえ",
                base_atk: 0,
                accuracy: 100,
                priority: 0,
                skill_effect: Some(SkillEffect {
                    status_effect: StatusEffect {
                        target: StatusType::Atk,
                        effect_value: -1,
                    },
                    target: Target::Enemy,
                }),
                type_: SkillType::ChangeStatus,
            };
            let quick_attack = Skill {
                name: "でんこうせっか",
                base_atk: 40,
                accuracy: 100,
                priority: 2,
                skill_effect: None,
                type_: SkillType::PhysicalAttack,
            };
            let tackle = Skill {
                name: "つのドリル",
                base_atk: 40,
                accuracy: 30,
                priority: 0,
                skill_effect: None,
                type_: SkillType::OneHitKO,
            };
        
            vec![thundershock, growl, quick_attack, tackle]
        }
        _ => {
            eprintln!("{}の技が設定されていません。", pokemon_name);
            unreachable!()
        }
    }
}

/// 技の設定
/// 
/// # 引数
/// * `trainer_name` - トレーナー名
/// 
/// # 戻り値
/// * `Trainer` - 初期化したトレーナー
fn set_trainer(trainer_name: &str) -> Trainer {
    match trainer_name {
        "satoshi" => {
            let pokemon = set_pokemon("pikachu");

            Trainer {
                name: "サトシ",
                pokemons: vec![pokemon.clone()],
                active_pokemon: Some(pokemon)
            }
        }
        "daigo" => {
            let pokemon = set_pokemon("metagross");

            Trainer {
                name: "ダイゴ",
                pokemons: vec![pokemon.clone()],
                active_pokemon: Some(pokemon)
            }
        }
        "short-pants-boy" => {
            let pokemon = set_pokemon("pidgey");

            Trainer {
                name: "たんぱんこぞうのミノル",
                pokemons: vec![pokemon.clone()],
                active_pokemon: Some(pokemon)
            }
        }
        _ => {
            eprintln!("トレーナーが設定されていません： {}", trainer_name);
            unreachable!()
        }
    }
}

/// ポケモンの設定
/// 
/// # 引数
/// * `pokemon_name` - ポケモン名
/// 
/// # 戻り値
/// * `Pokemon` - 初期化したポケモン
fn set_pokemon(pokemon_name: &str) -> Pokemon {
    let selected_pokemon;

    match pokemon_name {
        "pikachu" => {
            let skill_list = set_skill_list(pokemon_name);

            selected_pokemon = Pokemon {
                name: "ピカチュウ",
                level: 10,
                type_: vec![PokemonType::Electric],
                status: Status {
                    hp: BufToStatus { value: 30, buff: 0 },
                    atk: BufToStatus { value: 19, buff: 0 },
                    def: BufToStatus { value: 16, buff: 0 },
                    sp_atk: BufToStatus { value: 18, buff: 0 },
                    sp_def: BufToStatus { value: 18, buff: 0 },
                    spd: BufToStatus { value: 26, buff: 0 },
                    current_hp: 30,
                },
                skills: skill_list,
            };
        }
        "metagross" => {
            let skill_list = set_skill_list(pokemon_name);

            selected_pokemon = Pokemon {
                name: "メタグロス",
                level: 68,
                type_: vec![PokemonType::Steel, PokemonType::Psychic],
                status: Status {
                    hp: BufToStatus { value: 255, buff: 0 },
                    atk: BufToStatus { value: 196, buff: 0 },
                    def: BufToStatus { value: 173, buff: 0 },
                    sp_atk: BufToStatus { value: 96, buff: 0 },
                    sp_def: BufToStatus { value: 124, buff: 0 },
                    spd: BufToStatus { value: 132, buff: 0 },
                    current_hp: 255,
                },
                skills: skill_list
            }
        }
        "pidgey" => {
            let skill_list = set_skill_list(pokemon_name);

            selected_pokemon = Pokemon {
                name: "ポッポ",
                level: 8,
                type_: vec![PokemonType::Normal, PokemonType::Flying],
                status: Status {
                    hp: BufToStatus { value: 25, buff: 0 },
                    atk: BufToStatus { value: 16, buff: 0 },
                    def: BufToStatus { value: 13, buff: 0 },
                    sp_atk: BufToStatus { value: 15, buff: 0 },
                    sp_def: BufToStatus { value: 15, buff: 0 },
                    spd: BufToStatus { value: 23, buff: 0 },
                    current_hp: 25,
                },
                skills: skill_list
            };
        }
        _ => {
            eprintln!("ポケモンが設定されていません： {}", pokemon_name);
            unreachable!()
        }
    }

    selected_pokemon
}

/// バトル開始
/// 
/// # 引数
/// * `self_` - 自分のポケモン
/// * `enemy` - 敵のポケモン
fn start_battle(self_: &mut Pokemon, enemy: &mut Pokemon) {
    // 画面クリア
    clear_screen();

    // テキストアニメーションを描画
    print_letter_with_delay("あ!　やせいの");
    print_letter_with_delay(&format!("{}が　とびだしてきた！", enemy.name));

    // 演出上の遅延
    thread::sleep(Duration::from_millis(500));

    // 現在のHPバーを描画
    clear_and_print_current_battle_status(&self_, &enemy);

    while self_.status.current_hp > 0 && enemy.status.current_hp > 0 {
        // 技リストの描画
        thread::sleep(Duration::from_millis(1000));
        print_skill_list(&self_);

        // 自分のターン
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let skill_idx = match input.trim() {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            _ => continue,
        };

        // 攻撃
        clear_and_print_current_battle_status(&self_, &enemy);
        self_.attack(skill_idx, enemy);
        clear_and_print_current_battle_status(self_, enemy);

        // 相手のターン
        let skill_idx = XorShift128::random_in_range(0, 4) as usize;

        if enemy.status.current_hp > 0 {
            enemy.attack(skill_idx, self_);
            clear_and_print_current_battle_status(self_, enemy);
        }
    }

    if self_.status.current_hp > 0 {
        print_letter_with_delay(&format!("てきの　{}　はたおれた！", enemy.name));
        thread::sleep(Duration::from_millis(2000));
        clear_and_print_current_battle_status(&self_, &enemy);
        print_letter_with_delay(&format!("{}との　しょうぶに かった!", enemy.name));
    } else {
        print_letter_with_delay(&format!("{}は　たおれた！", self_.name));
        thread::sleep(Duration::from_millis(2000));
        clear_and_print_current_battle_status(&self_, &enemy);
        print_letter_with_delay(&format!("めのまえが　まっくらに　なった"));
    }
}

// TODO ポケモンのすばやさ技の優先度を比較して行動順を決定
// fn battle_round(self_: &mut Pokemon, enemy: &mut Pokemon) {
//     // 並び順の取得
//     let order = Pokemon::get_battle_order(self_.clone(), enemy.clone());

//     // ポケモンのターンを順番に処理
//     for pokemon in &order {
//         // 自分のターン
//         if pokemon == self_ {
//             let mut input = String::new();
//             std::io::stdin()
//                 .read_line(&mut input)
//                 .expect("Failed to read line");

//             let skill_idx = match input.trim() {
//                 "1" => 0,
//                 "2" => 1,
//                 "3" => 2,
//                 "4" => 3,
//                 "5" => 4,
//                 _ => continue,
//             };

//             // 攻撃を行う
//             clear_and_print_current_battle_status(&self_, &enemy);
//             self_.attack(skill_idx, enemy);
//             clear_and_print_current_battle_status(self_, enemy);
//         } else if pokemon == enemy {
//             // 相手のターン
//             let skill_idx = XorShift128::random_in_range(0, 4) as usize;

//             if enemy.status.current_hp > 0 {
//                 enemy.attack(skill_idx, self_);
//                 clear_and_print_current_battle_status(self_, enemy);
//             }
//         }
//     }
// }

fn main() {
    // トレーナーを選択
    let mut trainers = vec![
        set_trainer("satoshi"),
        set_trainer("daigo"),
        set_trainer("short-pants-boy")
    ];
    let mut selected_trainer = select_trainer(&mut trainers);

    // トレーナーが選んだポケモンを選択
    select_pokemon(&mut selected_trainer);

    // バトルするポケモンを取得
    let mut enemy_pokemon = select_random_enemy_pokemon();
    let mut self_pokemon = selected_trainer.active_pokemon.clone().unwrap();

    // バトル開始
    start_battle(&mut self_pokemon, &mut enemy_pokemon);
}
