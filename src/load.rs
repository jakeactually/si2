use crate::types::{Game, Enemy, Object, EnemyData, Vec2};
use crate::util;

use std::fs::File;
use std::io::Read;

impl Game {
    pub fn load_level<'a>(&mut self, id: u8) -> std::io::Result<Vec<Enemy>> {
        let file = File::open(format!("data/levels/{}.dat", id))?;
        let bytes = file.bytes().collect::<std::io::Result<Vec<u8>>>()?; 

        let amount = bytes[0];
        let mut result = vec![];

        for i in 0..amount {
            let offset = i * 5;
            let view = bytes[(offset as usize + 1)..(offset as usize + 6)].to_vec();
            let enemy_id = view[3];
            let enemy_data = self.load_enemy(enemy_id)?;

            let enemy = Enemy {
                id: enemy_id,
                position: Vec2 {
                    x: view[0] as i32 * 256 + view[1] as i32,
                    y: view[2] as i32
                },
                dir: (view[4] as i32) - 1,
                data: enemy_data,
                explosion_frames: 6,
                anim_state: 0
            };

            result.push(enemy);
        }

        Ok(result)
    }

    pub fn load_enemy<'a>(&mut self, id: u8) -> std::io::Result<EnemyData> {
        if self.enemies_cache.contains_key(&id) {
            return Ok(self.enemies_cache.get(&id).unwrap().clone());
        }

        let file = File::open(format!("data/enemies/{}.dat", id))?;
        let bytes = file.bytes().collect::<std::io::Result<Vec<u8>>>()?; 

        let mut enemy = EnemyData {
            model_id: bytes[0],
            size: Vec2 { x: 0, y: 0 },
            anim_count: bytes[1],
            lives: bytes[2] as i8,
            floats: bytes[3] == 1,
            shot_time: bytes[4],
            move_up: bytes[5] == 1,
            move_down: bytes[6] == 1,
            move_anyway: bytes[7] == 1,
            moves_between: Vec2 { x: bytes[8] as i32, y: bytes[9] as i32 }
        };

        for _ in enemy.model_id..enemy.anim_count {
            let obj = self.load_object(enemy.model_id)?;

            if enemy.size.x < obj.size.x {
                enemy.size.x = obj.size.x;
            }

            if enemy.size.y < obj.size.y {
                enemy.size.y = obj.size.y;
            }
        }

        self.enemies_cache.insert(id, enemy.clone());
        Ok(enemy)
    }
    
    pub fn load_object<'a>(&mut self, id: u8) -> std::io::Result<Object> {
        if self.objects_cache.contains_key(&id) {
            return Ok(self.objects_cache.get(&id).unwrap().clone());
        }

        let file = File::open(format!("data/objects/{}.dat", id))?;
        let bytes = file.bytes().collect::<std::io::Result<Vec<u8>>>()?; 

        let obj = Object {
            size: Vec2 {
                x: bytes[0] as i32,
                y: bytes[1] as i32
            },
            data: util::uncompress(bytes[2..].to_vec())
        };

        self.objects_cache.insert(id, obj.clone());
        Ok(obj)
    }
}
