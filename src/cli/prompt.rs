use std::{thread, time::Duration};

use crate::{models::trainer::Trainer, services::battle::trainer::TrainerActions};
use super::print::{clear_screen, print_letter_with_delay};

/// トレーナーを選択
/// 
/// # 引数
/// * `trainers` - トレーナーのリスト
/// 
/// # 戻り値
/// * `&mut Trainer` - 選択したトレーナー
pub fn select_trainer(trainers: &mut [Trainer]) -> &mut Trainer {
    loop {
        clear_screen();
        print_letter_with_delay("トレーナーを選んでください：");
    
        let num_trainers = trainers.len();
        
        for (idx, trainer) in trainers.iter().enumerate() {
            println!("{}: {}", idx + 1, trainer.name);
        }
        println!("=============================================");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: usize = match choice.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= num_trainers => num - 1,
            _ => {
                println!("もう一度選びなおしてください。");
                continue;
            }
        };

        if trainers[choice].pokemons.is_empty() {
            println!(
                "{} はポケモンを所持していません。\n別のトレーナーを選んでください。",
                trainers[choice].name
            );
            thread::sleep(Duration::from_millis(2000));
            continue;
        }

        return &mut trainers[choice];
    }
}

/// ポケモンを選択
/// 
/// # 引数
/// * `trainer` - トレーナー
pub fn select_pokemon(trainer: &mut Trainer) {
    loop {
        clear_screen();
        // print_letter_with_delay(&format!("選んだトレーナー: {}\n", trainer.name));
        println!("選んだトレーナー: {}\n", trainer.name);
        print_letter_with_delay("ポケモンを選んでください：\n");
        print_letter_with_delay("所持ポケモン：");
    
        for (idx, pokemon) in trainer.pokemons.iter().enumerate() {
            println!("{}: {}", idx + 1, pokemon.name);
        }
        println!("=============================================");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: usize = match choice.trim().parse() {
            Ok(num) if num > 0 && num <= trainer.pokemons.len() => num,
            _ => {
                println!("もう一度選びなおしてください。");
                thread::sleep(Duration::from_millis(2000));
                continue;
            }
        };

        trainer.set_active_pokemon(trainer.pokemons[choice.saturating_sub(1)].clone());
        return;
    }
}