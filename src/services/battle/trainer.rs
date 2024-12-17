use crate::{models::trainer::Trainer, Pokemon};

#[allow(dead_code)]
pub trait TrainerActions {
    fn new(name: &str, pokemons: Vec<Pokemon>) -> Self;
    fn set_active_pokemon(&mut self, pokemon: Pokemon);
}

impl TrainerActions for Trainer {
    fn new(name: &str, pokemons: Vec<Pokemon>) -> Self {
        Trainer {
            name: name.to_string(),
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