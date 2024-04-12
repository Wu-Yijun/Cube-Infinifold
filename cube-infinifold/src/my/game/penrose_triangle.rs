// use std::f32::consts::PI;

use my_items::{Pillar, V3};

const S2: f32 = 1.414213562373095;

trait InRange {
    fn in_range(self, begin: Self, end: Self) -> bool;
}

impl InRange for f32 {
    fn in_range(self, begin: f32, end: f32) -> bool {
        self >= begin && self < end
    }
}

struct Content {
    pub base: Pillar,
    pub left: Pillar,
    pub right: Pillar,
    pub musk: Pillar,
    pub top: Pillar,
    pub front: Pillar,
    pub musk2: Pillar,

    pub up: (Pillar, Pillar, Pillar),
    pub back: Pillar,
    pub shrink: (Pillar, Pillar),
    pub shrink2: Pillar,
    pub shrink3: (Pillar, Pillar),
}

#[derive(PartialEq)]
enum State {
    Basic,
    NoMusk,
    AtTop,
    Transform,
    Shrink,
    Shrink2,
    Shrink3,
}

pub struct PenroseTriangle {
    faces: Vec<my_items::Face>,
    // updated: bool,
    state: State,

    content: Content,
}
impl PenroseTriangle {
    pub fn new() -> Self {
        let content = Content {
            base: Pillar::new_upright(V3::from(-6.0, -2.0, -1.0), V3::from(12.0, 2.0, 2.0)),
            left: Pillar::new_upright(V3::from(-6.0, -2.0, -11.0), V3::from(2.0, 2.0, 12.0)),
            right: Pillar::new_upright(V3::from(4.0, 0.0, -1.0), V3::from(2.0, 10.0, 2.0)),
            top: Pillar::new_upright(V3::from(4.0, 8.0, -1.0), V3::from(2.0, 2.0, 12.0)),
            front: Pillar::new_upright(V3::from(4.0, 8.0, 9.0), V3::from(12.0, 2.0, 2.0)),
            musk: Pillar::new_upright(V3::from(-6.0, -2.0, -11.0), V3::from(2.0, 2.0, 4.0))
                .with_w(0.5)
                .with_skipped_filter(false, true, false, true, true, true),
            musk2: Pillar::new_upright(V3::from(4.0, 0.0, -1.0), V3::from(2.0, 2.0, 2.0))
                .with_w(0.5)
                .with_skipped_filter(false, true, true, true, false, true),

            up: (
                Pillar::new_upright(V3::from(4.0, 8.0, 9.0), V3::from(8.0, 2.0, 2.0)),
                Pillar::new_upright(V3::from(10.0, 8.0, 9.0), V3::from(2.0, 6.0, 2.0)),
                Pillar::new_upright(V3::from(10.0, 12.0, 9.0), V3::from(12.0, 2.0, 2.0)),
            ),
            back: Pillar::new_upright(V3::from(4.0, 4.0, -1.0), V3::from(-4.0, 2.0, 2.0)),
            shrink: (
                Pillar::new_upright(V3::from(4.0, -2.0, -1.0), V3::from(2.0, 12.0, 2.0)),
                Pillar::new_upright(
                    V3::from(4.0, 5.0 * S2 - 2.0, -11.0),
                    V3::from(2.0, 2.0, 12.0),
                ),
            ),
            shrink2: Pillar::new_upright(V3::from(4.0, 8.0, -1.0), V3::from(2.0, 2.0, -5.8)),
            shrink3: (
                Pillar::new_upright(V3::from(4.0, -2.0, -1.0), V3::from(2.0, 6.0, 2.0)),
                Pillar::new_upright(V3::from(4.0, 4.0, -1.0), V3::from(2.0, 2.0, 7.7)),
            ),
        };
        let mut s = Self {
            content,
            faces: vec![],
            // updated: true,
            state: State::Basic,
        };
        s.gen_vec();
        s
    }
    // /// Is the faces inside is changed
    // /// Update state will disappear when set_updated method is called
    // pub fn updated(&self) -> bool {
    //     self.updated
    // }
    // pub fn set_updated(&mut self, state: bool) {
    //     self.updated = state;
    // }
    /// this will not disable update state
    pub fn get(&self) -> &Vec<my_items::Face> {
        &self.faces
    }
    fn gen_vec(&mut self) {
        self.faces.clear();
        match self.state {
            State::Basic => {
                self.faces.append(&mut self.content.base.clone().into_vec());
                self.faces.append(&mut self.content.left.clone().into_vec());
                self.faces
                    .append(&mut self.content.right.clone().into_vec());
                self.faces.append(&mut self.content.musk.clone().into_vec());
            }
            State::AtTop => {
                self.faces
                    .append(&mut self.content.right.clone().into_vec());
                self.faces.append(&mut self.content.top.clone().into_vec());
                self.faces
                    .append(&mut self.content.front.clone().into_vec());
                self.faces
                    .append(&mut self.content.musk2.clone().into_vec());
            }
            State::Transform => {
                self.faces
                    .append(&mut self.content.right.clone().into_vec());
                self.faces.append(&mut self.content.top.clone().into_vec());
                self.faces.append(&mut self.content.up.0.clone().into_vec());
                self.faces.append(&mut self.content.up.1.clone().into_vec());
                self.faces.append(&mut self.content.up.2.clone().into_vec());
                self.faces.append(&mut self.content.back.clone().into_vec());
            }
            State::NoMusk => {
                self.faces.append(&mut self.content.base.clone().into_vec());
                self.faces.append(&mut self.content.left.clone().into_vec());
                self.faces
                    .append(&mut self.content.right.clone().into_vec());
            }
            State::Shrink => {
                self.faces
                    .append(&mut self.content.shrink.0.clone().into_vec());
                self.faces
                    .append(&mut self.content.shrink.1.clone().into_vec());
            }
            State::Shrink2 => {
                self.faces
                    .append(&mut self.content.shrink.0.clone().into_vec());
                self.faces
                    .append(&mut self.content.shrink2.clone().into_vec());
            }
            State::Shrink3 => {
                self.faces
                    .append(&mut self.content.shrink3.0.clone().into_vec());
                self.faces
                    .append(&mut self.content.shrink3.1.clone().into_vec());
            }
        }
    }

    fn range_and_state(&mut self, angle: f32, begin: f32, end: f32, state: State) -> bool {
        if angle.in_range(begin, end) {
            if self.state == state {
                false
            } else {
                self.state = state;
                self.gen_vec();
                true
            }
        } else {
            false
        }
    }

    /// when the angle changed, you can call this to test an update
    ///
    /// return true when there is a update, not affected by self.updated, false when called at next time
    ///
    /// **not change update state**
    pub fn when_angled(&mut self, angle: f32) -> bool {
        let angle = angle.to_degrees();
        // let angle = angle.to_degrees().rem_euclid(360_f32);

        self.range_and_state(angle, -100f32, -45_f32, State::Transform)
            || self.range_and_state(angle, -45_f32, 0_f32, State::AtTop)
            || self.range_and_state(angle, 0_f32, 45_f32, State::Basic)
            || self.range_and_state(angle, 45_f32, 315_f32, State::NoMusk)
            || self.range_and_state(angle, 315_f32, 405_f32, State::Shrink)
            || self.range_and_state(angle, 405_f32, 585_f32, State::Shrink2)
            || self.range_and_state(angle, 585_f32, 1000_f32, State::Shrink3)
    }
    // pub fn set_angle(&mut self, angle: f32) {
    //     if self.when_angled(angle) {
    //         self.updated = true;
    //     }
    // }
}
