use crate::models::{pokemon::Pokemon, trainer::{Trainer, TrainerJson}};

use super::pokemon::set_pokemon;

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

impl TrainerJson {
    pub fn into_trainer(self, pokemon_data: &std::collections::HashMap<&'static str, Pokemon>) -> Trainer {
        let pokemons = self.pokemons.unwrap_or_default()
            .iter()
            .filter_map(|id| pokemon_data.get(id.as_str()).cloned())
            .collect();

        let active_pokemon = self.active_pokemon.and_then(|id| pokemon_data.get(id.as_str()).cloned());

        Trainer {
            name: Box::leak(self.name.into_boxed_str()),
            pokemons,
            active_pokemon,
        }
    }
}