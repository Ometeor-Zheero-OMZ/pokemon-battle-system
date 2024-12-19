use crate::models::{pokemon::Pokemon, trainer::Trainer};

#[allow(dead_code)]
pub trait TrainerActions {
    fn new(name: &'static str, pokemons: Vec<Pokemon>) -> Self;
    fn set_active_pokemon(&mut self, pokemon: Pokemon);
}

impl TrainerActions for Trainer {
    fn new(name: &'static str, pokemons: Vec<Pokemon>) -> Self {
        Trainer {
            name,
            pokemons,
            active_pokemon: None,
        }
    }

    /// 現在使用しているポケモンを設定
    /// 
    /// # 引数
    /// * `pokemon` - ポケモン
    fn set_active_pokemon(&mut self, pokemon: Pokemon) {
        self.active_pokemon = Some(pokemon);
    }
}