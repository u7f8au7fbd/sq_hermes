use rand::Rng;
use reqwest::{header::{HeaderMap, HeaderValue, USER_AGENT}, Client};
use scraper::{Html, Selector};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, time};

pub const USER_AGENTS_INDEX: [&str; 10] = [
    // Windows用のChromeブラウザ
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    // Windows用のFirefoxブラウザ
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0",
    // Windows用のEdgeブラウザ
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59",
    // Macintosh用のSafariブラウザ
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15",
    // Macintosh用のChromeブラウザ
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    // Ubuntu用のFirefoxブラウザ
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0",
    // Ubuntu用のChromeブラウザ
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    // iPhone用のSafariブラウザ
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
    // iPhone用のChromeブラウザ
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Mobile Safari/537.36",
    // Android用のChromeブラウザ
    "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Mobile Safari/537.36"
];

pub async fn get_query(word: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut attempt = 0;
    let max_attempts = 10;

    while attempt < max_attempts {
        attempt += 1;
        let url:&str = &format!("https://www.google.co.jp/search?q={}&oq={}&hl=ja&lr=lang_ja&pws=0&sourceid=chrome&ie=UTF-8&num=100", word, word);
        let mut urls: Vec<String> = Vec::new();

        // ランダムなユーザーエージェントを選択
        let user_agent =
            USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
        headers.insert("Cookie", HeaderValue::from_str("")?);

        // HTTPクライアントの作成
        let client = Client::builder().cookie_store(true).build()?;
        let response = client.get(url).headers(headers).send().await?;
        let content = response.text().await?;

        // HTMLパース
        let document = Html::parse_document(&content);
        let selector = Selector::parse(r#"a[jsname="UWckNb"]"#).unwrap();

        // href属性の抽出と出力
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                urls.push(href.to_string());
            }
        }

        println!("Attempt {}: URLs count: {}", attempt, urls.len());

        if !urls.is_empty() {
            return Ok(urls);
        }

        // 再試行する前に少し待つ
        let wait_time = rand::thread_rng().gen_range(40..80);
        println!("Wait:{}", wait_time);
        tokio::time::sleep(time::Duration::from_secs(wait_time)).await;
    }

    Err("データの取得に失敗".into())
}

pub async fn download_html(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ユーザーエージェントの配列からランダムに1つを選ぶ
    let user_agent = USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
    let mut headers = HeaderMap::new();

    // ヘッダーにユーザーエージェントを追加
    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
    // ヘッダーに空のクッキーを追加
    headers.insert("Cookie", HeaderValue::from_str("")?);

    // クッキーをサポートするクライアントをビルド
    let client = Client::builder().cookie_store(true).build()?;

    // エラーハンドリングのためにマッチ式を使用
    match client.get(url).headers(headers).send().await {
        Ok(response) => {
            // 指定されたファイルパスにファイルを作成または開く
            let mut file = OpenOptions::new()
                .create(true) // ファイルが存在しない場合は作成
                .write(true) // 書き込みモードで開く
                .truncate(true) // ファイルの内容を消去
                .open(file_path)
                .await?;

            // レスポンスの内容をテキストとして取得
            let content = response.text().await?;
            // ファイルに書き込む
            file.write_all(content.as_bytes()).await?;
        }
        Err(e) => {
            // エラーが発生した場合はエラーメッセージとURLを表示
            println!("Failed to download {}: {}", url, e);
        }
    }

    Ok(())
}