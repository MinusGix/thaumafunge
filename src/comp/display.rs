use bevy::{
    prelude::{Bundle, Color, Handle},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};

#[derive(Bundle)]
pub struct Renderable {
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
}
impl Renderable {
    pub fn from_index(atlas: Handle<TextureAtlas>, index: usize, color: Color) -> Renderable {
        Renderable {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color,
                    index,
                    flip_x: false,
                    flip_y: false,
                    custom_size: None,
                },
                texture_atlas: atlas,
                ..Default::default()
            },
        }
    }
}
