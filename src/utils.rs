use crate::spider::Information;
use serde_json::{Value,json};
use std::{
    env,
    fs::{self, File},
    io::{prelude::*, Result},
    path::Path,
};
pub enum Save {
    Storage,
    Running,
    V2ray,
    Subscribes,
}
fn create_storage_before() {
    let home = env::var("HOME").unwrap();
    fs::create_dir_all(home + "/.config/tv2ray").unwrap();
}
pub fn create_json_file(save: Save, input: String) -> Result<()> {
    let home = env::var("HOME").unwrap();
    let location = match save {
        Save::Storage => format!("{}/.config/tv2ray/storage.json", home),
        Save::Running => format!("{}/.config/tv2ray/running.json", home),
        Save::V2ray => format!("{}/.config/tv2ray/v2core.json", home),
        Save::Subscribes => format!("{}/.config/tv2ray/subscribes.json", home),
    };
    let path = Path::new(location.as_str());
    let mut file = File::create(&path)?;
    //let storge: String = input;
    file.write_all(input.as_bytes())?;
    Ok(())
}
fn get_json(save: Save) -> Result<String> {
    let home = env::var("HOME").unwrap();
    let location = match save {
        Save::Storage => format!("{}/.config/tv2ray/storage.json", home),
        Save::Running => format!("{}/.config/tv2ray/running.json", home),
        Save::V2ray => format!("{}/.config/tv2ray/v2core.json", home),
        Save::Subscribes => format!("{}/.config/tv2ray/subscribes.json", home),
    };
    let mut file = File::open(location)?;
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    Ok(output)
}
pub fn start_v2core() -> String {
    create_storage_before();
    let message = match get_json(Save::V2ray) {
        Ok(output) => output,
        Err(_) => {
            let core = json!({
                "v2core":"/usr/bin/v2ray"
            }).to_string();
            create_json_file(Save::V2ray, core.clone()).unwrap_or_else(|err| panic!("{}", err));
            core
        }
    };
    let v: Value = serde_json::from_str(message.as_str()).unwrap();
    let message_pre = v["v2core"].to_string();
    crate::spider::remove_quotation(message_pre)
}
pub fn get_subs() -> Vec<String> {
    create_storage_before();
    let messages = match get_json(Save::Subscribes) {
        Ok(output) => output,
        Err(_) => {
            create_json_file(Save::Storage, "[]".to_string())
                .unwrap_or_else(|err| panic!("{}", err));
            "[]".to_string()
        }
    };
    let mut subscribes = Vec::new();
    let v: Value = serde_json::from_str(messages.as_str()).unwrap();
    let mut index = 0;
    while v[index] != Value::Null {
        let sub = v[index]["url"].to_string();
        let length = sub.len();
        let sub = (&sub[1..length - 1]).to_string();
        subscribes.push(sub);
        index += 1;
    }
    subscribes
}
pub fn start() -> Vec<Information> {
    create_storage_before();
    let messages = match get_json(Save::Storage) {
        Ok(output) => output,
        Err(_) => {
            create_json_file(Save::Storage, "[]".to_string())
                .unwrap_or_else(|err| panic!("{}", err));
            "[]".to_string()
        }
    };
    let mut informations = Vec::new();
    let v: Value = serde_json::from_str(messages.as_str()).unwrap();
    let mut index = 0;
    while v[index] != Value::Null {
        let the_url = v[index]["url"].to_string();
        let length = the_url.len();
        let instore = &the_url[1..length - 1];
        informations.push(Information {
            urls: instore.to_string(),
            func: v[index]["func"].to_string(),
            add: v[index]["add"].to_string(),
            aid: v[index]["aid"].to_string(),
            host: v[index]["host"].to_string(),
            id: v[index]["id"].to_string(),
            net: v[index]["net"].to_string(),
            path: v[index]["path"].to_string(),
            port: v[index]["port"].to_string(),
            ps: v[index]["ps"].to_string(),
            tls: v[index]["tls"].to_string(),
            typpe: v[index]["type"].to_string(),
        });
        //let names = v[index]["ps"].to_string();
        //start.add_item(remove_quotation(names), url);
        index += 1;
    }
    informations
}
