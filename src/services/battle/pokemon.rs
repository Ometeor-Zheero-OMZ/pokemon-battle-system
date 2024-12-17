use std::{cmp::Ordering, thread, time::Duration};

use crate::{
    cli::print::print_letter_with_delay,
    logic::rand::XorShift128,
    models::pokemon::{BufToStatus, Pokemon, Skill, SkillType, StatusType, Target}
};

pub trait PokemonActions {
    fn attack(&mut self, skill_idx: usize, target: &mut Pokemon);
    fn compute_physical_damage(&self, skill: &Skill, attacker: &Pokemon, target: &Pokemon) -> u8;
    fn compute_special_damage(&self, skill: &Skill, attacker: &Pokemon, target: &Pokemon) -> u8;
    fn compute_status_buff(&self, status: &BufToStatus) -> u8;
    fn is_hit(&self, accuracy: u8) -> bool;
    fn apply_status_change(
        &self,
        target: &mut Pokemon,
        status_type: StatusType,
        effect_value: i8,
        status_change_more: &str,
        status_change: &str
    );
    fn get_battle_order(self_pokemon: Pokemon, enemy_pokemon: Pokemon) -> Vec<Pokemon>;
}

impl PokemonActions for Pokemon {
    /// 攻撃
    /// 
    /// # 引数
    /// * `skill_idx` - 技リストのインデックス
    /// * `target`    - 攻撃するターゲット
    fn attack(&mut self, skill_idx: usize, target: &mut Pokemon) {
        let skill = match self.skills.get(skill_idx) {
            Some(skill) => *skill,
            None => {
                eprintln!("Error: Invalid skill index {}", skill_idx);
                return;
            }
        };

        // テキスト演出
        print_letter_with_delay(&format!("{}の{}！", self.name, skill.name));
        thread::sleep(Duration::from_millis(1000));

        // 命中確率を計算
        if !self.is_hit(skill.accuracy) {
            print_letter_with_delay(&format!("{}には\n当たらなかった！", target.name));
            return;
        }

        // 技の種類を判定
        match skill.type_ {
            SkillType::PhysicalAttack => {
                let damage = self.compute_physical_damage(&skill, self, target);
                target.status.current_hp = target.status.current_hp.saturating_sub(damage);
            }
            SkillType::SpecialAttack => {
                let damage = self.compute_special_damage(&skill, self, target);
                target.status.current_hp = target.status.current_hp.saturating_sub(damage);
            }
            SkillType::ChangeStatus => {
                if let Some(skill_effect) = skill.skill_effect {
                    match skill_effect.target {
                        Target::Self_ => {
                            let status_change = if skill_effect.status_effect.effect_value > 0 {
                                "あがった"
                            } else {
                                "さがった"
                            };
                            let status_change_more =
                                if skill_effect.status_effect.effect_value.abs() == 2 {
                                    "ぐーんと"
                                } else {
                                    ""
                                };
                            self.apply_status_change(
                                target,
                                skill_effect.status_effect.target,
                                skill_effect.status_effect.effect_value,
                                status_change_more,
                                status_change
                            )
                        }
                        Target::Enemy => {
                            let status_change = if skill_effect.status_effect.effect_value > 0 {
                                "あがった"
                            } else {
                                "さがった"
                            };
    
                            let status_change_more =
                                if skill_effect.status_effect.effect_value.abs() == 2 {
                                    "ぐーんと"
                                } else {
                                    ""
                                };
                            self.apply_status_change(
                                target,
                                skill_effect.status_effect.target,
                                skill_effect.status_effect.effect_value,
                                status_change_more,
                                status_change
                            )
                        }
                        Target::Ally => {
                            // TODO
                        }
                    }
                } else {
                    eprintln!("Error: Skill effect is missing for skill: {}", skill.name);
                    return;
                }
            }
            SkillType::OneHitKO => {
                print_letter_with_delay("一撃必殺！");
                target.status.current_hp = 0;
            }
        }
        thread::sleep(Duration::from_millis(1000));
    }

    /// 物理攻撃を計算
    /// 
    /// # 引数
    /// * `skill`    - 使用する技
    /// * `attacker` - 攻撃するポケモン
    /// * `target`   - 攻撃するターゲット
    /// 
    /// # 戻り値
    /// * `dmg`      - 与えるダメージ
    fn compute_physical_damage(&self, skill: &Skill, attacker: &Pokemon, target: &Pokemon) -> u8 {
        let mut dmg = attacker.level as f32 * 2.0 / 5.0 + 2.0;
        dmg = dmg.floor();

        let atk = self.compute_status_buff(&attacker.status.atk) as f32;
        let def = self.compute_status_buff(&target.status.def) as f32;

        dmg = dmg * skill.base_atk as f32 * atk / def;
        dmg = dmg.floor();
        dmg = dmg / 50.0 + 2.0;
        dmg = dmg.floor();

        // let rand = (1 / xor_shift_rand(42)) as f32;

        dmg as u8
    }

