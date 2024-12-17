use std::io::stdout;
use std::io::Write;
use std::{thread, time::Duration};

use crate::models::pokemon::Pokemon;

/// 現在のHPバーを出力
/// 
/// # 引数
/// * `self_pokemon` - 自分のポケモン
/// * `enemy_poke`   - 敵のポケモン
#[rustfmt::skip]
pub fn print_current_battle_status(self_pokemon: &Pokemon, enemy_poke: &Pokemon) {
  println!("　{}:L{}", enemy_poke.name, enemy_poke.level);
  println!("|　HP: {}", get_hp_bar(enemy_poke.status.current_hp, enemy_poke.status.hp.value, 15));
  println!("------------------------▶");

  println!("");
  println!("");
  
  println!("　　　　　　　　{}:L{}", self_pokemon.name, self_pokemon.level);
  println!("　　　　　　　　HP: {}", get_hp_bar(self_pokemon.status.current_hp, self_pokemon.status.hp.value, 15));
  println!("　　　　　　　　　　　{}/ 　{}　　　　|", self_pokemon.status.current_hp, self_pokemon.status.hp.value);
  println!("　　　　　　　◀------------------------");

  println!("");
  println!("=============================================");
}

/// HP値を取得
/// 
/// # 引数
/// * `hp`          - 現在のHP
/// * `max_hp`      - 最大HP
/// * `max_bar_len` - 最大HPバー
fn get_hp_bar(hp: u8, max_hp: u8, max_bar_len: u8) -> String {
    let mut bar = String::new();
    let bar_len = (hp as f32 / max_hp as f32 * max_bar_len as f32) as u8;
    for _ in 0..bar_len {
        bar.push('=');
    }
    for _ in bar_len..max_bar_len {
        bar.push('-');
    }
    bar
}

/// 技一覧を出力
/// 
/// # 引数
/// * `self_pokemon` - 自分のポケモン
pub fn print_skill_list(self_pokemon: &Pokemon) {
    let pd = "　　　　　　　　　　　　";
    for (i, skill) in self_pokemon.skills.iter().enumerate() {
        println!("{}||　{}.{}", pd, i + 1, skill.name);
    }
    println!("{}========================", pd)
}

/// テキスト表示アニメーションを扱う関数
///
/// # 引数
/// * `text` - テキスト表示する全文
pub fn print_letter_with_delay(text: &str) {
    for c in text.chars() {
        print!("{}", c);
        let _ = stdout().flush();
        thread::sleep(Duration::from_millis(40));
    }
    println!("");
}

/// 画面をクリア
pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/// 画面をクリアし現在のHPバーを出力
/// 
/// # 引数
/// * `self_pokemon` - 自分のポケモン
/// * `enemy_poke`   - 敵のポケモン
pub fn clear_and_print_current_battle_status(self_pokemon: &Pokemon, enemy_poke: &Pokemon) {
    clear_screen();
    print_current_battle_status(self_pokemon, enemy_poke);
}
