use levels_interface;

pub struct Level {
    faces: Vec<my_items::Face>,
    mif: levels_interface::MyInterface,
    p: levels_interface::Pointered,
}

impl Level {
    fn none() -> Self {
        Self {
            faces: vec![],
            mif: levels_interface::my_interface::MyInterfaceBuilder::NONE.build(None),
            p: levels_interface::Pointered::VOID,
        }
    }
    pub fn new() -> Self {
        // todo!("load the lib and call init()");
        match levels_interface::MyInterface::from_lib_safe("testpenrose.dll".to_string()) {
            Ok(mif) => {
                // todo!("call new() and save self to Level");
                let p = (mif.new)();
                let faces = (mif.get_faces)(p);
                Self { faces, mif, p }
            }
            Err(err) => {
                println!("Error: {}", err);
                Self::none()
            }
        }
    }
    pub fn get(&self) -> &Vec<my_items::Face> {
        &self.faces
    }
    pub fn when_angled(&mut self, angle: f32) -> bool {
        // todo!("call when_angled()");
        let angled = (self.mif.when_angled)(self.p, angle);
        // todo!("if it returned true, call get_faces() and write to Level");
        if !angled {
            return angled;
        }
        self.faces = (self.mif.get_faces)(self.p);
        true
    }
    #[allow(dead_code)]
    pub fn destory(self) {
        // todo!("call destory()");
        (self.mif.destory)(self.p);
        // todo!("release self");
        self.mif.close();
    }
}
