use crate::{cli::{battle::start_battle, prompt::{select_pokemon, select_trainer}}, dto::skill::set_skill_list, logic::rand::XorShift128, models::{pokemon::{BufToStatus, ElementType, Pokemon, Status}, trainer::Trainer}};

/// データはハードコードで設定
pub fn test() {
    // トレーナーを選択
    let mut trainers = vec![
        set_trainer("satoshi"),
        set_trainer("daigo"),
        set_trainer("short_pants_boy")
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

/// ポケモンの設定
/// 
/// # 引数
/// * `pokemon_name` - ポケモン名
/// 
/// # 戻り値
/// * `Pokemon` - 初期化したポケモン
pub fn set_pokemon(pokemon_name: &str) -> Pokemon {
    let selected_pokemon;

    match pokemon_name {
        "pikachu" => {
            let skill_list = set_skill_list(pokemon_name);

            selected_pokemon = Pokemon {
                id: "0",
                name: "ピカチュウ".to_string(),
                level: 10,
                element: vec![ElementType::Electric],
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
                id: "1",
                name: "メタグロス".to_string(),
                level: 68,
                element: vec![ElementType::Steel, ElementType::Psychic],
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
                id: "2",
                name: "ポッポ".to_string(),
                level: 8,
                element: vec![ElementType::Normal, ElementType::Flying],
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

/// 敵ポケモンをランダムに選択
/// 
/// # 戻り値
/// * `Pokemon` - 初期化したポケモン
pub fn select_random_enemy_pokemon() -> Pokemon {
    // TODO JSONから取得したポケモンを設定
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
/// * `trainer_name` - トレーナー名
/// 
/// # 戻り値
/// * `Trainer` - 初期化したトレーナー
pub fn set_trainer(trainer_name: &str) -> Trainer {
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
        "short_pants_boy" => {
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