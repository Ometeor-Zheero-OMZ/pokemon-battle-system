use std::{thread, time::Duration};

use crate::{logic::rand::XorShift128, models::pokemon::Pokemon, services::battle::pokemon::PokemonActions};
use super::print::{clear_screen, clear_and_print_current_battle_status, print_letter_with_delay, print_skill_list};

/// バトル開始
/// 
/// # 引数
/// * `self_` - 自分のポケモン
/// * `enemy` - 敵のポケモン
pub fn start_battle(self_: &mut Pokemon, enemy: &mut Pokemon) {
    // 画面クリア
    clear_screen();

    // テキストアニメーションを描画
    print_letter_with_delay("あ!　やせいの");
    print_letter_with_delay(&format!("{}が　とびだしてきた！", enemy.name));

    // 演出上の遅延
    thread::sleep(Duration::from_millis(2000));

    // 現在のHPバーを描画
    clear_and_print_current_battle_status(&self_, &enemy);

    while self_.status.current_hp > 0 && enemy.status.current_hp > 0 {
        // 技リストの描画
        thread::sleep(Duration::from_millis(1000));
        print_skill_list(&self_);

        // 自分のターン
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let skill_idx = match input.trim() {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            _ => continue,
        };

        // 攻撃
        clear_and_print_current_battle_status(&self_, &enemy);
        self_.attack(skill_idx, enemy);
        clear_and_print_current_battle_status(self_, enemy);

        // 相手のターン
        let skill_idx = XorShift128::random_in_range(0, 4) as usize;

        if enemy.status.current_hp > 0 {
            enemy.attack(skill_idx, self_);
            clear_and_print_current_battle_status(self_, enemy);
        }
    }

    if self_.status.current_hp > 0 {
        print_letter_with_delay(&format!("てきの　{}　はたおれた！", enemy.name));
        thread::sleep(Duration::from_millis(2000));
        clear_and_print_current_battle_status(&self_, &enemy);
        print_letter_with_delay(&format!("{}との　しょうぶに かった!", enemy.name));
    } else {
        print_letter_with_delay(&format!("{}は　たおれた！", self_.name));
        thread::sleep(Duration::from_millis(2000));
        clear_and_print_current_battle_status(&self_, &enemy);
        print_letter_with_delay(&format!("めのまえが　まっくらに　なった"));
    }
}

// TODO ポケモンのすばやさ技の優先度を比較して行動順を決定
// pub fn battle_round(self_: &mut Pokemon, enemy: &mut Pokemon) {
//     // 並び順の取得
//     let order = Pokemon::get_battle_order(self_.clone(), enemy.clone());

//     // ポケモンのターンを順番に処理
//     for pokemon in &order {
//         // 自分のターン
//         if pokemon == self_ {
//             let mut input = String::new();
//             std::io::stdin()
//                 .read_line(&mut input)
//                 .expect("Failed to read line");

//             let skill_idx = match input.trim() {
//                 "1" => 0,
//                 "2" => 1,
//                 "3" => 2,
//                 "4" => 3,
//                 "5" => 4,
//                 _ => continue,
//             };

//             // 攻撃を行う
//             clear_and_print_current_battle_status(&self_, &enemy);
//             self_.attack(skill_idx, enemy);
//             clear_and_print_current_battle_status(self_, enemy);
//         } else if pokemon == enemy {
//             // 相手のターン
//             let skill_idx = XorShift128::random_in_range(0, 4) as usize;

//             if enemy.status.current_hp > 0 {
//                 enemy.attack(skill_idx, self_);
//                 clear_and_print_current_battle_status(self_, enemy);
//             }
//         }
//     }
// }