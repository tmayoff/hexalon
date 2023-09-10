use bevy::prelude::*;

use crate::cell::HexCoord;

#[derive(Event)]
pub enum TokenEvent {
    Spawn,
}

#[derive(Component)]
pub struct Token {
    pos: HexCoord,
}

impl Token {
    fn new(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Self {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("sprites/shield-sword.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 55.0, y: 55.0 }),
                ..Default::default()
            },
            ..Default::default()
        });

        Self {
            pos: HexCoord { q: 0, r: 0 },
        }
    }
}

pub fn on_token_event(
    mut event_reader: EventReader<TokenEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for e in event_reader.iter() {
        match e {
            TokenEvent::Spawn => Token::new(&mut commands, &asset_server),
        };
    }
}
