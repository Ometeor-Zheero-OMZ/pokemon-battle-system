use crate::Pokemon;

#[allow(dead_code)]
impl Pokemon {
    /// 列挙型で定義されたポケモンのタイプを文字列型に変換
    /// 
    /// # 引数
    /// * `&self` - Pokemon
    /// 
    /// # 戻り値
    /// * `Vec<&'static str>` - ポケモンのタイプを格納したベクター
    pub fn convert_to_text(&self) -> Vec<&'static str> {
        self.type_
            .iter()
            .map(|t| t.as_str())
            .collect()
    }
}