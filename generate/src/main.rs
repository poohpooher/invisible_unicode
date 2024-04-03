mod generator;

use std::path::Path;
use reqwest::Error;
use crate::generator::Generator;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = "https://invisible-characters.com/".to_string();
    let Ok(response) = reqwest::get(&request_url).await else {
        panic!("Error getting response");
    };


    let Ok(html) = response.text().await else {
        panic!("Error getting response");
    };

    // response to string
    let Ok(re) = regex::Regex::new(r"U\+([0-9A-Fa-f]{4,5})") else {
        panic!("Error creating regex");
    };
    let mut set = std::collections::HashSet::new();
    re.captures_iter(&html).for_each(|cap| {
        set.insert(cap.get(1).unwrap().as_str());
    });


    let mut invisibilities = vec![];
    for &s in set.iter() {
        let unicode = u32::from_str_radix(s, 16).expect("from_str_radix fail");
        let c = char::from_u32(unicode).expect("from_u32 fail");
        invisibilities.push(c);
    }

    // let invisibilities = set.iter().map(|&s| format!("\\u{}", s)).collect_vec();

    let mut gen = Generator::new();
    gen.add_invisibilities(invisibilities);

    let file_path = Path::new(r#"invisible_unicode/src/invisibilities.rs"#);

    if let Err(e) = gen.generate(file_path) {
        panic!("generate failed, {}", e);
    }

    Ok(())
}
