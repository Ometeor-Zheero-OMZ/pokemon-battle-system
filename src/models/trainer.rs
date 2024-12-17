use crate::Pokemon;

/// トレーナーを表す構造体
/// 
/// トレーナ名、所持ポケモン、現在場に出しているポケモンで構成
pub struct Trainer {
    pub name: String,
    pub pokemons: Vec<Pokemon>,
    pub active_pokemon: Option<Pokemon>,
}