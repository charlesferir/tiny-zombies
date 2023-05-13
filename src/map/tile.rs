use bevy::prelude::*;

pub enum SpriteIndex {
    // Player = 98,
    Wall = 40,
    WallTop = 2,
    WallLeft = 13,
    WallRight = 15,
    WallInner = 58,
    WallInnerLeft = 57,
    WallInnerRight = 59,
    WallBottom = 26,
    WallInnerCornerTopLeft = 1,
    WallInnerCornerTopRight = 3,
    WallInnerCornerBottomLeft = 25,
    WallInnerCornerBottomRight = 27,
    WallOuterCornerTopLeft = 4,
    WallOuterCornerTopRight = 5,
    WallOuterCornerBottomLeft = 16,
    WallOuterCornerBottomRight = 17,
    Roof = 0,
    Ground = 48,
    GroundShadow = 50,
}
impl Default for SpriteIndex {
    fn default() -> Self {
        Self::Ground
    }
}

pub fn ctotile(c: char) -> Option<SpriteIndex> {
    Some(match c {
        '─' => SpriteIndex::Wall,
        '▲' => SpriteIndex::WallTop,
        '│' => SpriteIndex::WallLeft,
        '┃' => SpriteIndex::WallRight,
        '║' => SpriteIndex::WallInner,
        '├' => SpriteIndex::WallInnerLeft,
        '┤' => SpriteIndex::WallInnerRight,
        '▼' => SpriteIndex::WallBottom,
        '┌' => SpriteIndex::WallInnerCornerTopLeft,
        '┐' => SpriteIndex::WallInnerCornerTopRight,
        '└' => SpriteIndex::WallInnerCornerBottomLeft,
        '┘' => SpriteIndex::WallInnerCornerBottomRight,
        '┍' => SpriteIndex::WallOuterCornerTopLeft,
        '┑' => SpriteIndex::WallOuterCornerTopRight,
        '┕' => SpriteIndex::WallOuterCornerBottomLeft,
        '┙' => SpriteIndex::WallOuterCornerBottomRight,
        '█' => SpriteIndex::Roof,
        ' ' => SpriteIndex::Ground,
        '░' => SpriteIndex::GroundShadow,
        _ => {
            warn!(
                "Unrecognize tile char '{}', placing default tile instead",
                c
            );
            return None;
        }
    })
}
