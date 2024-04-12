mod exporting_functions_and_variables {
    use levels_interface::Pointered;
    use my_items::Face;

    #[no_mangle]
    static REQUIRED_INCLUDED: bool = true;

    #[no_mangle]
    fn init() {
        match std::panic::catch_unwind(|| crate::init()) {
            Ok(()) => (),
            Err(err) => {
                println!("{:#?}", err);
                unsafe { STATE_IS_OK = false };
            }
        }
    }

    static mut STATE_IS_OK: bool = true;
    #[no_mangle]
    fn is_ok() -> bool {
        unsafe { STATE_IS_OK }
    }

    #[no_mangle]
    fn new() -> Pointered {
        match std::panic::catch_unwind(|| crate::new()) {
            Ok(ok) => ok,
            Err(err) => {
                println!("{:#?}", err);
                unsafe { STATE_IS_OK = false };
                Pointered::VOID
            }
        }
    }

    #[no_mangle]
    fn destory(p: Pointered) {
        match std::panic::catch_unwind(|| crate::destory(p)) {
            Ok(ok) => ok,
            Err(err) => {
                println!("{:#?}", err);
                unsafe { STATE_IS_OK = false };
            }
        }
    }

    #[no_mangle]
    fn when_angled(p: Pointered, angle: f32) -> bool {
        match std::panic::catch_unwind(|| crate::when_angled(p, angle)) {
            Ok(ok) => ok,
            Err(err) => {
                println!("{:#?}", err);
                unsafe { STATE_IS_OK = false };
                false
            }
        }
    }

    #[no_mangle]
    fn get_faces(p: Pointered) -> Vec<Face> {
        match std::panic::catch_unwind(|| crate::get_faces(p)) {
            Ok(ok) => ok,
            Err(err) => {
                println!("{:#?}", err);
                unsafe { STATE_IS_OK = false };
                vec![]
            }
        }
    }
}
