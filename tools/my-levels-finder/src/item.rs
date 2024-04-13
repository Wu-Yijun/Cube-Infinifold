use std::{collections::HashMap, fs};

use json::JsonValue;

use crate::{get_json, js_obj_arr, js_obj_num, js_obj_str};

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
#[derive(Debug, Clone)]
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
    fn is_valid(&self) -> bool {
        if let Ok(md) = fs::metadata(&self.0) {
            md.is_file()
        } else {
            false
        }
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
    pub fn collect(&self, collections: &mut CollectedGame) {
        let group_index = self.index;
        collections.new_group(group_index, &self.name);
        for l in &self.levels {
            collections.new_level(group_index, l.index, &l.name, l.filename.with(""));
        }
        for g in &self.groups {
            g.collect(collections);
        }
    }
}

impl Game {
    pub fn find(&mut self) {
        self.0.find();
    }
    pub fn collect(&self) -> CollectedGame {
        let mut collection = CollectedGame::new(&self.0.name);
        self.0.collect(&mut collection);
        collection.clean();
        collection
    }
}

#[derive(Debug, Clone)]
pub struct CollectedLevel {
    pub name: String,
    pub link: Link,
}
#[derive(Debug, Clone)]
pub struct CollectedGroup {
    pub name: String,
    pub levels: HashMap<i64, CollectedLevel>,
}
#[derive(Debug, Clone)]
pub struct CollectedGame {
    pub name: String,
    pub groups: HashMap<i64, CollectedGroup>,
}
impl PartialEq for CollectedGame {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl CollectedLevel {
    fn new(name: &String, link: Link) -> Self {
        Self {
            name: name.clone(),
            link,
        }
    }
}
impl CollectedGroup {
    fn new(name: &String) -> Self {
        Self {
            name: name.clone(),
            levels: HashMap::new(),
        }
    }
}
impl CollectedGame {
    fn new(name: &String) -> Self {
        Self {
            name: name.clone(),
            groups: HashMap::new(),
        }
    }
    fn new_group(&mut self, group_index: i64, name: &String) {
        if let Some(g) = self.groups.get_mut(&group_index) {
            g.name = name.clone();
        } else {
            self.groups.insert(group_index, CollectedGroup::new(name));
        }
    }
    fn new_level(&mut self, group_index: i64, index: i64, name: &String, link: Link) {
        if let Some(g) = self.groups.get_mut(&group_index) {
            g.levels.insert(index, CollectedLevel::new(name, link));
        } else {
            self.groups.insert(group_index, CollectedGroup::new(name));
        }
    }
    /// Notice it is **clean** but not ~~clear~~
    ///
    /// This is called at the end of the collect of game,
    /// and groups with no level will be deleted,
    /// and levels with no valid link will be deleted
    fn clean(&mut self) {
        self.groups.retain(|_, group| {
            group.levels.retain(|_, level| level.link.is_valid());
            !group.levels.is_empty()
        });
    }
}
