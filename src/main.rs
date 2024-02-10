use serde::Serialize;
use std::fs;
use std::net::Ipv4Addr;
use std::sync::RwLock as Rw;
use tera::{Context, Tera};
#[macro_use]
extern crate rocket;

#[derive(Serialize, PartialEq, Clone)]
struct Entry {
    path: String,
    isdir: bool,
}

struct PageContents {
    root: String,
    file: Vec<Entry>,
}

use rocket::response::content::RawHtml;
use rocket::{Config, State};

const ROOT: &'static str = "/home/plof/Disco";
//const ROOT: &'static str = "/home/plof/Pictures";
#[get("/")]
fn index(tera: &State<Tera>, pg: &State<Rw<PageContents>>) -> RawHtml<String> {
    let pg = pg.read().unwrap();
    let mut ctx = Context::new();
    ctx.insert("dir", &pg.root);
    ctx.insert("files", &pg.file);
    let ret = if pg.root == ROOT { false } else { true };
    ctx.insert("ret", &ret);
    let content = tera.render("index.html", &ctx).unwrap();
    let html: RawHtml<String> = RawHtml(content);
    html
}

#[get("/<dir>")]
fn index2(dir: &str, tera: &State<Tera>, pg: &State<Rw<PageContents>>) -> RawHtml<String> {
    let mut state = pg.write().unwrap();
    if state.file.contains(&Entry {
        path: dir.to_owned(),
        isdir: true,
    }) {
        state.root = format!("{}/{}", &state.root, dir);
        state.file = get_vec_file(&state.root);
    }
    let pg = state;
    let mut ctx = Context::new();
    ctx.insert("dir", &pg.root);
    ctx.insert("files", &pg.file);
    let ret = if pg.root == ROOT { false } else { true };
    ctx.insert("ret", &ret);
    let content = tera.render("index.html", &ctx).unwrap();
    let html: RawHtml<String> = RawHtml(content);
    html
}

#[get("/file/<f>")]
fn get_file(f: &str, pg: &State<Rw<PageContents>>) -> Vec<u8> {
    let f_path = format!("{}/{}", pg.read().unwrap().root, f);
    println!("full path: {f_path}");
    let y = fs::read(&f_path);
    y.unwrap().into()
}

#[launch]
fn rocket() -> _ {
    let mut tera = Tera::default();
    tera.add_raw_template("index.html", include_str!("../templates/index.html"))
        .unwrap();

    let v = get_vec_file(ROOT);
    let pg = PageContents {
        root: ROOT.to_owned(),
        file: v,
    };

    let mut cfgi = Config::default();
    cfgi.address = Ipv4Addr::new(0, 0, 0, 0).into();
    rocket::build()
        .configure(cfgi)
        .manage(tera)
        .manage(Rw::new(pg))
        .mount("/", routes![index, get_file, index2])
}

fn get_vec_file(root: &str) -> Vec<Entry> {
    if root != ROOT {}
    let paths = fs::read_dir(root).unwrap();
    let mut v = Vec::new();
    for path in paths {
        let path = path.unwrap();
        let y = path.file_name().into_string().unwrap();
        if y.ends_with("png") || y.ends_with("jpg") || y.ends_with("jpeg") {
            v.push(Entry {
                path: y,
                isdir: false,
            });
        } else if path.path().is_dir() {
            v.push(Entry {
                path: y,
                isdir: true,
            });
        }
    }
    v
}
