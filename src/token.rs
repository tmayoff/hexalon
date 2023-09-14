use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{grid::Grid, hex::HexCoord};

pub enum TokenType {
    Party,
    Enemy,
}

#[derive(Event)]
pub enum TokenEvent {
    Spawn((TokenType, Transform)),
}

#[derive(Component)]
pub struct Token {
    coords: HexCoord,
}

impl Token {
    fn create(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        token_type: &TokenType,
        pos: Vec2,
        coords: &HexCoord,
    ) -> Entity {
        let texture;
        let color;

        match token_type {
            TokenType::Party => {
                texture = asset_server.load("sprites/shield-sword.png");
                color = Color::BLUE;
            }
            TokenType::Enemy => {
                texture = asset_server.load("sprites/skull.png");
                color = Color::rgb(0.93, 0.13, 0.25);
            }
        }

        let entity = commands.spawn((
            Token { coords: *coords },
            SpriteBundle {
                texture,
                transform: Transform::from_translation(pos.extend(0.1)),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 55.0, y: 55.0 }),
                    color,
                    ..Default::default()
                },
                ..Default::default()
            },
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation += Vec2 {
                    x: drag.delta.x,
                    y: -drag.delta.y,
                }
                .extend(0.0);
            }),
            On::<Pointer<DragEnd>>::run(on_token_dropped),
        ));

        entity.id()
    }

    fn token_at(token_q: &Query<&Token>, coords: &HexCoord) -> bool {
        for token in token_q.iter() {
            if token.coords == *coords {
                return true;
            }
        }

        false
    }
}

fn on_token_dropped(
    event: Listener<Pointer<DragEnd>>,
    mut token_q: Query<(&mut Token, &mut Transform)>,
    grid_q: Query<&Grid>,
) {
    let grid = grid_q.single();

    let (mut token, mut t) = token_q.get_mut(event.target).unwrap();

    let hex_coords = grid.pos_to_hex_coord(&Vec2 {
        x: t.translation.x,
        y: t.translation.y,
    });
    let rounded_pos = grid.hex_coord_to_pos(&hex_coords);

    t.translation = rounded_pos.extend(0.1);
    token.coords = hex_coords;
}

pub fn on_token_event(
    mut event_reader: EventReader<TokenEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_q: Query<&Grid>,
    token_q: Query<&Token>,
) {
    let grid = grid_q.single();

    for e in event_reader.iter() {
        match e {
            TokenEvent::Spawn((token_type, t)) => {
                let pos = Vec2 {
                    x: t.translation.x,
                    y: t.translation.y,
                };

                let coords = grid.pos_to_hex_coord(&pos);
                let existing = Token::token_at(&token_q, &coords);

                if !existing {
                    let pos = grid.hex_coord_to_pos(&coords);
                    Token::create(&mut commands, &asset_server, token_type, pos, &coords);
                } else {
                    log::error!("Token exists in that location");
                }
            }
        };
    }
}
