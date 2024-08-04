#[derive(Debug, serde::Serialize)]
pub struct QueryData {
    main: String,
    sub: Vec<String>,
}

#[macro_use]
mod macros;

mod google;
mod json;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

fn get_now_time() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d_%H-%M-%S").to_string()
}

use rand::Rng;
use std::sync::Arc;
use tokio::task;

async fn process_in_chunks(num_str: Vec<String>, path: Arc<str>, chunk_size: usize) {
    let num_str_with_index: Vec<(usize, String)> = num_str.into_iter().enumerate().collect();
    for chunk in num_str_with_index.chunks(chunk_size) {
        let mut handles = Vec::new();

        for (index, num) in chunk {
            let num = num.clone(); // この部分のcloneはやむを得ない
            let index = *index; // インデックスをコピー
            let path = Arc::clone(&path);

            let handle = task::spawn(async move {
                google::download_html(&num, &format!("{}/{}.html", &path, index))
                    .await
                    .unwrap();
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let path = "./sample.json";
    let data_list = json::read_and_print_json(path)?;
    println!("{:#?}", data_list);

    // Create a directory
    let dir = format!("./db/{}", get_now_time());
    println!("{}: Finished", get_now_time());
    std::fs::create_dir_all(&dir)?;

    for data in &data_list {
        println!("{}", data.main);
        std::fs::create_dir_all(format!("{}/{}", &dir, data.main))?;
        for sub in &data.sub {
            let wait_time = rand::thread_rng().gen_range(40..80);
            println!("Wait:{}", wait_time);
            tokio::time::sleep(tokio::time::Duration::from_secs(wait_time)).await;

            println!("{}", sub);
            std::fs::create_dir_all(format!("{}/{}/{}", &dir, data.main, sub))?;
            let word = google::get_query(sub).await?;
            let chunk_size = 100;
            // `Vec<String>`を渡して所有権の問題を回避します。
            process_in_chunks(
                word,
                format!("{}/{}/{}", &dir, data.main, sub).into(),
                chunk_size,
            )
            .await;
        }
    }

    // Serialize data_list to JSON
    let json_str = serde_json::to_string_pretty(&data_list)?;
    // Write JSON to output file
    std::fs::write(format!("{}/log.json", &dir), json_str)?;
    Ok(())
}
