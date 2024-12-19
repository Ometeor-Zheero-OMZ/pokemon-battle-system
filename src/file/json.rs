use std::collections::HashMap;

use serde::Deserialize;

/// JSONファイルを読み込む関数
/// 取得したJSON全文を返す
/// 
/// 型キャストして使用すること
/// read_json::<TrainerJson>("./json/trainer_data.json")
pub fn read_json<T>(file_path: &str) -> Result<HashMap<String, T>, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>
{
    let file = std::fs::File::open(file_path).expect("ファイルが開けませんでした");
    let json_data: HashMap<String, T> = serde_json::from_reader(file)?;

    Ok(json_data)
}