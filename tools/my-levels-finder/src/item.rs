use json::JsonValue;

use crate::{get_json, js_obj_arr, js_obj_num, js_obj_str, js_str};

static mut INDEX_ID: i64 = 0;
fn next_id() -> i64 {
    unsafe {
        INDEX_ID += 1;
        INDEX_ID
    }
}
fn with_id(id: i64) -> i64 {
    unsafe {
        if id > INDEX_ID {
            INDEX_ID = id;
        }
        INDEX_ID += 1;
        id
    }
}

#[derive(Debug)]
pub struct Level {
    filename: Link,
    name: String,
    index: i64,
}
#[derive(Debug)]
pub struct Link(String);
#[derive(Debug)]
pub struct Linker {
    path: Link,
    filename: String,
}

#[derive(Debug)]
pub struct Group {
    pub path: Link,
    pub name: String,
    pub index: i64,
    pub levels: Vec<Level>,
    pub groups: Vec<Group>,
    pub linkers: Vec<Linker>,
}

#[derive(Debug)]
pub struct Game(Group);

pub trait FromJson {
    fn get(path: &Link, j: &JsonValue) -> Self;
}

impl FromJson for Game {
    fn get(path: &Link, j: &JsonValue) -> Self {
        Game(Group::get(path, j))
    }
}
impl FromJson for Group {
    fn get(path: &Link, j: &JsonValue) -> Self {
        let dir = js_obj_str(j, "path", "");
        let path = path.with(dir);
        let name = js_obj_str(j, "name", "").to_string();
        let index = with_id(js_obj_num(j, "index", next_id()));
        let mut levels = Vec::new();
        if let Some(a) = js_obj_arr(j, "levels") {
            for j in a {
                levels.push(Level::get(&path, j));
            }
        }
        let mut groups = Vec::new();
        if let Some(a) = js_obj_arr(j, "groups") {
            for j in a {
                groups.push(Group::get(&path, j));
            }
        }
        let mut linkers = Vec::new();
        if let Some(a) = js_obj_arr(j, "linkers") {
            for j in a {
                linkers.push(Linker::get(&path, j));
            }
        }
        Self {
            path,
            name,
            index,
            levels,
            groups,
            linkers,
        }
    }
}

impl FromJson for Level {
    fn get(path: &Link, j: &JsonValue) -> Self {
        let dir = js_obj_str(j, "filename", "");
        let filename = path.with(dir);
        let name = js_obj_str(j, "name", "").to_string();
        let index = with_id(js_obj_num(j, "index", next_id()));
        Self {
            name,
            index,
            filename,
        }
    }
}
impl FromJson for Linker {
    fn get(path: &Link, j: &JsonValue) -> Self {
        let dir = js_obj_str(j, "path", "");
        let path = path.with(dir);
        let filename = js_obj_str(j, "filename", "levels.json").to_string();
        Linker { path, filename }
    }
}

impl Link {
    pub fn new(path: &str) -> Link {
        Link(path.to_string())
    }
    pub fn with(&self, dir: &str) -> Link {
        Link(self.path().clone() + dir)
    }
    pub fn current() -> Link {
        Link(String::new())
    }
    pub fn path(&self) -> &String {
        &self.0
    }
}

impl Group {
    pub fn find(&mut self) {
        for i in self.linkers.iter() {
            let j = get_json(&i.path.with(&i.filename).path());
            self.groups.push(Group::get(&i.path, &j));
        }
        for i in self.groups.iter_mut() {
            i.find();
        }
    }
}

impl Game {
    pub fn find(&mut self) {
        self.0.find();
    }
}
