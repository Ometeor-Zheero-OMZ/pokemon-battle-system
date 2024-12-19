use crate::models::pokemon::{ElementType, Skill, SkillEffect, SkillType, StatusEffect, StatusType, Target};



/// 技の設定
/// 
/// # 引数
/// * `pokemon_name` - ポケモン名
/// 
/// # 戻り値
/// * `Vec<Skill>` - 初期化した技のリスト
pub fn set_skill_list(pokemon_name: &str) -> Vec<Skill> {
    match pokemon_name {
        "pikachu" => {
            let tailwind = Skill {
                name: "しっぽをふる".to_string(),
                element: ElementType::Normal,
                base_atk: 0,
                accuracy: 100,
                priority: 0,
                skill_effect: Some(SkillEffect {
                    status_effect: StatusEffect {
                        target: StatusType::Def,
                        effect_value: -1,
                    },
                    target: Target::Enemy,
                }),
                class: SkillType::ChangeStatus,
            };
            let thundershock = Skill {
                name: "でんきショック".to_string(),
                element: ElementType::Electric,
                base_atk: 40,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                class: SkillType::SpecialAttack,
            };
            let quick_attack = Skill {
                name: "でんこうせっか".to_string(),
                element: ElementType::Normal,
                base_atk: 40,
                accuracy: 100,
                priority: 2,
                skill_effect: None,
                class: SkillType::PhysicalAttack,
            };
            let tackle = Skill {
                name: "たいあたり".to_string(),
                element: ElementType::Normal,
                base_atk: 40,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                class: SkillType::PhysicalAttack,
            };
        
            vec![tailwind, thundershock, quick_attack, tackle]
        },
        "metagross" => {
            let bullet_punch = Skill {
                name: "バレットパンチ".to_string(),
                element: ElementType::Steel,
                base_atk: 40,
                accuracy: 100,
                priority: 2,
                skill_effect: None,
                class: SkillType::PhysicalAttack,
            };
            let earthquake = Skill {
                name: "じしん".to_string(),
                element: ElementType::Ground,
                base_atk: 100,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                class: SkillType::PhysicalAttack,
            };
            let ice_punch = Skill {
                name: "れいとうパンチ".to_string(),
                element: ElementType::Ice,
                base_atk: 75,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                class: SkillType::PhysicalAttack,
            };
            let psychic_fangs = Skill {
                name: "サイコファング".to_string(),
                element: ElementType::Psychic,
                base_atk: 85,
                accuracy: 100,
                priority: 0,
                skill_effect: None,
                class: SkillType::PhysicalAttack,
            };
        
            vec![bullet_punch, earthquake, ice_punch, psychic_fangs]
        },
        "pidgey" => {
            let thundershock = Skill {
                name: "ゴッドバード".to_string(),
                element: ElementType::Flying,
                base_atk: 120,
                accuracy: 50,
                priority: 0,
                skill_effect: None,
                class: SkillType::SpecialAttack,
            };
            let growl = Skill {
                name: "なきごえ".to_string(),
                element: ElementType::Normal,
                base_atk: 0,
                accuracy: 100,
                priority: 0,
                skill_effect: Some(SkillEffect {
                    status_effect: StatusEffect {
                        target: StatusType::Atk,
                        effect_value: -1,
                    },
                    target: Target::Enemy,
                }),
                class: SkillType::ChangeStatus,
            };
            let quick_attack = Skill {
                name: "でんこうせっか".to_string(),
                element: ElementType::Normal,
                base_atk: 40,
                accuracy: 100,
                priority: 2,
                skill_effect: None,
                class: SkillType::PhysicalAttack,
            };
            let tackle = Skill {
                name: "つのドリル".to_string(),
                element: ElementType::Normal,
                base_atk: 40,
                accuracy: 30,
                priority: 0,
                skill_effect: None,
                class: SkillType::OneHitKO,
            };
        
            vec![thundershock, growl, quick_attack, tackle]
        }
        _ => {
            eprintln!("{}の技が設定されていません。", pokemon_name);
            unreachable!()
        }
    }
}