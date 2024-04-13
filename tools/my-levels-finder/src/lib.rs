#[allow(dead_code)]
mod item;

use std::fs;

use item::Game;
use json::JsonValue;

pub fn get_json(path: &str) -> JsonValue {
    let documnet = fs::read_to_string(path).unwrap_or("".to_string());
    json::parse(&documnet).unwrap_or(JsonValue::Null)
}

pub fn js_str<'a>(j: &'a JsonValue, default: &'a str) -> &'a str {
    if let JsonValue::String(s) = j {
        s
    } else if let JsonValue::Short(s) = j {
        s.as_str()
    } else {
        default
    }
}
pub fn js_num(j: &JsonValue, default: i64) -> i64 {
    if let JsonValue::Number(s) = j {
        s.as_fixed_point_i64(0).unwrap_or(default)
    } else {
        default
    }
}
pub fn js_obj_str<'a>(j: &'a JsonValue, key: &str, default: &'a str) -> &'a str {
    if let JsonValue::Object(o) = j {
        if let Some(j) = o.get(key) {
            return js_str(j, default);
        }
    }
    default
}
pub fn js_obj_num(j: &JsonValue, key: &str, default: i64) -> i64 {
    if let JsonValue::Object(o) = j {
        if let Some(j) = o.get(key) {
            return js_num(j, default);
        }
    }
    default
}
pub fn js_obj_arr<'a>(j: &'a JsonValue, key: &str) -> Option<&'a Vec<JsonValue>> {
    if let JsonValue::Object(o) = j {
        if let JsonValue::Array(arr) = o.get(key)? {
            return Some(arr);
        }
    }
    None
}

pub use item::CollectedGame;
pub use item::Link;
pub fn get_levels(path: Link, filename: &str) -> CollectedGame {
    let j = get_json(&path.with(filename).path());
    let mut g: Game = item::FromJson::get(&path, &j);
    g.find();
    g.collect()
}

#[cfg(test)]
mod tests {
    use json::{self, object};

    use crate::{
        get_json,
        item::{FromJson, Game, Link},
    };

    #[test]
    fn parse_file() {
        let path = Link::new("../../");
        let filename = "levels.json";
        let j = get_json(&path.with(filename).path());
        let mut g = Game::get(&path, &j);
        println!("{:#?}", j);
        println!("{:#?}", g);
        g.find();
        println!("{:#?}", g);
        let c = g.collect();
        println!("{:#?}", c);
    }

    #[test]
    fn json_works() {
        let parsed = json::parse(
            r#"{
                "code": 200,
                "success": true,
                "payload": {
                    "features": [
                        "awesome",
                        "easyAPI",
                        "lowLearningCurve"
                    ]
                }
            }"#,
        )
        .unwrap();

        let instantiated = object! {
            // quotes on keys are optional
            "code": 200,
            success: true,
            payload: {
                features: [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ]
            }
        };

        assert_eq!(parsed, instantiated);
    }
}
