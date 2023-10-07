use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{grid::Grid, hex::HexCoord, initiative_tracker::TrackerEvent};

#[derive(Debug, Clone)]
pub enum TokenType {
    Party,
    Enemy,
}

#[derive(Event)]
pub struct TurnEvent;

#[derive(Event)]
pub enum TokenEvent {
    BatchSpawn(Vec<(Token, Vec2)>),
}

#[derive(Component, Clone, Debug)]
pub struct Token {
    name: String,
    creature_id: String,
    token_type: TokenType,
    coords: HexCoord,
    color: Color,
}

impl Token {
    pub fn new(
        creature_id: &str,
        name: &str,
        token_type: TokenType,
        coords: &HexCoord,
        color: &Color,
    ) -> Self {
        Self {
            name: name.to_string(),
            creature_id: creature_id.to_string(),
            token_type,
            coords: *coords,
            color: *color,
        }
    }

    fn create(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        token: Token,
        pos: &Vec2,
    ) -> Entity {
        let texture = match token.token_type {
            TokenType::Party => asset_server.load("sprites/shield-sword.png"),
            TokenType::Enemy => asset_server.load("sprites/skull.png"),
        };

        let entity = commands
            .spawn((
                token.clone(),
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(pos.extend(0.1)),
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 55.0, y: 55.0 }),
                        color: token.color,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                On::<Pointer<Drag>>::run(on_token_drag),
                On::<Pointer<DragEnd>>::run(on_token_dropped),
            ))
            .id();

        let text = commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    token.name.clone(),
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

fn on_token_drag(
    event: Listener<Pointer<Drag>>,
    cam_q: Query<&OrthographicProjection, With<Camera>>,
    mut token_q: Query<&mut Transform, With<Token>>,
) {
    let cam_proj = cam_q.single();
    let mut t = token_q.get_mut(event.target).unwrap();
    t.translation += Vec3 {
        x: event.delta.x * cam_proj.scale,
        y: -event.delta.y * cam_proj.scale,
        z: 0.0,
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

pub fn on_tracker_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<TrackerEvent>,
    mut token_q: Query<(Entity, &Token, &mut Sprite)>,
    grid_q: Query<&Grid>,
) {
    let grid = grid_q.single();

    for e in event_reader.iter() {
        match e {
            TrackerEvent::TurnUpdate(c) => {
                for (_, tok, mut sprite) in &mut token_q {
                    if c.id == tok.creature_id {
                        sprite.color = tok.color + vec4(1.0, 1.0, 1.0, 0.0);
                    } else {
                        sprite.color = tok.color;
                    }
                }
            }
            TrackerEvent::CreaturesAdded(to_add) => {
                for c in to_add {
                    let mut name = c.name.split(' ').next().unwrap().to_string();
                    if c.number > 0 {
                        name += &format!(" {}", c.number);
                    }

                    let mut taken_coords = token_q.iter().map(|t| t.1.coords).collect();
                    let coords = find_empty_cells(
                        &taken_coords,
                        grid,
                        grid.pos_to_hex_coord(&Vec2 { x: 0.0, y: 0.0 }),
                    );

                    match coords {
                        Some(coords) => {
                            let player = c.player.is_some();

                            let pos = grid.hex_coord_to_pos(&coords);
                            let tok = Token {
                                name,
                                creature_id: c.id.clone(),
                                token_type: if player {
                                    TokenType::Party
                                } else {
                                    TokenType::Enemy
                                },
                                coords,
                                color: if player {
                                    Color::BLUE
                                } else {
                                    Color::rgb(0.93, 0.13, 0.25)
                                },
                            };
                            Token::create(&mut commands, &asset_server, tok.clone(), &pos);
                            taken_coords.push(coords);
                        }
                        None => log::error!("Token exists in that location"),
                    }
                }
            }
            TrackerEvent::CreaturesRemoved(to_remove) => {
                for c in to_remove.iter().map(|c| c.id.clone()) {
                    for t in token_q.iter() {
                        if t.1.creature_id == c {
                            commands.entity(t.0).despawn_recursive();
                        }
                    }
                }
            }
            TrackerEvent::HealthUpdate(_) => todo!(),
            TrackerEvent::StatusUpdate(_) => todo!(),
        }
    }
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
            TokenEvent::BatchSpawn(toks) => {
                let mut taken_coords = token_q.iter().map(|t| t.coords).collect();
                for (tok, pos) in toks {
                    let coords = find_empty_cells(&taken_coords, grid, grid.pos_to_hex_coord(pos));

                    match coords {
                        Some(coords) => {
                            let pos = grid.hex_coord_to_pos(&coords);
                            Token::create(&mut commands, &asset_server, tok.clone(), &pos);
                            taken_coords.push(coords);
                        }
                        None => log::error!("Token exists in that location"),
                    }
                }
            }
        };
    }
}
