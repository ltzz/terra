extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
// use terra::http::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub instance_settings: Vec<InstanceSetting>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceSetting {
    pub host_name: String,
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String
}

pub fn save() -> Result<(), Box<dyn std::error::Error>> {
    let setting = Setting {
        instance_settings: vec![
            InstanceSetting{
                host_name: "xxx".into(), 
                client_id: "xxx".into(), 
                client_secret: "xxx".into(), 
                access_token: "xxx".into()
            }.into()
            ],
    };

    let mut file = File::create("Setting.toml")?;
    let toml = toml::to_string(&setting).unwrap();
    write!(file, "{}", toml)?;
    file.flush()?;
    Ok(())
}


pub fn initialize() -> Result<Setting, Box<dyn std::error::Error>> {

    let str: String = fs::read_to_string("Setting.toml")?;
    let setting: Result<Setting, toml::de::Error> = toml::from_str(&str);
    // match setting {
    //     Ok(p) => println!("Settings loaded: \n{:#?}", p),
    //     Err(e) => panic!("Failed to parse TOML: {}", e),
    // }

    // 設定ない場合
    // クライアント登録
    //dbg!(connection::register_client("hostname"));

    // TODO: 実装
    // 認証
    // use webbrowser;

    // if webbrowser::open("http://github.com").is_ok() {
    //     // ...
    // }



    match setting {
        Ok(p) => Ok(p),
        Err(e) => Err(Box::new(e)),
    }
}
