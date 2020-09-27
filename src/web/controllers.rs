use rocket_contrib::json::JsonValue;
use std::string::String;

use sp_core::crypto::Pair;
use sp_core::hexdisplay::HexDisplay;
use passwords::PasswordGenerator;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!, rust web"
}

#[post("/address/new")]
pub fn new_address() -> JsonValue {
    
    // 生成随机密码
    let pg = PasswordGenerator {
        length: 8,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false, // 特殊字符
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    let pwd = pg.generate_one().unwrap();

    // 生成地址及助记词
    // let (pair, phrase, _) = Pair::generate_with_phrase(Some(pwd.as_str()));

    // json!({
    //     "code": 0,
    //     "mnemonic": phrase,
    //     "password": pwd,
    //     "public": format!("0x{}", HexDisplay::from(&pair.public().clone().as_ref())),
    //     "dotAddress": "",
    //     "ksmAddress": "",
    //     "edgAddress": "",
    //     "cringAddress": "",
    //     "plmAddress": "",
    //     "klpAddress": "",
    //     "fisAddress": "",
    // })
    json!({
        "code": 0,
        "data": {
            "password": pwd,
        },
        "msg": "",
    })
}

// 公钥转dot地址
// fn pubkey_to_dot_address(pair: Pair) -> String {
//     return "".to_string();
// }