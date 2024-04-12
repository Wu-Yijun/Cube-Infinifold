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
        res.push(Face::new_on_x(pos.x, V2(pos.y, pos.z), V2(size.y, size.z)).with_color(color_777));
        // up
        res.push(
            Face::new_on_y(pos.y + size.y, V2(pos.x, pos.z), V2(size.x, size.z))
                .with_color(color_ccc.clone()),
        );
        // down
        res.push(Face::new_on_y(pos.y, V2(pos.x, pos.z), V2(size.x, size.z)).with_color(color_ccc));
        // front
        res.push(
            Face::new_on_z(pos.z + size.z, V2(pos.x, pos.y), V2(size.x, size.y))
                .with_color(color_333.clone()),
        );
        // back
        res.push(Face::new_on_z(pos.z, V2(pos.x, pos.y), V2(size.x, size.y)).with_color(color_333));
        Self(res)
    }
    pub fn into_vec(self) -> Vec<Face> {
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
