use crate::models::{pokemon::Pokemon, trainer::{Trainer, TrainerJson}};

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