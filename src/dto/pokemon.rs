use std::collections::HashMap;

use crate::{logic::rand::XorShift128, models::pokemon::{BufToStatus, Pokemon, PokemonJson, ElementType, Skill, Status}};
use super::skill::set_skill_list;

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

impl PokemonJson {
    pub fn into_pokemon(self, id: &'static str, skill_json: &HashMap<String, Skill>) -> Pokemon {
        let types: Vec<ElementType> = self.element
            .into_iter()
            .filter_map(|t| match t.as_str() {
                "Normal" => Some(ElementType::Normal),
                "Fire" => Some(ElementType::Fire),
                "Water" => Some(ElementType::Water),
                "Electric" => Some(ElementType::Electric),
                "Grass" => Some(ElementType::Grass),
                "Ice" => Some(ElementType::Ice),
                "Fighting" => Some(ElementType::Fighting),
                "Poison" => Some(ElementType::Poison),
                "Ground" => Some(ElementType::Ground),
                "Flying" => Some(ElementType::Flying),
                "Psychic" => Some(ElementType::Psychic),
                "Bug" => Some(ElementType::Bug),
                "Rock" => Some(ElementType::Rock),
                "Ghost" => Some(ElementType::Ghost),
                "Dragon" => Some(ElementType::Dragon),
                "Dark" => Some(ElementType::Dark),
                "Steel" => Some(ElementType::Steel),
                "Fairy" => Some(ElementType::Fairy),
                _ => None,
            })
            .collect();
        
        let skills: Vec<Skill> = self.skills
            .into_iter()
            .filter_map(|skill_name| skill_json.get(&skill_name).cloned())
            .collect();

        Pokemon {
            id,
            name: self.name,
            level: self.level,
            element: types,
            status: self.status,
            skills
        }
    }
}