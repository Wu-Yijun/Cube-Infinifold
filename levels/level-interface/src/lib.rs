use libloading;

#[allow(dead_code)]
/// 导入函数名和导入变量名的列表, 可以作为编写库时的参考或者编写接口的导入功能时的引用.
pub mod variables_functions_names {
    pub type S = &'static str;
    pub type B = &'static [u8];
    pub fn s2b(s: S) -> B {
        s.as_bytes()
    }
    pub fn b2s(b: B) -> S {
        unsafe { core::mem::transmute(b) }
    }

    // functions
    // necessary
    pub const INIT: B = b"init\0";
    pub const NEW: B = b"new\0";
    pub const DESTORY: B = b"destory\0";

    // test
    pub const APPEND: B = b"append\0";
    pub const SHOW: B = b"show\0";

    // variables
    pub const LEVEL_INFO: B = b"LEVEL_INFO\0";
}
#[allow(unused_imports)]
// 对这个列表设置的别名为`names`, 不然太长太难用了
use variables_functions_names as names;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Pointered 是一个指针的替代品, 指向一段内存地址
///
/// 当我们不需要指针时, 返回的结果为 None, 否则为 Some(内存地址 as usize)
pub struct Pointered(Option<usize>);

/// Pointerable 是一个很好的特征, 它保证了任意一个结构体可以化为 Pointered 指针
///
/// 内部实现的一些函数可以方便我们快速在 Self 和 Pointered 之间来回转化,
/// 这样我们的函数就可以实现统一的传入传出类型
pub trait Pointerable: Sized {
    /// 不是指针,
    const VOID: Pointered = Pointered(None);
    /// 空指针, 可以快速获取一个指向 null 指针
    const NULL: Pointered = Pointered(Some(0));
    /// 从指针获取 self, 可能失败
    ///
    /// 失败的原因有两种, NULL 或者 VOID, 这些都不会
    fn from_pointer(p: Pointered) -> Option<&'static mut Self> {
        let address = p.0?;
        if address == 0 {
            return None;
        }
        let t = unsafe {
            let refer = address as *mut usize;
            let refer = refer as *mut Self;
            &mut *refer
        };
        Some(t)
    }
    /// 获取指向 self 的指针
    fn get_pointer(&self) -> Pointered {
        let r = self as *const Self;
        let r = r as *const usize;
        let address = r as usize;
        Pointered(Some(address))
    }
    /// 将值变为静态值
    fn as_static(self) -> &'static Self {
        Box::leak(Box::new(self))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LevelInfo {
    pub id: usize,
    pub name: &'static str,
    pub group: &'static str,
}
impl LevelInfo {
    pub const NONE: Self = Self {
        id: 0,
        name: "",
        group: "",
    };
}

#[derive(Debug)]
pub struct MyInterface {
    /// This function is loaded and called after we have loaded the lib.
    /// ```Rust
    /// #[no_mangle]
    /// pub fn new() -> Pointered {
    ///     // codes here ...
    /// }
    /// ```
    /// Once the lib is loaded, we call init()->() function, to help you init variables.
    /// ```Rust
    /// #[no_mangle]
    /// pub fn init() {
    ///     // init LEVEL_INFO here ...
    /// }
    /// ```
    pub new: fn() -> Pointered,
    /// This function is called when the level is decided to be closed, we can clear info here
    ///
    /// *As the lib is going to be closed, it may be meaningless to collect garbage*
    /// ```Rust
    /// #[no_mangle]
    /// pub fn destory(p: Pointered) {
    ///     // codes here ...
    /// }
    /// ```
    pub destory: fn(Pointered) -> (),
    /// This stores the info of level, it is a ! [@important] (static) variable and should not change.
    ///
    /// The loader will in fact copy it and never update it even it changed
    ///
    /// and it was loaded right after the init() function is called.
    /// ```Rust
    /// #[no_mangle]
    /// pub static LEVEL_INFO: LevelInfo = LevelInfo::NONE;
    /// ```
    pub level_info: LevelInfo,

    pub append: fn(Pointered, &str) -> (),
    pub show: fn(Pointered) -> (),

    /// We set a lib here to ensure the lib is not closed at the end of the function
    #[allow(dead_code)]
    lib: Option<libloading::Library>,
}
#[cfg(feature = "cube-infinifold_main")]
impl MyInterface {
    pub fn from_lib_safe(path: String) -> Result<Self, String> {
        let result = std::panic::catch_unwind(|| unsafe { Self::from_lib(path) });
        match result {
            Ok(s) => s,
            Err(_) => Err(String::from("Paniced!"))
            // Err(err) =>{ match err.downcast::<String>() {
            //     Ok(res) => Err(*res),
            //     Err(_) => Err("Unknown error occured when loading library".to_string()),
            // }},
        }
    }
    pub unsafe fn from_lib(path: String) -> Result<Self, String> {
        let lib = match libloading::Library::new(path) {
            Ok(lib) => lib,
            Err(err) => return Err(err.to_string()),
        };
        // initialization
        if let Ok(init) = lib.get::<libloading::Symbol<fn()>>(names::INIT) {
            init();
        };
        // get necessary funs and vars
        let info: *mut LevelInfo = if let Ok(info) = lib.get(names::LEVEL_INFO) {
            *info
        } else {
            return Err("Cannot find LEVEL_INFO".to_string());
        };
        let new: fn() -> Pointered = if let Ok(new) = lib.get(names::NEW) {
            *new
        } else {
            return Err("Cannot find new".to_string());
        };
        let destory: fn(Pointered) = if let Ok(destory) = lib.get(names::DESTORY) {
            *destory
        } else {
            return Err("Cannot find destory".to_string());
        };
        #[allow(unused_mut)]
        let mut bd = my_interface::MyInterfaceBuilder::new(*info, new, destory)
            // .with_info(info)
            // .with_new(new)
            // .with_destory(destory)
            ;

        let new: fn(Pointered, &str) -> () = if let Ok(new) = lib.get(names::APPEND) {
            *new
        } else {
            return Err("Cannot find append".to_string());
        };
        let destory: fn(Pointered) = if let Ok(destory) = lib.get(names::SHOW) {
            *destory
        } else {
            return Err("Cannot find show".to_string());
        };
        Ok(bd.build(lib, new, destory))
    }
}

