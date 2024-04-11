use level_interface::*;

#[derive(Debug, Clone)]
struct MyStruct {
    data: String,
    next: Option<Box<MyStruct>>,
}
impl Pointerable for MyStruct {}

impl MyStruct {
    pub fn new() -> Self {
        Self {
            data: "新的".to_string(),
            next: None,
        }
    }
    pub fn append(&mut self, name: &str) {
        self.next = Some(Box::new(self.clone()));
        self.data += name;
    }
}

#[no_mangle]
pub static mut LEVEL_INFO: LevelInfo = LevelInfo {
    id: 1,
    name: "Test level 1",
    group: "Test",
};

#[no_mangle]
pub fn init() {}

#[no_mangle]
pub fn new() -> Pointered {
    let s = MyStruct::new().as_static();
    s.get_pointer()
}

#[no_mangle]
pub fn destory(_p: Pointered) {}

#[no_mangle]
pub fn append(p: Pointered, name: &str) {
    if let Some(p) = MyStruct::from_pointer(p) {
        p.append(name);
    }
}

#[no_mangle]
pub fn show(p: Pointered) {
    println!("{:#?}", MyStruct::from_pointer(p).unwrap());
}

// -------------------------
//
#[cfg(test)]
mod tests {
    // use super::*;

    use level_interface::Pointerable;

    use crate::{append, show, MyStruct};

    #[test]
    fn it_works() {
        let s = MyStruct::new();
        let p = s.get_pointer();
        show(p);
        append(p, "123");
        show(p);
        append(p, "456");
        show(p);
        append(p, "789");
        // show(p);
        let s = MyStruct::from_pointer(p);
        println!("{:#?}", s);
    }
}
