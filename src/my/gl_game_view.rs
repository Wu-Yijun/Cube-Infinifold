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
    pub struct V3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    pub struct Musk {
        pub pos: V3,
        pub dir: V3,
    }

    pub struct Line {
        pub pos1: V3,
        pub pos2: V3,
        pub msk: Option<Musk>,
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
            }
        }
    }
}

pub struct GLLinesView {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    lines: Vec<items::Line>,
}

impl GLLinesView {
    pub fn set_lines(&mut self, line_vec: Vec<items::Line>) {
        self.lines = line_vec;
    }
    pub fn add_lines(&mut self, line: items::Line) {
        self.lines.push(line);
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
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_color1").as_ref(),
                0.1,
                0.1,
                0.1,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "u_color2").as_ref(),
                0.2,
                0.9,
                0.2,
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
                if let Some(msk) = &l.msk {
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
                gl.draw_arrays(glow::LINES, 0, 2);
            }
        }
    }
}
