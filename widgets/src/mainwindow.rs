use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, NamedView, ResizedView, ScrollView, SelectView};
use cursive::view::{Nameable,Scrollable,Resizable};
use cursive::Cursive;
use spider::get_the_key;
use futures::executor::block_on;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde_json::Value;
use crate::abutton::mybutton::MyButton;
//#[derive(Clone)]
//pub struct MyButton {
//    pub name : String,
//}
//impl MyButton{
//    #[allow(dead_code)]
//    pub fn new () -> MyButton{
//        MyButton{
//            name: String::from("ssss"),
//        }
//    }
//    pub fn output(&self) -> Dialog {
//        Dialog::around(TextView::new("Hello Dialog!"))
//                .title("Cursive")
//                .button(self.name.clone(), |s| s.quit())
//    }
//}
//impl Drop for MyButton {
//    fn drop(&mut self) {
//        println!("sss");
//    }
//}
fn url_select() -> ResizedView<ScrollView<NamedView<SelectView<MyButton>>>> {
    let mut start = SelectView::<MyButton>::new()
        .on_submit(on_submit);
    let path = Path::new("storage.json");
    //let display = path.display();
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(_) => {
            let path2 = Path::new("storage.json");
            let display2 = path2.display();
            let mut file2 = match File::create(&path2) {
                Err(why) => panic!("couldn't create {}: {}",
                                   display2,
                                   why.to_string()),
                Ok(file2) => file2,
            };
            let mut storge2:String = String::new();
            storge2.push_str("[]");
            // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
            match file2.write_all(storge2.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", display2,
                                                       why.to_string())
                },
                Ok(_) => {
                },
            }
            let path3 = Path::new("storage.json");
            File::open(&path3).unwrap()

        }
        Ok(file) => file,
    };
    let mut ss = String::new();
    match file.read_to_string(&mut ss){
        Err(_) => {},
        Ok(_)=>{
            let v:Value = serde_json::from_str(ss.as_str()).unwrap();
            let mut index = 0;
            while v[index]!=Value::Null{
                let the_url = v[index]["url"].to_string();
                let lenghth = the_url.len();
                let instore = &the_url[1..lenghth-1];
                let url = MyButton{
                    urls :instore.to_string(),
                    func :v[index]["func"].to_string(),
                    add  :v[index]["add"].to_string(),
                    aid  :v[index]["aid"].to_string(),
                    host :v[index]["host"].to_string(),
                    id   :v[index]["id"].to_string(),
                    net  :v[index]["net"].to_string(),
                    path :v[index]["path"].to_string(),
                    port :v[index]["path"].to_string(),
                    ps   :v[index]["ps"].to_string(),
                    tls  :v[index]["tls"].to_string(),
                    typpe:v[index]["type"].to_string()
                };
                let names = v[index]["ps"].to_string();
                start.add_item(names, url);
                index=index+1;
            }

        }
    }
    let select =start
        .with_name("select")
        .scrollable()
        .scroll_y(true)
        .fixed_size((60, 25));
    return select;

}
pub fn url_buttons() -> Dialog {
    let select = url_select();
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(Button::new("Load", onload))
        .child(DummyView)
        .child(Button::new("Quit", quit));
    let dialog = Dialog::around(LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons))
        .title("Fuck you GFW");
    return dialog;

}
fn delete_name(s: &mut Cursive) {
    let select = s.find_name::<SelectView<MyButton>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(_) => {
            s.add_layer(Dialog::around(LinearLayout::horizontal()
                    .child(Button::new("Sure", |s|{
                        let mut select = s.find_name::<SelectView<MyButton>>("select").unwrap();
                        match select.selected_id() {
                            None => {
                                s.add_layer(Dialog::info("No name to remove"));
                                s.pop_layer();
                                s.pop_layer();
                            },
                            Some(focus) =>{
                                select.remove_item(focus);
                                s.pop_layer();
                            }
                        }
                    }))
                    .child(Button::new("cancle", |s|{
                        s.pop_layer();
                    })))
                .title("Sure?"));
        }
    }
}

fn on_submit(s: &mut Cursive, name: &MyButton) {
    s.add_layer(name.output());
}
fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<MyButton>| {
            let mut temp : Vec<String> = vec![];
            temp.push(name.to_string());
            let future = get_the_key(temp);
            let output :Vec<Vec<String>> = block_on(future).unwrap();
            let mut storge : String = String::new();
            storge.push('[');
            storge.push('\n');
            for urls in output.into_iter(){
                for url in urls.into_iter(){
                    let temp2 = MyButton::new(url);
                    storge.push_str(
format!("{{
    \"func\":{},
    \"url\":\"{}\",
    \"add\":{},
    \"aid\":{},
    \"host\":{},
    \"id\":{},
    \"net\":{},
    \"path\":{},
    \"port\":{},
    \"ps\":{},
    \"tls\":{},
    \"type\":{}
}},\n",
                    temp2.clone().func,
                    temp2.clone().urls,
                    temp2.clone().add,
                    temp2.clone().aid,
                    temp2.clone().host,
                    temp2.clone().id,
                    temp2.clone().net,
                    temp2.clone().path,
                    temp2.clone().port,
                    temp2.clone().ps,
                    temp2.clone().tls,
                    temp2.clone().typpe
                    ).as_str());
                    let names = temp2.clone().ps;
                    view.add_item(names, temp2);
                }
                storge.pop();
                storge.pop();
                storge.push('\n');
                storge.push(']');
            }
            let path2 = Path::new("storage.json");
            let display2 = path2.display();
            let mut file2 = match File::create(&path2) {
                Err(why) => panic!("couldn't create {}: {}",
                                   display2,
                                   why.to_string()),
                Ok(file2) => file2,
            };

            // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
            match file2.write_all(storge.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", display2,
                                                       why.to_string())
                },
                Ok(_) => {
                },
            }


        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name")
            .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}
fn quit(s: &mut Cursive){
    Cursive::quit(s);
}
fn onload(s: &mut Cursive){
    let path = Path::new("storage.json");
    let display = path.display();
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.to_string()),
        Ok(file) => file,
    };
    let mut ss = String::new();
    match file.read_to_string(&mut ss){
        Err(why) => {
            s.add_layer(Dialog::info(why.to_string()));
        },
        Ok(_)=>{
        s.call_on_name("select", |view: &mut SelectView<MyButton>| {
            view.clear();
            let v:Value = serde_json::from_str(ss.as_str()).unwrap();
            let mut index = 0;
            while v[index]!=Value::Null{
                let url = MyButton{
                    urls :v[index]["url"].to_string(),
                    func :v[index]["func"].to_string(),
                    add  :v[index]["add"].to_string(),
                    aid  :v[index]["aid"].to_string(),
                    host :v[index]["host"].to_string(),
                    id   :v[index]["id"].to_string(),
                    net  :v[index]["net"].to_string(),
                    path :v[index]["path"].to_string(),
                    port :v[index]["path"].to_string(),
                    ps   :v[index]["ps"].to_string(),
                    tls  :v[index]["tls"].to_string(),
                    typpe:v[index]["type"].to_string()
                };
                let names = v[index]["ps"].to_string();
                view.add_item(names, url);
                index=index+1;
            }
        });
        }
    }
}
