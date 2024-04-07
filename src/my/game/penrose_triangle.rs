use crate::my::gl_views::items::{self, Face, V3};

pub fn get() -> Vec<items::Face>{
    let mut res = Vec::new();
    res.append(&mut Face::new_pillar(V3::from(-4.0, -2.0, -1.0), V3::from(8.0, 2.0, 2.0)));
    res
}