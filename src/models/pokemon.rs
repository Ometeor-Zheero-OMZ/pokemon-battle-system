use serde::{Deserialize, Serialize};

/// ポケモンを表す構造体
///
/// 各ポケモンは名前、レベル、ステータス、使用できる技で構成
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Pokemon {
    pub id: &'static str,
    pub name: String,
    pub level: u8,
    pub element: Vec<ElementType>,
    pub status: Status,
    pub skills: Vec<Skill>,
}

/// ポケモンを表す構造体
///
/// 各ポケモンは名前、レベル、ステータス、使用できる技で構成
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PokemonJson {
    pub name: String,
    pub level: u8,
    pub element: Vec<String>,
    pub status: Status,
    pub skills: Vec<String>,
}

/// ポケモンのステータスを表す構造体
///
/// HP、こうげき、ぼうぎょ、とくこう、とくぼう、すばやさで構成
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Status {
    pub hp: BufToStatus,
    pub atk: BufToStatus,
    pub def: BufToStatus,
    pub sp_atk: BufToStatus,
    pub sp_def: BufToStatus,
    pub spd: BufToStatus,
    pub current_hp: u8,
}

/// 各ステータスのバフ/デバフを表す構造体
///
/// 基本値、一時的な変更値で構成
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BufToStatus {
    pub value: u8,
    pub buff: i8,
}

/// ポケモンが使用できる技を表す構造体
///
/// 技名、威力、命中率、技の追加効果、技の種類で構成
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub element: ElementType,
    pub base_atk: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub skill_effect: Option<SkillEffect>,
    pub class: SkillType,
}

/// 技の種類を表す列挙型
///
/// 物理攻撃、特殊攻撃、ステータス変化で構成
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum SkillType {
    PhysicalAttack,
    SpecialAttack,
    ChangeStatus,
    OneHitKO,
}

/// 技の効果を表す構造体
///
/// ステータスへの影響の詳細、効果の対象で構成
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct SkillEffect {
    pub status_effect: StatusEffect,
    pub target: Target,
}

/// 技の効果が適用される対象を表す列挙型
///
/// 自分自身、味方、敵で構成
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum Target {
    Self_,
    Ally,
    Enemy,
}

/// ステータスの具体的な変更内容を表す構造体
///
/// 対象のステータスとその変化量で構成
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct StatusEffect {
    pub target: StatusType,
    pub effect_value: i8,
}

/// ポケモンのステータスを表す列挙型
///
/// ステータス効果がどのステータスを対象にするかを特定するために使用
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum StatusType {
    Hp,
    Atk,
    Def,
    SpAtk,
    SpDef,
    Spd,
    Field
}

/// ポケモンのタイプを表す列挙型
/// 
/// 各タイプ
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ElementType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy
}

/// ポケモンのタイプを文字列型に変換
#[allow(dead_code)]
impl ElementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ElementType::Normal => "ノーマル",
            ElementType::Fire => "ほのお",
            ElementType::Water => "みず",
            ElementType::Electric => "でんき",
            ElementType::Grass => "くさ",
            ElementType::Ice => "こおり",
            ElementType::Fighting => "かくとう",
            ElementType::Poison => "どく",
            ElementType::Ground => "じめん",
            ElementType::Flying => "ひこう",
            ElementType::Psychic => "エスパー",
            ElementType::Bug => "むし",
            ElementType::Rock => "いわ",
            ElementType::Ghost => "ゴースト",
            ElementType::Dragon => "ドラゴン",
            ElementType::Dark => "あく",
            ElementType::Steel => "はがね",
            ElementType::Fairy => "フェアリー"
        }
    }
}

#[allow(dead_code)]
impl SkillType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkillType::PhysicalAttack => "物理技",
            SkillType::SpecialAttack => "特殊技",
            SkillType::ChangeStatus => "変化技",
            SkillType::OneHitKO => "一撃必殺",
        }
    }
}

#[allow(dead_code)]
impl StatusType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusType::Hp => "HP",
            StatusType::Atk => "こうげき",
            StatusType::Def => "ぼうぎょ",
            StatusType::SpAtk => "とくこう",
            StatusType::SpDef => "とくぼう",
            StatusType::Spd => "すばやさ",
            StatusType::Field => "フィールド技"
        }
    }
}