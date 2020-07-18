use crate::types::Object;

use std::collections::HashMap;

pub fn uncompress(data: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];

    for byte in data.iter() {
        for i in 0..8 {
            result.push((byte >> (7 - i)) & 1);
        }
    }

    result
}

pub fn does_collide(obj1: Object, x1: i32, y1: i32, obj2: Object, x2: i32, y2: i32) -> bool {
    let mut collisions: HashMap<(i32, i32), u32> = HashMap::new();

    for ry in 0..obj1.height as i32 {
        for rx in 0..obj1.width as i32 {
            let offset = (ry * obj1.width as i32 + rx) as usize;

            if  offset < obj1.data.len() && obj1.data[offset as usize] == 1 {
                let key = (x1 + rx, y1 + ry);
                collisions.insert(key, *collisions.get(&key).unwrap_or(&0) + 1);
            }
        }
    }

    for ry in 0..obj2.height as i32 {
        for rx in 0..obj2.width as i32 {
            let offset = (ry * obj2.width as i32 + rx) as usize;

            if  offset < obj2.data.len() && obj2.data[offset as usize] == 1 {
                let key = (x2 + rx, y2 + ry);
                collisions.insert(key, *collisions.get(&key).unwrap_or(&0) + 1);
            }
        }
    }

    collisions.values().any(|x| *x > 1)
}
