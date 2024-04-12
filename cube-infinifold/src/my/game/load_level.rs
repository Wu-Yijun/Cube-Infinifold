use std::{sync::mpsc, thread};

use levels_interface::{self, MyInterface, Pointered};
use my_items::Face;

pub struct Level {
    handler: thread::JoinHandle<()>,
    sender: mpsc::Sender<Actions>,
    cb_recver: mpsc::Receiver<Callback>,

    faces: Vec<my_items::Face>,

    is_ok: bool,
}
enum Actions {
    GetFaces,
    Angled(f32),
    Destory,
}
enum Callback {
    Angled(bool),
    Faces(Vec<Face>),
}

impl Level {
    pub fn new() -> Option<Self> {
        let (sender, recver) = mpsc::channel();
        let (cb_sender, cb_recver) = mpsc::channel();
        let handler = thread::spawn(move || {
            let newed = my_new();
            if newed.is_none() {
                return;
            }
            let (mif, p, faces) = newed.unwrap();
            cb_sender.send(Callback::Faces(faces)).expect("Error");
            while let Ok(action) = recver.recv() {
                match action {
                    Actions::GetFaces => {
                        let faces = my_get_faces(&mif, p);
                        cb_sender.send(Callback::Faces(faces)).expect("Send Error");
                    }
                    Actions::Angled(angle) => cb_sender
                        .send(Callback::Angled(my_when_angled(&mif, p, angle)))
                        .expect("Send Error"),
                    Actions::Destory => break,
                }
                if !(mif.is_ok)() {
                    break;
                }
            }
            // destory
            my_destory(mif, p);
        });
        if let Ok(Callback::Faces(faces)) = cb_recver.recv() {
            Some(Self {
                handler,
                sender,
                cb_recver,
                faces,
                is_ok: true,
            })
        } else {
            None
        }
    }
    pub fn get_faces(&self) -> &Vec<my_items::Face> {
        &self.faces
    }
    pub fn when_angled(&mut self, angle: f32) -> bool {
        if let Err(err) = self.sender.send(Actions::Angled(angle)) {
            println!("{}", err.to_string());
            self.is_ok = false;
            return false;
        }
        if let Ok(Callback::Angled(angled)) = self.cb_recver.recv() {
            if angled {
                if let Err(err) = self.sender.send(Actions::GetFaces) {
                    println!("{}", err.to_string());
                    self.is_ok = false;
                    return false;
                }
                if let Ok(Callback::Faces(faces)) = self.cb_recver.recv() {
                    self.faces = faces;
                    return true;
                }
            } else {
                return false;
            }
        }
        // error here
        self.is_ok = false;
        false
    }
    #[allow(dead_code)]
    pub fn destory(self) {
        let _ = self.sender.send(Actions::Destory);
        let _ = self.handler.join();
    }

    pub fn is_ok(&self) -> bool {
        self.is_ok
    }
}

fn my_new() -> Option<(MyInterface, Pointered, Vec<Face>)> {
    // todo!("load the lib and call init()");
    match MyInterface::from_lib_safe("testpenrose.dll".to_string()) {
        Ok(mif) => {
            // todo!("call new() and save self to Level");
            let p = (mif.new)();
            let faces = (mif.get_faces)(p);
            Some((mif, p, faces))
        }
        Err(err) => {
            println!("Error: {}", err);
            None
        }
    }
}

fn my_destory(mif: MyInterface, p: Pointered) {
    // todo!("call destory()");
    (mif.destory)(p);
    // todo!("release self");
    mif.close();
}

fn my_when_angled(mif: &MyInterface, p: Pointered, angle: f32) -> bool {
    (mif.when_angled)(p, angle)
}

fn my_get_faces(mif: &MyInterface, p: Pointered) -> Vec<Face> {
    (mif.get_faces)(p)
}
