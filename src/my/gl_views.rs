use std::sync::Arc;

use eframe::{egui::mutex::Mutex, glow};
// use rand::distributions::uniform;

pub struct MyGLView {
    pub basic: Arc<Mutex<GLGameView>>,
    pub lines: Arc<Mutex<GLLinesView>>,
    pub faces: Arc<Mutex<GLFacesView>>,
}

impl MyGLView {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            basic: Arc::new(Mutex::new(GLGameView::new(gl))),
            lines: Arc::new(Mutex::new(GLLinesView::new(gl))),
            faces: Arc::new(Mutex::new(GLFacesView::new(gl))),
        }
    }
    pub fn destroy_all(&self, gl: &glow::Context) {
        self.lines.lock().destroy(gl);
        self.basic.lock().destroy(gl);
    }
}

#[derive(Clone, PartialEq)]
pub struct GlPaintOptions {
    pub angle: f32,
    pub translate: items::V3,
    pub scale: f32,
    pub aspect_ratio: f32,
}
impl Default for GlPaintOptions {
    fn default() -> Self {
        Self {
            angle: Default::default(),
            translate: Default::default(),
            scale: 1.0,
            aspect_ratio: 1.0,
        }
    }
}
impl GlPaintOptions {
    fn get_projection_mat(&self) -> [f32; 9] {
        [
            self.scale * self.angle.cos(),
            0.0,
            -self.scale * self.angle.sin(),
            0.0,
            self.scale,
            0.0,
            self.scale * self.angle.sin(),
            0.0,
            self.scale * self.angle.cos(),
        ]
    }
}

pub trait GLGameBase {
    fn new(gl: &glow::Context) -> Self;
    fn destroy(&self, gl: &glow::Context);
    fn paint(&self, gl: &glow::Context, option: &GlPaintOptions);
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

