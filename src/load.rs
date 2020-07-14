use crate::types::{MyGame, Enemy, Object, EnemyData};
use crate::util;

use std::fs::File;
use std::io::Read;

impl MyGame {
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
                x: view[0] as i32 * 256 + view[1] as i32,
                y: view[2] as i32,
                dir: (view[4] as i32) - 1,
                data: enemy_data
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

        let model = EnemyData {
            model_id: bytes[0]
        };

        self.enemies_cache.insert(id, model.clone());
        Ok(model)
    }
    
    pub fn load_object<'a>(&mut self, id: u8) -> std::io::Result<Object> {
        if self.objects_cache.contains_key(&id) {
            return Ok(self.objects_cache.get(&id).unwrap().clone());
        }

        let file = File::open(format!("data/objects/{}.dat", id))?;
        let bytes = file.bytes().collect::<std::io::Result<Vec<u8>>>()?; 

        let obj = Object {
            width: bytes[0] as u32,
            height: bytes[1] as u32,
            data: util::uncompress(bytes[2..].to_vec())
        };

        self.objects_cache.insert(id, obj.clone());
        Ok(obj)
    }
}
