use serde_json::Value;

use crate::QueryData;

pub fn read_and_print_json(path: &str) -> Result<Vec<QueryData>, Box<dyn std::error::Error>> {
    let mut data_list: Vec<QueryData> = Vec::new();

    let json_str = std::fs::read_to_string(path)?;

    let json: Value = serde_json::from_str(&json_str)?;

    if let Value::Array(arr) = json {
        for obj in arr {
            if let Value::String(main_word) = obj["main_word"].clone() {
                if let Value::Array(sub_word) = obj["sub_word"].clone() {
                    let mut sub: Vec<String> = Vec::new();

                    for word in sub_word {
                        if let Value::String(word) = word {
                            let url = format!("{}+{}", main_word, word);
                            sub.push(url);
                        }
                    }
                    let data_entry = QueryData {
                        main: main_word,
                        sub,
                    };
                    data_list.push(data_entry);
                }
            }
        }
    }
    Ok(data_list)
}