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
    // private
    pub const REQUIRED_INCLUDED: B = b"REQUIRED_INCLUDED\0";
    pub const CHECK_STATE: B = b"is_ok\0";
    // necessary
    pub const INIT: B = b"init\0";
    pub const NEW: B = b"new\0";
    pub const DESTORY: B = b"destory\0";

    // selective
    pub const WHEN_ANGLED: B = b"when_angled\0";
    pub const GET_FACES: B = b"get_faces\0";

    // variables
    pub const LEVEL_INFO: B = b"LEVEL_INFO\0";
}
use my_items::Face;
#[allow(unused_imports)]
// 对这个列表设置的别名为`names`, 不然太长太难用了
use variables_functions_names as names;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Pointered 是一个指针的替代品, 指向一段内存地址
///
/// 当我们不需要指针时, 返回的结果为 None, 否则为 Some(内存地址 as usize)
pub struct Pointered(Option<usize>);
impl Pointered {
    /// 空指针, 可以快速获取一个指向 null 指针
    pub const NULL: Pointered = Pointered(Some(0));
    /// 不是指针,
    pub const VOID: Pointered = Pointered(None);
    /// 错误指针, 相当于野指针
    pub const ERROR: Pointered = Pointered(Some(usize::MAX));
}

/// Pointerable 是一个很好的特征, 它保证了任意一个结构体可以化为 Pointered 指针
///
/// 内部实现的一些函数可以方便我们快速在 Self 和 Pointered 之间来回转化,
/// 这样我们的函数就可以实现统一的传入传出类型
pub trait Pointerable: Sized {
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

    pub get_faces: fn(Pointered) -> Vec<Face>,
    pub when_angled: fn(Pointered, f32) -> bool,

    pub is_ok: fn() -> bool,

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
            Err(_) => Err(String::from("Paniced!")), // Err(err) =>{ match err.downcast::<String>() {
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
        // check lib
        let req_inc: *mut bool = if let Ok(req_inc) = lib.get(names::REQUIRED_INCLUDED) {
            *req_inc
        } else {
            return Err("The library is not vaild".to_string());
        };
        if !*req_inc {
            return Err("The required is not included in library".to_string());
        }
        // get the ok test fun
        let is_ok: fn() -> bool = if let Ok(is_ok) = lib.get(names::CHECK_STATE) {
            *is_ok
        } else {
            return Err("The library is not vaild(cannot find is_ok)".to_string());
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
        // get unnecessary
        let mut mif_builder = my_interface::MyInterfaceBuilder::new(is_ok, *info, new, destory);

        if let Ok(get_faces) = lib.get(names::GET_FACES) {
            mif_builder.with_get_faces(*get_faces);
        }
        if let Ok(when_angled) = lib.get(names::WHEN_ANGLED) {
            mif_builder.with_when_angled(*when_angled);
        }
        Ok(mif_builder.build(Some(lib)))
    }
    pub fn close(self) {
        match self.lib {
            Some(lib) => {
                let _ = lib.close();
            }
            None => (),
        }
    }
}

#[cfg(feature = "cube-infinifold_main")]
#[allow(dead_code)]
pub mod my_interface {

    use my_items::Face;

    use crate::*;

    static mut NEXT_ID: usize = 0;
    pub fn next_id() -> usize {
        unsafe {
            NEXT_ID += 1;
            NEXT_ID
        }
    }
    pub struct MyInterfaceBuilder {
        pub is_ok: Option<fn() -> bool>,
        pub f_new: Option<fn() -> Pointered>,
        pub f_destory: Option<fn(Pointered) -> ()>,
        pub f_when_angled: Option<fn(Pointered, f32) -> bool>,
        pub f_get_faces: Option<fn(Pointered) -> Vec<Face>>,
        pub level_info: Option<my_interface::LevelInfo>,
    }
    impl MyInterfaceBuilder {
        pub const NONE: Self = Self {
            f_new: None,
            f_destory: None,
            f_get_faces: None,
            f_when_angled: None,
            level_info: None,
            is_ok: None,
        };
        pub const NOT_OK: fn() -> bool = || false;
        pub const NEW: fn() -> Pointered = || Pointered(None);
        pub const DESTORY: fn(Pointered) -> () = |_| ();
        pub const GET_FACES: fn(Pointered) -> Vec<Face> = |_| (vec![]);
        pub const WHEN_ANGLED: fn(Pointered, f32) -> bool = |_, _| (false);

        pub fn build(self, lib: Option<libloading::Library>) -> MyInterface {
            MyInterface {
                is_ok: self.is_ok.unwrap_or(Self::NOT_OK),
                new: self.f_new.unwrap_or(Self::NEW),
                destory: self.f_destory.unwrap_or(Self::DESTORY),
                get_faces: self.f_get_faces.unwrap_or(Self::GET_FACES),
                when_angled: self.f_when_angled.unwrap_or(Self::WHEN_ANGLED),
                level_info: self.level_info.unwrap_or(LevelInfo::NONE),
                lib,
            }
        }

        pub fn new(
            is_ok: fn() -> bool,
            info: LevelInfo,
            new: fn() -> Pointered,
            destory: fn(Pointered) -> (),
        ) -> Self {
            Self {
                level_info: Some(info),
                f_new: Some(new),
                f_destory: Some(destory),
                f_get_faces: None,
                f_when_angled: None,

                is_ok: Some(is_ok),
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
        pub fn with_when_angled(&mut self, when_angled: fn(Pointered, f32) -> bool) -> &mut Self {
            self.f_when_angled = Some(when_angled);
            self
        }
        pub fn with_get_faces(&mut self, get_faces: fn(Pointered) -> Vec<Face>) -> &mut Self {
            self.f_get_faces = Some(get_faces);
            self
        }
    }
}
