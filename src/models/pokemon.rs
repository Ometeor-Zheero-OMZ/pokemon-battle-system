/// ポケモンを表す構造体
///
/// 各ポケモンは名前、レベル、ステータス、使用できる技で構成
#[derive(Clone, PartialEq)]
pub struct Pokemon {
    pub name: &'static str,
    pub level: u8,
    pub type_: Vec<PokemonType>,
    pub status: Status,
    pub skills: Vec<Skill>,
}

/// ポケモンのステータスを表す構造体
///
/// HP、こうげき、ぼうぎょ、とくこう、とくぼう、すばやさで構成
#[derive(Clone, PartialEq)]
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
#[derive(Clone, PartialEq)]
pub struct BufToStatus {
    pub value: u8,
    pub buff: i8,
}

/// ポケモンが使用できる技を表す構造体
///
/// 技名、威力、命中率、技の追加効果、技の種類で構成
#[derive(Clone, Copy, PartialEq)]
pub struct Skill {
    pub name: &'static str,
    pub base_atk: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub skill_effect: Option<SkillEffect>,
    pub type_: SkillType,
}

/// 技の種類を表す列挙型
///
/// 物理攻撃、特殊攻撃、ステータス変化で構成
#[derive(Clone, Copy, PartialEq)]
pub enum SkillType {
    PhysicalAttack,
    SpecialAttack,
    ChangeStatus,
    OneHitKO,
}

/// 技の効果を表す構造体
///
/// ステータスへの影響の詳細、効果の対象で構成
#[derive(Clone, Copy, PartialEq)]
pub struct SkillEffect {
    pub status_effect: StatusEffect,
    pub target: Target,
}

/// 技の効果が適用される対象を表す列挙型
///
/// 自分自身、味方、敵で構成
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum Target {
    Self_,
    Ally,
    Enemy,
}

/// ステータスの具体的な変更内容を表す構造体
///
/// 対象のステータスとその変化量で構成
#[derive(Clone, Copy, PartialEq)]
pub struct StatusEffect {
    pub target: StatusType,
    pub effect_value: i8,
}

/// ポケモンのステータスを表す列挙型
///
/// ステータス効果がどのステータスを対象にするかを特定するために使用
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum StatusType {
    Hp,
    Atk,
    Def,
    SpAtk,
    SpDef,
    Spd,
}

/// ポケモンのタイプを表す列挙型
/// 
/// 各タイプ
#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum PokemonType {
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
impl PokemonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PokemonType::Normal => "ノーマル",
            PokemonType::Fire => "ほのお",
            PokemonType::Water => "みず",
            PokemonType::Electric => "でんき",
            PokemonType::Grass => "くさ",
            PokemonType::Ice => "こおり",
            PokemonType::Fighting => "かくとう",
            PokemonType::Poison => "どく",
            PokemonType::Ground => "じめん",
            PokemonType::Flying => "ひこう",
            PokemonType::Psychic => "エスパー",
            PokemonType::Bug => "むし",
            PokemonType::Rock => "いわ",
            PokemonType::Ghost => "ゴースト",
            PokemonType::Dragon => "ドラゴン",
            PokemonType::Dark => "あく",
            PokemonType::Steel => "はがね",
            PokemonType::Fairy => "フェアリー"
        }
    }
}