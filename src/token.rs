use bevy::prelude::*;
use bevy_mod_picking::prelude::Pickable;

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
    fn create(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        pos: Vec2,
        coords: &HexCoord,
    ) -> Entity {
        let entity = commands.spawn((
            Token { coords: *coords },
            SpriteBundle {
                texture: asset_server.load("sprites/shield-sword.png"),
                transform: Transform::from_translation(pos.extend(0.1)),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 55.0, y: 55.0 }),
                    ..Default::default()
                },
                ..Default::default()
            },
            Pickable::default(),
        ));

        entity.id()
    }
}

pub fn on_token_event(
    mut event_reader: EventReader<TokenEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_q: Query<&mut Grid>,
) {
    let mut grid = grid_q.single_mut();

    for e in event_reader.iter() {
        match e {
            TokenEvent::Spawn(t) => {
                let pos = Vec2 {
                    x: t.translation.x,
                    y: t.translation.y,
                };

                let coords = grid.pos_to_hex_coord(&pos);
                let existing = grid.tokens.get(&coords).is_some();

                if !existing {
                    let pos = grid.hex_coord_to_pos(&coords);
                    let entity = Token::create(&mut commands, &asset_server, pos, &coords);
                    grid.tokens.insert(coords, entity);
                } else {
                    log::error!("Token exists in that location");
                }
            }
        };
    }
}