    fn paint(&self, gl: &glow::Context, option: &GlPaintOptions) {
        // let aspect = rect.size().y / rect.size().x;
        let aspect = option.aspect_ratio;
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
        let proj = option.get_projection_mat();

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
            let mut angle = option.angle;
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
    use std::{fmt::Debug, sync::Arc};

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct V2(f32, f32);
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct V3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl V3 {
        pub fn from(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct V4 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl V4 {
        pub fn from(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }
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
        pub a: f32,
    }
    impl Color {
        pub fn as_slice3(&self) -> [f32; 3] {
            [self.r, self.g, self.b]
        }
        pub fn as_slice4(&self) -> [f32; 4] {
            [self.r, self.g, self.b, self.a]
        }
        pub fn as_slice3_4(c1: &Self, c2: &Self, c3: &Self, c4: &Self) -> [f32; 12] {
            [
                c1.r, c1.g, c1.b, // vec3
                c2.r, c2.g, c2.b, // vec3
                c3.r, c3.g, c3.b, // vec3
                c4.r, c4.g, c4.b, // vec3
            ]
        }
        pub fn as_slice4_4(c1: &Self, c2: &Self, c3: &Self, c4: &Self) -> [f32; 16] {
            [
                c1.r, c1.g, c1.b, c1.a, // vec4
                c2.r, c2.g, c2.b, c2.a, // vec4
                c3.r, c3.g, c3.b, c3.a, // vec4
                c4.r, c4.g, c4.b, c4.a, // vec4
            ]
        }
    }

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
        pub fn default_with(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
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

    #[derive(Clone, Debug, Default)]
    pub struct Face {
        pub pos11: V3,
        pub pos12: V3,
        pub pos21: V3,
        pub pos22: V3,

        pub musk: Option<Musk>,

        pub color: Colored,
        /// range from -1 to 1, default to be 0
        /// the index of drawing, usually achieved by applying an z offset to gl drawing
        pub index: f32,
        /// to skip this face when drawing
        pub skipped: bool,

        pos_slice: Option<[f32; 12]>,
    }

    impl Face {
        pub fn default_with(pos: &Vec<(f32, f32, f32)>) -> Self {
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
                musk: None,
                color: Colored::Default,
                pos_slice: None,
                index: 0.0,
                skipped: false,
            }
        }
        pub fn with_musk(mut self, musk: Musk) -> Self {
            self.musk = Some(musk);
            self
        }
        pub fn with_color(mut self, color: Colored) -> Self {
            self.color = color;
            self
        }
        pub fn with_w(mut self, w: f32) -> Self {
            self.set_w(w);
            self
        }
        pub fn set_w(&mut self, w: f32) {
            self.index = w;
        }
        pub fn new(pos11: V3, pos12: V3, pos21: V3, pos22: V3) -> Self {
            Face {
                pos11,
                pos12,
                pos21,
                pos22,
                ..Default::default()
            }
        }
        pub fn new_on_x(x: f32, pos: V2, size: V2) -> Self {
            Self::new(
                V3 {
                    x,
                    y: pos.0,
                    z: pos.1,
                },
                V3 {
                    x,
                    y: pos.0 + size.0,
                    z: pos.1,
                },
                V3 {
                    x,
                    y: pos.0,
                    z: pos.1 + size.1,
                },
                V3 {
                    x,
                    y: pos.0 + size.0,
                    z: pos.1 + size.1,
                },
            )
        }
        pub fn new_on_y(y: f32, pos: V2, size: V2) -> Self {
            Self::new(
                V3 {
                    x: pos.0,
                    y,
                    z: pos.1,
                },
                V3 {
                    x: pos.0 + size.0,
                    y,
                    z: pos.1,
                },
                V3 {
                    x: pos.0,
                    y,
                    z: pos.1 + size.1,
                },
                V3 {
                    x: pos.0 + size.0,
                    y,
                    z: pos.1 + size.1,
                },
            )
        }
        pub fn new_on_z(z: f32, pos: V2, size: V2) -> Self {
            Self::new(
                V3 {
                    x: pos.0,
                    y: pos.1,
                    z,
                },
                V3 {
                    x: pos.0 + size.0,
                    y: pos.1,
                    z,
                },
                V3 {
                    x: pos.0,
                    y: pos.1 + size.1,
                    z,
                },
                V3 {
                    x: pos.0 + size.0,
                    y: pos.1 + size.1,
                    z,
                },
            )
        }
        pub fn gen_pos_slice(&mut self) {
            self.pos_slice = Some([
                self.pos11.x,
                self.pos11.y,
                self.pos11.z,
                self.pos12.x,
                self.pos12.y,
                self.pos12.z,
                self.pos21.x,
                self.pos21.y,
                self.pos21.z,
                self.pos22.x,
                self.pos22.y,
                self.pos22.z,
            ]);
        }
        pub fn get_pos_slice<'a>(&self) -> [f32; 12] {
            if let Some(v) = self.pos_slice {
                v
            } else {
                [
                    self.pos11.x,
                    self.pos11.y,
                    self.pos11.z,
                    self.pos12.x,
                    self.pos12.y,
                    self.pos12.z,
                    self.pos21.x,
                    self.pos21.y,
                    self.pos21.z,
                    self.pos22.x,
                    self.pos22.y,
                    self.pos22.z,
                ]
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Pillar(Vec<Face>);
    impl Pillar {
        pub fn new_upright(pos: V3, size: V3) -> Self {
            let color_ccc = Colored::Pure(Color {
                r: 0.8,
                g: 0.8,
                b: 0.8,
                a: 1.0,
            });
            let color_777 = Colored::Pure(Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            });
            let color_333 = Colored::Pure(Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            });
            let mut res = Vec::with_capacity(6);
            // right
            res.push(
                Face::new_on_x(pos.x + size.x, V2(pos.y, pos.z), V2(size.y, size.z))
                    .with_color(color_777.clone()),
            );
            // left
            res.push(
                Face::new_on_x(pos.x, V2(pos.y, pos.z), V2(size.y, size.z)).with_color(color_777),
            );
            // up
            res.push(
                Face::new_on_y(pos.y + size.y, V2(pos.x, pos.z), V2(size.x, size.z))
                    .with_color(color_ccc.clone()),
            );
            // down
            res.push(
                Face::new_on_y(pos.y, V2(pos.x, pos.z), V2(size.x, size.z)).with_color(color_ccc),
            );
            // front
            res.push(
                Face::new_on_z(pos.z + size.z, V2(pos.x, pos.y), V2(size.x, size.y))
                    .with_color(color_333.clone()),
            );
            // back
            res.push(
                Face::new_on_z(pos.z, V2(pos.x, pos.y), V2(size.x, size.y)).with_color(color_333),
            );
            Self(res)
        }
        pub fn into_vec(mut self) -> Vec<Face> {
            self.0
        }
        pub fn with_w(mut self, w: f32) -> Self {
            for f in self.0.iter_mut() {
                f.set_w(w);
            }
            self
        }

        pub fn set_skipped_filter_all(&mut self, skipped: bool) {
            self.set_skipped_filter(skipped, skipped, skipped, skipped, skipped, skipped);
        }
        pub fn set_skipped_filter(
            &mut self,
            right_x: bool,
            left: bool,
            top_y: bool,
            down: bool,
            front_z: bool,
            back: bool,
        ) {
            self.0[0].skipped = right_x;
            self.0[1].skipped = left;
            self.0[2].skipped = top_y;
            self.0[3].skipped = down;
            self.0[4].skipped = front_z;
            self.0[5].skipped = back;
        }
        pub fn with_skipped_filter(
            mut self,
            right_x: bool,
            left: bool,
            top_y: bool,
            down: bool,
            front_z: bool,
            back: bool,
        ) -> Self {
            self.0[0].skipped = right_x;
            self.0[1].skipped = left;
            self.0[2].skipped = top_y;
            self.0[3].skipped = down;
            self.0[4].skipped = front_z;
            self.0[5].skipped = back;
            self
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

    fn paint(&self, gl: &glow::Context, option: &GlPaintOptions) {
        // let aspect = rect.size().y / rect.size().x;
        let aspect = option.aspect_ratio;
        let proj = option.get_projection_mat();

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

pub struct GLFacesView {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    faces: Vec<items::Face>,
    musk_enabled: bool,
}

impl GLFacesView {
    pub fn set_faces(&mut self, faces: Vec<items::Face>) {
        self.faces = faces;
    }
    pub fn add_face(&mut self, face: items::Face) {
        self.faces.push(face);
    }
    pub fn add_faces(&mut self, mut faces: Vec<items::Face>) {
        self.faces.append(&mut faces);
    }
    pub fn set_musk_enabled(&mut self, musk: bool) {
        self.musk_enabled = musk;
    }
}

impl GLGameBase for GLFacesView {
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
                include_str!("../../assets/shaders/b_faces.vs"),
                include_str!("../../assets/shaders/b_faces.fs"),
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
                faces: vec![],
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

    fn paint(&self, gl: &glow::Context, option: &GlPaintOptions) {
        // let aspect = rect.size().y / rect.size().x;
        let aspect = option.aspect_ratio;
        let proj = option.get_projection_mat();

        use glow::HasContext as _;
        let col: &items::Colored = &items::Colored::default();

        unsafe {
            gl.use_program(Some(self.program));
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            // gl.depth_func(glow::GREATER);
            gl.clear(glow::DEPTH_BUFFER_BIT);

            gl.uniform_matrix_3_f32_slice(
                gl.get_uniform_location(self.program, "u_proj").as_ref(),
                false,
                &proj,
            );
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_x_scale").as_ref(),
                aspect,
            );
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_index").as_ref(),
                0.0,
            );
            gl.uniform_4_f32_slice(
                gl.get_uniform_location(self.program, "u_color").as_ref(),
                &items::Color::as_slice4_4(&col.get(0), &col.get(1), &col.get(2), &col.get(3)),
            );
            let mut use_mask = false;
            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_use_mask").as_ref(),
                use_mask as i32,
            );
            gl.bind_vertex_array(Some(self.vertex_array));

            for f in self.faces.iter().filter(|f| !f.skipped) {
                gl.uniform_3_f32_slice(
                    gl.get_uniform_location(self.program, "u_pos").as_ref(),
                    &f.get_pos_slice(),
                );
                gl.uniform_1_f32(
                    gl.get_uniform_location(self.program, "u_index").as_ref(),
                    f.index,
                );
                let col2 = &f.color;
                if !(col == col2) {
                    let col = col2;
                    gl.uniform_4_f32_slice(
                        gl.get_uniform_location(self.program, "u_color").as_ref(),
                        &items::Color::as_slice4_4(
                            &col.get(0),
                            &col.get(1),
                            &col.get(2),
                            &col.get(3),
                        ),
                    );
                }
                if self.musk_enabled {
                    if let Some(msk) = &f.musk {
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
                gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 6);
            }
        }
    }
}
