use std::sync::Arc;

use eframe::{
    egui::{self, mutex::Mutex},
    glow,
};

pub struct MyGLView {
    pub basic: Arc<Mutex<GLGameView>>,
    pub lines: Arc<Mutex<GLLinesView>>,
}

impl MyGLView {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            lines: Arc::new(Mutex::new(GLLinesView::new(gl))),
            basic: Arc::new(Mutex::new(GLGameView::new(gl))),
        }
    }
    pub fn destroy_all(&self, gl: &glow::Context) {
        self.lines.lock().destroy(gl);
        self.basic.lock().destroy(gl);
    }
}

pub trait GLGameBase {
    fn new(gl: &glow::Context) -> Self;
    fn destroy(&self, gl: &glow::Context);
    fn paint(&self, gl: &glow::Context, rect: &egui::Rect, angle: f32);
}

pub struct GLGameView {
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

impl GLGameBase for GLGameView {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                include_str!("../../assets/shaders/basic.vs"),
                include_str!("../../assets/shaders/basic.fs"),
            );
            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Self {
                program,
                vertex_array,
            }
        }
    }
    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, rect: &egui::Rect, angle: f32) {
        let aspect = rect.size().y / rect.size().x;
        let identity = glm::mat4(
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        );

        let colors = [
            0.0, 1.0, 0.5, 0.6, /* Left */
            1.0, 0.5, 0.4, 1.0, /* Top */
            1.0, 0.5, 0.0, 1.0, /* Bottom */
            0.5, 0.0, 1.0, 1.0, /* Right*/
        ];
        let points = [
            0.7, 0.0, 0.2, // Left
            0.0, 0.7, 0.2, // Top
            0.0, -0.7, 0.2, // Bottum
            -0.7, 0.0, 0.2, // Right
        ];
        let proj = [
            angle.cos(),
            0.0,
            -angle.sin(),
            0.0,
            1.0,
            0.0,
            angle.sin(),
            0.0,
            angle.cos(),
        ];

        use glow::HasContext as _;

        unsafe {
            gl.use_program(Some(self.program));
            // gl.depth_mask(true);
            gl.enable(glow::DEPTH_TEST);
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.depth_func(glow::LEQUAL);

            gl.uniform_matrix_3_f32_slice(
                gl.get_uniform_location(self.program, "u_proj").as_ref(),
                false,
                &proj,
            );
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_x_scale").as_ref(),
                aspect,
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_colors").as_ref(),
                false,
                &colors,
            );
            gl.uniform_matrix_4x3_f32_slice(
                gl.get_uniform_location(self.program, "u_points").as_ref(),
                false,
                &points,
            );
            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_use_mask").as_ref(),
                true as i32,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_mask_pos").as_ref(),
                0.0,
                0.0,
                0.0,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_mask_dir").as_ref(),
                1.0,
                1.0,
                0.0,
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            let mut angle = angle;
            for _ in 0..10 {
                let points = glm::mat4(
                    0.7, 0.0, 0.2, 0.0, // Left
                    0.0, 0.7, 0.2, 0.0, // Top
                    0.0, -0.7, 0.2, 0.0, // Bottum
                    -0.7, 0.0, 0.2, 0.0, // Right
                );
                let rot = glm::ext::rotate(&identity, angle, glm::vec3(0.0, 1.0, 0.0));
                let rot = rot * points;
                let mut pts: Vec<f32> = vec![];
                for v in rot.as_array() {
                    pts.push(v.x);
                    pts.push(v.y);
                    pts.push(v.z);
                }
                gl.uniform_matrix_4x3_f32_slice(
                    gl.get_uniform_location(self.program, "u_points").as_ref(),
                    false,
                    pts.as_slice(),
                );
                // println!("{:#?}", pts.as_slice());
                angle += (36.0f32).to_radians();

                gl.uniform_1_i32(
                    gl.get_uniform_location(self.program, "u_base_layer")
                        .as_ref(),
                    true as i32,
                );
                gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

                gl.uniform_1_i32(
                    gl.get_uniform_location(self.program, "u_base_layer")
                        .as_ref(),
                    false as i32,
                );
                gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            }

            let points = [
                0.0, 0.0, 0.0, // Left
                1.0, 1.0, -0.05, // Top
                1.0, 1.05, 0.0, // Bottum
                0.0, 0.05, 0.0, // Right
            ];
            gl.uniform_matrix_4x3_f32_slice(
                gl.get_uniform_location(self.program, "u_points").as_ref(),
                false,
                &points,
            );
            let colors = [
                1.0, 0.0, 0.0, 1.0, /* Left */
                1.0, 0.0, 0.0, 1.0, /* Top */
                1.0, 0.0, 0.0, 1.0, /* Bottom */
                1.0, 0.0, 0.0, 1.0, /* Right*/
            ];
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_colors").as_ref(),
                false,
                &colors,
            );
            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_base_layer")
                    .as_ref(),
                true as i32,
            );
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }
}

