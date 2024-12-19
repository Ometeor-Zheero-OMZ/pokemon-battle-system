use serde::{Deserialize, Serialize};

use super::pokemon::Pokemon;

/// トレーナーを表すJSON用構造体
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrainerJson {
    pub name: String,
    pub pokemons: Option<Vec<String>>,
    pub active_pokemon: Option<String>,
}

/// トレーナーを表す構造体
/// 
/// トレーナ名、所持ポケモン、現在場に出しているポケモンで構成
#[derive(Serialize, Clone, Debug)]
pub struct Trainer {
    pub name: &'static str,
    pub pokemons: Vec<Pokemon>,
    pub active_pokemon: Option<Pokemon>,
}