use std::collections::HashMap;

use crate::{logic::rand::XorShift128, models::pokemon::{Pokemon, PokemonJson, ElementType, Skill}};


/// 敵ポケモンをランダムに選択
/// 
/// # 戻り値
/// * `Pokemon` - 初期化したポケモン
pub fn select_random_enemy_pokemon(pokemon_data: HashMap<&'static str, Pokemon>) -> Pokemon {
    let keys: Vec<&str> = pokemon_data.keys().cloned().collect();

    // 乱数生成
    let rand_idx = XorShift128::random_in_range(0, keys.len() as u64 - 1) as usize;

    // ランダムに選ばれたポケモン名でポケモンを取得
    let selected_name = keys[rand_idx];
    pokemon_data.get(selected_name).unwrap().clone()
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