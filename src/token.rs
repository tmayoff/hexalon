use bevy::prelude::*;

use crate::{grid::Grid, hex::HexCoord};

#[derive(Event)]
pub enum TokenEvent {
    Spawn(Transform),
}

#[derive(Component)]
pub struct Token {
    coords: HexCoord,
}

impl Token {
    fn new(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        pos: Vec2,
        coords: HexCoord,
    ) -> Self {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("sprites/shield-sword.png"),
            transform: Transform::from_translation(pos.extend(0.1)),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 55.0, y: 55.0 }),
                ..Default::default()
            },
            ..Default::default()
        });

        Self { coords }
    }
}

pub fn on_token_event(
    mut event_reader: EventReader<TokenEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_q: Query<&Grid>,
) {
    let grid = grid_q.single();

    for e in event_reader.iter() {
        match e {
            TokenEvent::Spawn(t) => {
                let pos = Vec2 {
                    x: t.translation.x,
                    y: t.translation.y,
                };

                let coords = grid.pos_to_hex_coord(&pos);
                let pos = grid.hex_coord_to_pos(&coords);
                Token::new(&mut commands, &asset_server, pos, coords);
            }
        };
    }
}
