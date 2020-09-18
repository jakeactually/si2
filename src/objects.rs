use crate::types;
use crate::util;

use types::{Object, Vec2, SceneryData};

pub const PMNUM: [[u8; 15]; 10] = [
    [1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1],
    [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
    [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
];

pub const PMSPACE: [u8; 804] = [0; 804];

pub const PMINTRO: [u8; 531] = [0; 531];

pub const PMIMPACT: [u8; 912] = [0; 912];

pub const CMSPACE: [u8; 101] = [
    15, 255, 63, 248, 127, 131, 252, 127, 227, 255, 199, 255, 159, 249, 255, 143, 252, 120, 0, 224,
    231, 15, 60, 3, 192, 30, 0, 56, 28, 225, 207, 0, 120, 3, 192, 7, 3, 28, 57, 224, 15, 0, 255,
    240, 255, 231, 255, 60, 1, 255, 143, 255, 31, 248, 255, 231, 0, 63, 224, 3, 231, 192, 28, 121,
    224, 15, 128, 0, 124, 248, 7, 143, 60, 1, 240, 0, 15, 31, 0, 241, 231, 128, 62, 1, 255, 227,
    224, 28, 56, 255, 231, 255, 127, 240, 248, 7, 143, 7, 249, 255, 12,
];

pub const CMINTRO: [u8; 65] = [
    0, 0, 0, 0, 0, 0, 45, 193, 128, 0, 0, 14, 0, 2, 244, 27, 0, 0, 2, 120, 0, 191, 168, 240, 0, 0,
    62, 156, 11, 219, 41, 128, 0, 9, 228, 242, 220, 163, 192, 0, 0, 224, 125, 0, 1, 176, 0, 0, 32,
    19, 192, 0, 96, 0, 0, 0, 1, 192, 0, 0, 0, 0, 0, 0, 2,
];

pub const CMIMPACT: [u8; 114] = [
    31, 31, 135, 207, 252, 63, 193, 254, 127, 241, 225, 252, 252, 255, 231, 254, 127, 231, 255, 30,
    31, 255, 204, 30, 225, 231, 128, 7, 131, 225, 255, 249, 193, 206, 28, 240, 0, 240, 62, 63, 255,
    156, 24, 225, 207, 0, 15, 3, 195, 206, 241, 255, 159, 252, 240, 0, 240, 60, 60, 207, 31, 241,
    255, 206, 0, 15, 3, 195, 192, 227, 192, 28, 121, 224, 1, 224, 60, 120, 14, 60, 3, 199, 158, 0,
    30, 7, 135, 129, 227, 192, 60, 121, 224, 1, 224, 120, 120, 30, 60, 3, 199, 31, 252, 30, 15,
    143, 1, 231, 128, 120, 240, 127, 131, 192,
];

pub const PMSCROLLMARK: [u8; 21] = [
    1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0,
];

pub const PMDOTEMPTY: [u8; 12] = [0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0];

pub const PMDOTFULL: [u8; 12] = [0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0];

pub const PMLIFE: [u8; 25] = [
    1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0,
];

pub const PMMISSILEICON: [u8; 25] = [
    0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0,
];

pub const PMBEAMICON: [u8; 25] = [
    0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const PMWALLICON: [u8; 25] = [
    0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0,
];

pub const PMSHOT: [u8; 3] = [1, 1, 1];

pub const PMEXPLOSION: [[u8; 25]; 2] = [
    [
        0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
];

pub fn get_static_objects() -> [Object; 23] {
    [
        /* Statikus objektumok, az elemekre a Graphics enumeráció tárol neveket */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[0].to_vec(),
        }, /* gNum0 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[1].to_vec(),
        }, /* gNum1 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[2].to_vec(),
        }, /* gNum2 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[3].to_vec(),
        }, /* gNum3 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[4].to_vec(),
        }, /* gNum4 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[5].to_vec(),
        }, /* gNum5 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[6].to_vec(),
        }, /* gNum6 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[7].to_vec(),
        }, /* gNum7 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[8].to_vec(),
        }, /* gNum8 */
        Object {
            size: Vec2 {
                x: 3,
                y: 5
            },
            data: PMNUM[9].to_vec(),
        }, /* gNum9 */
        Object {
            size: Vec2 {
                x: 67,
                y: 12
            },
            data: util::uncompress(CMSPACE.to_vec()),
        }, /* gSpace */
        Object {
            size: Vec2 {
                x: 59,
                y: 9
            },
            data: util::uncompress(CMINTRO.to_vec()),
        }, /* gIntro */
        Object {
            size: Vec2 {
                x: 76,
                y: 12
            },
            data: util::uncompress(CMIMPACT.to_vec()),
        }, /* gImpact */
        Object {
            size: Vec2 {
                x: 3,
                y: 7
            },
            data: PMSCROLLMARK.to_vec(),
        }, /* gScrollMark */
        Object {
            size: Vec2 {
                x: 4,
                y: 3
            },
            data: PMDOTEMPTY.to_vec(),
        }, /* gDotEmpty */
        Object {
            size: Vec2 {
                x: 4,
                y: 3
            },
            data: PMDOTFULL.to_vec(),
        }, /* gDotFull */
        Object {
            size: Vec2 {
                x: 5,
                y: 5
            },
            data: PMLIFE.to_vec(),
        }, /* gLife */
        Object {
            size: Vec2 {
                x: 5,
                y: 5
            },
            data: PMMISSILEICON.to_vec(),
        }, /* gMissileIcon */
        Object {
            size: Vec2 {
                x: 5,
                y: 5
            },
            data: PMBEAMICON.to_vec(),
        }, /* gBeamIcon */
        Object {
            size: Vec2 {
                x: 5,
                y: 5
            },
            data: PMWALLICON.to_vec(),
        }, /* gWallIcon */
        Object {
            size: Vec2 {
                x: 3,
                y: 1
            },
            data: PMSHOT.to_vec(),
        }, /* gShot */
        Object {
            size: Vec2 {
                x: 5,
                y: 5
            },
            data: PMEXPLOSION[0].to_vec(),
        }, /* gExplosionA1 */
        Object {
            size: Vec2 {
                x: 5,
                y: 5
            },
            data: PMEXPLOSION[1].to_vec(),
        }, /* gExplosionA2 */
    ]
}

pub fn get_weapons() -> [Object; 3] {
    [
        Object {
            size: Vec2 {
                x: 5,
                y: 5
            },
            data: PMMISSILEICON.to_vec(),
        }, /* Missile */
        Object {
            size: Vec2 {
                x: 84,
                y: 1
            },
            data: [1; 84].to_vec(),
        }, /* Beam */
        Object {
            size: Vec2 {
                x: 1,
                y: 43
            },
            data: [1; 43].to_vec(),
        }, /* Wall */
    ]
}

pub enum Graphics {
    /* Számok, 3*5-ös méretben */
    GNum0, GNum1, GNum2, GNum3, GNum4, GNum5, GNum6, GNum7, GNum8, GNum9,
    /* Menüelemek */
    GSpace, GIntro, GImpact, GScrollMark, GDotEmpty, GDotFull,
    /* Játékosssal kapcsolatos modellek és ikonok */
    GLife, /* Életjel */
    GMissileIcon, /* Rakéta ikonja */
    GBeamIcon,/* SuGár ikonja */
    GWallIcon, /* Fal ikonja */
    GShot, /* Lövés */
    GExplosionA1, GExplosionA2 /* Robbanás animáció 2 lépése */
}

pub const scenery_data: [SceneryData; 6] = [
    SceneryData {
        first_object: 0,
        objects: 0,
        upper: 0,
        inverted_color: true
    }, /* Az 1. szinten nincs táj */
    SceneryData {
        first_object: 0,
        objects: 2,
        upper: 0,
        inverted_color: false
    }, /* 2. szint, 0. dinamikus helytől 2 elemű, 700 pixel széles táj */
    SceneryData {
        first_object: 2,
        objects: 6,
        upper: 0,
        inverted_color: false
    }, /* 3. szint, 2. dinamikus helytől 6 elemű, 750 pixel széles táj */
    SceneryData {
        first_object: 8,
        objects: 6,
        upper: 0,
        inverted_color: false
    }, /* 4. szint, 8. dinamikus helytől 6 elemű, 1000 pixel széles táj */
    SceneryData {
        first_object: 14,
        objects: 4,
        upper: 1,
        inverted_color: true
    }, /* 5. szint, 14. dinamikus helytől 4 elemű, 1250 pixel széles felső táj */
    SceneryData {
        first_object: 14,
        objects: 4,
        upper: 1,
        inverted_color: true
    }, /* 6. szint, az 5. szint elemeiből, 1600 pixel szélesen */
];