    /// 特殊攻撃を計算
    /// 
    /// # 引数
    /// * `skill`    - 使用する技
    /// * `attacker` - 攻撃するポケモン
    /// * `target`   - 攻撃するターゲット
    /// 
    /// # 戻り値
    /// * `dmg`      - 与えるダメージ
    fn compute_special_damage(&self, skill: &Skill, attacker: &Pokemon, target: &Pokemon) -> u8 {
        let mut dmg = attacker.level as f32 * 2.0 / 5.0 + 2.0;
        dmg = dmg.floor();

        let sp_atk = self.compute_status_buff(&attacker.status.sp_atk) as f32;
        let sp_def = self.compute_status_buff(&target.status.sp_def) as f32;

        dmg = dmg * skill.base_atk as f32 * sp_atk / sp_def;
        dmg = dmg.floor();
        dmg = dmg / 50.0 + 2.0;
        dmg = dmg.floor();

        // let rand = (1 / xor_shift_rand(42)) as f32;

        dmg as u8
    }

    /// ステータスバフを計算
    /// 
    /// # 引数
    /// * `status` - 使用する技
    /// 
    /// # 戻り値
    /// * `(status.value as f64 * rate)` - バフの実数値
    fn compute_status_buff(&self, status: &BufToStatus) -> u8 {
        if status.buff < -6 {
            return status.value * 2 / 8;
        }

        if status.buff > 6 {
            return status.value * 8 / 2;
        }

        let rate = match status.buff {
            -6 => 2.0 / 8.0,
            -5 => 2.0 / 7.0,
            -4 => 2.0 / 6.0,
            -3 => 2.0 / 5.0,
            -2 => 2.0 / 4.0,
            -1 => 2.0 / 3.0,
            0 => 2.0 / 2.0,
            1 => 3.0 / 2.0,
            2 => 4.0 / 2.0,
            3 => 5.0 / 2.0,
            4 => 6.0 / 2.0,
            5 => 7.0 / 2.0,
            6 => 8.0 / 2.0,
            _ => unreachable!(),
        };

        (status.value as f64 * rate) as u8
    }

    /// 命中確率を計算
    /// 
    /// # 引数
    /// * `accuracy` - 命中率
    /// 
    /// # 戻り値
    /// * `rand_num < accuracy as u64` - 技が当たったかどうか
    fn is_hit(&self, accuracy: u8) -> bool {
        let rand_num = XorShift128::random_in_range(0, 100);
        rand_num < accuracy as u64
    }

    fn apply_status_change(
        &self,
        target: &mut Pokemon,
        status_type: StatusType,
        effect_value: i8,
        status_change_more: &str,
        status_change: &str
    ) {
        // ステータス変更
        match status_type {
            StatusType::Hp => {
                unreachable!()
            }
            StatusType::Atk => {
                target.status.atk.buff += effect_value;
            }
            StatusType::Def => {
                target.status.def.buff += effect_value;
            }
            StatusType::SpAtk => {
                target.status.sp_atk.buff += effect_value;
            }
            StatusType::SpDef => {
                target.status.sp_def.buff += effect_value;
            }
            StatusType::Spd => {
                target.status.spd.buff += effect_value;
            }
        }

        // 変更メッセージの表示
        print_letter_with_delay(&format!(
            "{}の{}が{}{}",
            target.name, 
            status_type.to_string(),
            status_change_more, 
            status_change
        ));
    }

    /// ポケモンの行動順を設定
    /// 
    /// # 引数
    /// * `self_pokemon`  - 自分のポケモン
    /// * `enemy_pokemon` - 敵のポケモン
    /// 
    /// # 戻り値
    /// * `pokemons` - 自分のポケモンと敵のポケモンのリスト
    fn get_battle_order(self_pokemon: Pokemon, enemy_pokemon: Pokemon) -> Vec<Pokemon> {
        let mut pokemons = vec![self_pokemon, enemy_pokemon];
    
        pokemons.sort_by(|a, b| {
            // すばやさでソート、同じなら技の優先度で比較
            let spd_comparison = b.status.spd.value.cmp(&a.status.spd.value);
            if spd_comparison == Ordering::Equal {
                // // すばやさが同じなら技の優先度を比較
                // b.skills.iter().map(|s| s.priority).max().cmp(
                //     &a.skills.iter().map(|s| s.priority).max(),
                // )

                let max_priority_a = a.skills.iter().map(|s| s.priority).max().unwrap_or(0);
                let max_priority_b = b.skills.iter().map(|s| s.priority).max().unwrap_or(0);
                
                max_priority_b.cmp(&max_priority_a)
            } else {
                spd_comparison
            }
        });
    
        pokemons
    }
}

/// StatusTypeの列挙子に応じてステータス名を返す
impl ToString for StatusType {
    fn to_string(&self) -> String {
        match self {
            StatusType::Hp => "HP".to_string(),
            StatusType::Atk => "こうげき".to_string(),
            StatusType::Def => "ぼうぎょ".to_string(),
            StatusType::SpAtk => "とくこう".to_string(),
            StatusType::SpDef => "とくぼう".to_string(),
            StatusType::Spd => "すばやさ".to_string(),
        }
    }
}