pub mod items {
    use std::{fmt::Debug, rc::Rc, sync::Arc};

    #[derive(Clone, Debug)]
    pub struct V3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Clone, Debug)]
    pub struct Musk {
        pub pos: V3,
        pub dir: V3,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
    }

    // pub trait ColorFun {
    //     fn get(&self, id: usize) -> Color {
    //         Color::default()
    //     }
    // }
    // pub trait ColorFun: Fn(usize) -> Color {
    //     fn clone_box<'a>(&self) -> Arc<dyn 'a + ColorFun>
    //     where
    //         Self: 'a;
    // }
    // impl<F: Fn(usize) -> Color + Clone> ColorFun for F {
    //     fn clone_box<'a>(&self) -> Arc<dyn 'a + ColorFun>
    //     where
    //         Self: 'a,
    //     {
    //         Arc::new(self)
    //     }
    // }
    // impl<'a> Clone for Rc<dyn 'a + ColorFun> {
    //     fn clone(&self) -> Self {
    //         (**self).clone_box()
    //     }
    // }

    pub enum Colored {
        Default,
        Pure(Color),
        Vertex(Vec<Color>),
        Fun(Arc<dyn Fn(usize) -> Color>),
    }

    impl Default for Colored {
        fn default() -> Self {
            Colored::Default
        }
    }
    impl Debug for Colored {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Default => write!(f, "Default"),
                Self::Pure(arg0) => f.debug_tuple("Pure").field(arg0).finish(),
                Self::Vertex(arg0) => f.debug_tuple("Vertex").field(arg0).finish(),
                Self::Fun(_) => write!(f, "ColorFun"),
            }
        }
    }
    impl Clone for Colored {
        fn clone(&self) -> Self {
            match self {
                Self::Default => Self::Default,
                Self::Pure(c) => Self::Pure(c.clone()),
                Self::Vertex(v_c) => Self::Vertex(v_c.clone()),
                Self::Fun(b_f) => Self::Fun(Arc::clone(b_f)),
            }
        }
    }
    impl PartialEq for Colored {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Pure(l0), Self::Pure(r0)) => l0 == r0,
                (Self::Vertex(l0), Self::Vertex(r0)) => l0 == r0,
                (Self::Fun(l0), Self::Fun(r0)) => Arc::ptr_eq(l0, r0),
                _ => false,
            }
        }
    }
    // I don't how to fix this
    unsafe impl Send for Colored {}

    impl Colored {
        pub fn get(&self, id: usize) -> Color {
            match self {
                Colored::Pure(p) => p.clone(),
                Colored::Vertex(v) => v.get(id).unwrap_or(&Color::default()).clone(),
                Colored::Fun(f) => f(id),
                _ => Color::default(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Line {
        pub pos1: V3,
        pub pos2: V3,
        pub msk: Option<Musk>,

        // pub color: Rc<Colored>,
        pub color: Colored,
    }

    impl Line {
        pub fn default(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
            Line {
                pos1: V3 {
                    x: x0,
                    y: y0,
                    z: z0,
                },
                pos2: V3 {
                    x: x1,
                    y: y1,
                    z: z1,
                },
                msk: None,
                color: Colored::Default,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Face {
        pub pos11: V3,
        pub pos12: V3,
        pub pos21: V3,
        pub pos22: V3,

        pub msk: Option<Musk>,

        pub color: Colored,
    }

    impl Face {
        pub fn default(pos: &Vec<(f32, f32, f32)>) -> Self {
            Face {
                pos11: V3 {
                    x: pos[0].0,
                    y: pos[0].1,
                    z: pos[0].2,
                },
                pos12: V3 {
                    x: pos[1].0,
                    y: pos[1].1,
                    z: pos[1].2,
                },
                pos21: V3 {
                    x: pos[2].0,
                    y: pos[2].1,
                    z: pos[2].2,
                },
                pos22: V3 {
                    x: pos[3].0,
                    y: pos[3].1,
                    z: pos[3].2,
                },
                msk: None,
                color: Colored::Default,
            }
        }
    }
}

pub struct GLLinesView {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    lines: Vec<items::Line>,
    musk_enabled: bool,
}

impl GLLinesView {
    pub fn set_lines(&mut self, line_vec: Vec<items::Line>) {
        self.lines = line_vec;
    }
    pub fn add_line(&mut self, line: items::Line) {
        self.lines.push(line);
    }
    pub fn add_lines(&mut self, mut lines: Vec<items::Line>) {
        self.lines.append(&mut lines);
    }
    pub fn set_musk_enabled(&mut self, musk: bool) {
        self.musk_enabled = musk;
    }
}

impl GLGameBase for GLLinesView {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                include_str!("../../assets/shaders/b_lines.vs"),
                include_str!("../../assets/shaders/b_lines.fs"),
            );
            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Self {
                program,
                vertex_array,
                lines: vec![],
                musk_enabled: true,
            }
        }
    }
    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, rect: &egui::Rect, angle: f32) {
        let aspect = rect.size().y / rect.size().x;
        let proj = [
            angle.cos(),
            0.0,
            -angle.sin(),
            0.0,
            1.0,
            0.0,
            angle.sin(),
            0.0,
            angle.cos(),
        ];

        use glow::HasContext as _;
        let col: &items::Colored = &items::Colored::default();

        unsafe {
            gl.use_program(Some(self.program));
            gl.enable(glow::DEPTH_TEST);
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.depth_func(glow::LEQUAL);

            gl.uniform_matrix_3_f32_slice(
                gl.get_uniform_location(self.program, "u_proj").as_ref(),
                false,
                &proj,
            );
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_x_scale").as_ref(),
                aspect,
            );
            let color = col.get(0);
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_color1").as_ref(),
                color.r,
                color.g,
                color.b,
            );
            let color = col.get(1);
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_color2").as_ref(),
                color.r,
                color.g,
                color.b,
            );
            let mut use_mask = false;
            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_use_mask").as_ref(),
                use_mask as i32,
            );
            gl.bind_vertex_array(Some(self.vertex_array));

            for l in self.lines.iter() {
                gl.uniform_3_f32(
                    gl.get_uniform_location(self.program, "u_pos1").as_ref(),
                    l.pos1.x,
                    l.pos1.y,
                    l.pos1.z,
                );
                gl.uniform_3_f32(
                    gl.get_uniform_location(self.program, "u_pos2").as_ref(),
                    l.pos2.x,
                    l.pos2.y,
                    l.pos2.z,
                );
                let col2 = &l.color;
                if !(col == col2) {
                    let col = col2;
                    let color = col.get(0);
                    // println!("{:#?}",col);
                    // println!("{:#?}",col2);
                    // todo!("test");
                    gl.uniform_3_f32(
                        gl.get_uniform_location(self.program, "u_color1").as_ref(),
                        color.r,
                        color.g,
                        color.b,
                    );
                    let color = col.get(3);
                    gl.uniform_3_f32(
                        gl.get_uniform_location(self.program, "u_color2").as_ref(),
                        color.r,
                        color.g,
                        color.b,
                    );
                }
                if self.musk_enabled {
                    if let Some(msk) = &l.msk {
                        use_mask = true;
                        gl.uniform_1_i32(
                            gl.get_uniform_location(self.program, "u_use_mask").as_ref(),
                            use_mask as i32,
                        );
                        gl.uniform_3_f32(
                            gl.get_uniform_location(self.program, "u_mask_pos").as_ref(),
                            msk.pos.x,
                            msk.pos.y,
                            msk.pos.z,
                        );
                        gl.uniform_3_f32(
                            gl.get_uniform_location(self.program, "u_mask_dir").as_ref(),
                            msk.dir.x,
                            msk.dir.y,
                            msk.dir.z,
                        );
                    } else if use_mask {
                        use_mask = false;
                        gl.uniform_1_i32(
                            gl.get_uniform_location(self.program, "u_use_mask").as_ref(),
                            use_mask as i32,
                        );
                    }
                }
                gl.draw_arrays(glow::LINES, 0, 2);
            }

            // // draw musks
            // gl.uniform_1_i32(
            //     gl.get_uniform_location(self.program, "u_use_mask").as_ref(),
            //     false as i32,
            // );
            // gl.uniform_3_f32(
            //     gl.get_uniform_location(self.program, "u_color1").as_ref(),
            //     0.9,
            //     0.1,
            //     0.1,
            // );
            // gl.uniform_3_f32(
            //     gl.get_uniform_location(self.program, "u_color2").as_ref(),
            //     0.9,
            //     0.2,
            //     0.2,
            // );
            // for l in self.lines.iter() {
            //     if let Some(msk) = &l.msk {
            //         gl.uniform_3_f32(
            //             gl.get_uniform_location(self.program, "u_pos1").as_ref(),
            //             msk.pos.x,
            //             msk.pos.y,
            //             msk.pos.z,
            //         );
            //         gl.uniform_3_f32(
            //             gl.get_uniform_location(self.program, "u_pos2").as_ref(),
            //             msk.pos.x + msk.dir.x,
            //             msk.pos.y + msk.dir.y,
            //             msk.pos.z + msk.dir.z,
            //         );
            //         gl.draw_arrays(glow::LINES, 0, 2);
            //     }
            // }
        }
    }
}