#[cfg(feature = "cube-infinifold_main")]
#[allow(dead_code)]
pub mod my_interface {

    use crate::*;

    static mut NEXT_ID: usize = 0;
    pub fn next_id() -> usize {
        unsafe {
            NEXT_ID += 1;
            NEXT_ID
        }
    }
    pub struct MyInterfaceBuilder {
        pub f_new: Option<fn() -> Pointered>,
        pub f_destory: Option<fn(Pointered) -> ()>,
        pub level_info: Option<my_interface::LevelInfo>,
    }
    impl MyInterfaceBuilder {
        pub const NONE: Self = Self {
            f_new: None,
            f_destory: None,
            level_info: None,
        };
        pub const NEW: fn() -> Pointered = || Pointered(None);
        pub const DESTORY: fn(Pointered) -> () = |_| ();

        pub fn build(
            self,
            lib: libloading::Library,
            a: fn(Pointered, &str) -> (),
            s: fn(Pointered) -> (),
        ) -> MyInterface {
            MyInterface {
                new: self.f_new.unwrap_or(Self::NEW),
                destory: self.f_destory.unwrap_or(Self::DESTORY),
                level_info: self.level_info.unwrap_or(LevelInfo::NONE),
                append: a,
                show: s,
                lib: Some(lib),
            }
        }

        pub fn new(info: LevelInfo, new: fn() -> Pointered, destory: fn(Pointered) -> ()) -> Self {
            Self {
                level_info: Some(info),
                f_new: Some(new),
                f_destory: Some(destory),
            }
        }
        pub fn with_info(&mut self, info: LevelInfo) -> &mut Self {
            self.level_info = Some(info);
            self
        }
        pub fn with_new(&mut self, new: fn() -> Pointered) -> &mut Self {
            self.f_new = Some(new);
            self
        }
        pub fn with_destory(&mut self, destory: fn(Pointered) -> ()) -> &mut Self {
            self.f_destory = Some(destory);
            self
        }
    }
}

#[cfg(feature = "cube-infinifold_main")]
#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MyType {
        pub name: String,
        pub ids: Vec<i32>,
    }
    impl Pointerable for MyType {}

    #[test]
    fn pointerable() {
        let my = MyType {
            name: "你好".to_string(),
            ids: vec![1, 5, 6, 8, 2],
        };
        let p = my.get_pointer();
        {
            let you = MyType::from_pointer(p).expect("Get a null pointer");
            println!("{:#?}", you);
            you.ids.pop();
            you.ids.push(123);
            you.name = "世界".to_string();
        }
        {
            let him = MyType::from_pointer(p).expect("Get a null pointer");
            println!("{:#?}", him);
            him.ids.pop();
            him.ids.push(456);
            him.name = "!!".to_string();
        }
    }

    static mut MY: MyType = MyType {
        name: String::new(),
        ids: vec![],
    };
    fn my_new() -> Pointered {
        unsafe {
            MY = MyType {
                name: "你好".to_string(),
                ids: vec![1, 5, 6, 8, 2],
            }
        };
        unsafe { MY.get_pointer() }
    }
    fn my_destory(p: Pointered) {
        if let Some(my) = MyType::from_pointer(p) {
            my.ids.clear();
            my.name.clear();
            #[allow(dropping_references)]
            drop(my);
        } else {
            panic!("Cannot destory void!");
        }
    }
    // #[test]
    // fn interfacable() {
    //     let my = MyInterface {
    //         new: my_new,
    //         destory: my_destory,
    //         level_info: LevelInfo::NONE,
    //         lib: None,
    //     };
    //     let p = (my.new)();
    //     {
    //         let you = MyType::from_pointer(p).expect("Get a null pointer");
    //         println!("{:#?}", you);
    //         you.ids.pop();
    //         you.ids.push(123);
    //         you.name = "世界".to_string();
    //     }
    //     {
    //         let him = MyType::from_pointer(p).expect("Get a null pointer");
    //         println!("{:#?}", him);
    //         him.ids.pop();
    //         him.ids.push(456);
    //         him.name = "!!".to_string();
    //     }
    //     (my.destory)(p);
    //     {
    //         let her = MyType::from_pointer(p).expect("Get a null pointer");
    //         println!("{:#?}", her);
    //         her.ids.pop();
    //         her.ids.push(456);
    //         her.name = "!!".to_string();
    //     }
    // }

    #[test]
    fn load() {
        let p = MyInterface::from_lib_safe("testlevel.dll".to_string());
        println!("{:#?}", p);
    }
}
