use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{grid::Grid, hex::HexCoord};

pub enum TokenType {
    Party,
    Enemy,
}

#[derive(Event)]
pub enum TokenEvent {
    Spawn((String, TokenType, Transform)),
    BatchSpawn(Vec<(String, TokenType, Transform)>),
}

#[derive(Component)]
pub struct Token {
    name: String,
    coords: HexCoord,
}

impl Token {
    fn create(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        token_type: &TokenType,
        name: String,
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

        let entity = commands
            .spawn((
                Token {
                    coords: *coords,
                    name: name.clone(),
                },
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
            ))
            .id();

        let text = commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    name,
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                ),
                transform: Transform::from_translation(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.1,
                }),
                ..Default::default()
            })
            .id();

        commands.entity(entity).add_child(text);

        entity
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

fn find_empty_cells(
    taken_coords: &Vec<HexCoord>,
    grid: &Grid,
    start: HexCoord,
) -> Option<HexCoord> {
    if !taken_coords.iter().any(|t| t == &start) {
        return Some(start);
    }

    for n in grid.get_neighbours(&start).iter() {
        if !taken_coords.iter().any(|t| t == n) {
            return Some(*n);
        }
    }

    for n in grid.get_neighbours(&start).iter() {
        if let Some(n) = find_empty_cells(taken_coords, grid, n.to_owned()) {
            return Some(n);
        }
    }

    None
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
            TokenEvent::Spawn((name, token_type, t)) => {
                let pos = Vec2 {
                    x: t.translation.x,
                    y: t.translation.y,
                };

                let taken_coords = token_q.iter().map(|t| t.coords).collect();

                let coords = find_empty_cells(&taken_coords, grid, grid.pos_to_hex_coord(&pos));

                match coords {
                    Some(coords) => {
                        let pos = grid.hex_coord_to_pos(&coords);
                        Token::create(
                            &mut commands,
                            &asset_server,
                            token_type,
                            name.to_owned(),
                            pos,
                            &coords,
                        );
                    }
                    None => log::error!("Token exists in that location"),
                }
            }
            TokenEvent::BatchSpawn(toks) => {
                let mut taken_coords = token_q.iter().map(|t| t.coords).collect();
                for (name, token_type, t) in toks {
                    let pos = Vec2 {
                        x: t.translation.x,
                        y: t.translation.y,
                    };

                    let coords = find_empty_cells(&taken_coords, grid, grid.pos_to_hex_coord(&pos));

                    match coords {
                        Some(coords) => {
                            let pos = grid.hex_coord_to_pos(&coords);
                            Token::create(
                                &mut commands,
                                &asset_server,
                                token_type,
                                name.to_owned(),
                                pos,
                                &coords,
                            );
                            taken_coords.push(coords);
                        }
                        None => log::error!("Token exists in that location"),
                    }
                }
            }
        };
    }
}
