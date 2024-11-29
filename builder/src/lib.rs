mod front_matter;

const DUMMY: &str = r#"---
title: Hello World
---
This is John.
"#;


// ターゲットのファイルパスを受け取り、そのファイルを読み込んで Markdown の構造を返す。
pub struct Builder;

impl Builder {
    pub fn new() -> Builder {
        Self 
    }

    pub fn run(&self) -> String {
        // 1. ファイルの読み込み

        // 2. 全ファイルのFrontmatter--Titleを取得する
        let parsed = front_matter::parse::<Headers>(DUMMY).unwrap();
        println!("{:?}", parsed.headers.title);
        println!("{:?}", parsed.body);

        // 
        //
        // 3. TitleをValidationする(同名は許可しない)
        //
        // 3. AutolinkをParseする
        //
        // 4. Autolinkの部分をタイトルと差し替える。
        //
        // 5. Markdownを返す。
        "Hello, World!".to_string()
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Headers {
    title: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let builder = Builder::new();
        let result = builder.run();
        assert_eq!(result, "Hello, World!".to_string());
    }
}
