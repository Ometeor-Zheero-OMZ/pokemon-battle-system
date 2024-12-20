use dotenvy::dotenv;
use test::hard_coded::test;
use std::{collections::HashMap, env};

use cli::{battle::start_battle, prompt::{select_pokemon, select_trainer}};
use dto::pokemon::select_random_enemy_pokemon;
use file::json::read_json;
use models::{pokemon::{Pokemon, PokemonJson, Skill}, trainer::{Trainer, TrainerJson}};

mod cli;
mod dto;
mod file;
mod logic;
mod models;
mod services;
mod test;

/// データはJSONから取得
fn init() {
    let trainer_json = match read_json::<TrainerJson>("./json/trainer_data.json") {
        Ok(trainer) => trainer,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let pokemon_json = match read_json::<PokemonJson>("./json/pokemon_data.json") {
        Ok(pokemon) => pokemon,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let skill_json = match read_json::<Skill>("./json/skill_data.json") {
        Ok(skill) => skill,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    // ポケモンデータを変換
    let mut pokemon_data: HashMap<&'static str, Pokemon> = HashMap::new();
    for (id, pokemon) in pokemon_json.into_iter() {
        // 図鑑番号
        let id_static = Box::leak(id.into_boxed_str());

        // Pokemon を HashMap に追加
        pokemon_data.insert(id_static, pokemon.into_pokemon(id_static, &skill_json));
    }

    // トレーナーデータを変換
    let mut trainers: Vec<Trainer> = trainer_json
        .into_iter()
        .map(|(_, trainer_json)| trainer_json.into_trainer(&pokemon_data))
        .collect();

    let mut selected_trainer = select_trainer(&mut trainers);

    // トレーナーが選んだポケモンを選択
    select_pokemon(&mut selected_trainer);

    // バトルするポケモンを取得
    let mut enemy_pokemon = select_random_enemy_pokemon(pokemon_data);
    let mut self_pokemon = selected_trainer.active_pokemon.clone().unwrap();

    // バトル開始
    start_battle(&mut self_pokemon, &mut enemy_pokemon);    
}

fn main() {
    dotenv().ok();
    let run_mode: &str = &env::var("RUN_MODE").expect("RUN_MODE が設定されていません。");

    match run_mode {
        // ハードコードしたデータで実行
        "hard_code" => test(),
        // JSON からデータを取得して実行
        "json" => init(),
        // バイナリファイルからデータを取得して実行
        "bin" => todo!(),
        _ => unreachable!()
    }
}