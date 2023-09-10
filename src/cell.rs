use std::ops;

use bevy::{math::vec4, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::egui::lerp;
use bevy_mod_picking::{prelude::*, PickableBundle};

lazy_static! {
    pub static ref HEX_COLOR: Color = Color::Rgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0
    };
}

#[derive(Event)]
pub enum CellEvent {
    Pressed(Entity),
    Released(Entity),
    Over(Entity),
}

#[derive(PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

pub struct FractionalHexCoord {
    pub q: f32,
    pub r: f32,
}

impl HexCoord {
    pub fn lerp(a: &HexCoord, b: &HexCoord, t: f32) -> HexCoord {
        Self::round(FractionalHexCoord {
            q: lerp(a.q as f32..=b.q as f32, t),
            r: lerp(a.r as f32..=b.r as f32, t),
        })
    }

    fn round(coord: FractionalHexCoord) -> HexCoord {
        let mut q = (coord.q).round();
        let mut r = (coord.r).round();
        let s = (-coord.q - coord.r).round();

        let q_diff = (q - coord.q).abs();
        let r_diff = (r - coord.r).abs();
        let s_diff = (s - (-coord.q - coord.r)).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s
        } else {
            // s = -q - r
        }

        HexCoord {
            q: q as i32,
            r: r as i32,
        }
    }
}

impl ops::Sub<&HexCoord> for &HexCoord {
    type Output = HexCoord;

    fn sub(self, rhs: &HexCoord) -> Self::Output {
        HexCoord {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

#[derive(Component)]
pub struct Cell {
    _size: f32,
    pub pos: HexCoord,
    pub color: Color,
}

impl Cell {
    pub fn create(
        world_pos: Vec2,
        size: f32,
        pos: HexCoord,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Entity {
        let c = Cell {
            _size: size,
            pos,
            color: *HEX_COLOR,
        };

        let mesh = MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::RegularPolygon::new(size, 6)))
                .into(),
            material: materials.add((*HEX_COLOR).into()),
            transform: Transform::default().with_translation(world_pos.extend(0.1)),
            ..Default::default()
        };

        commands
            .spawn((
                mesh,
                c,
                RaycastPickTarget::default(),
                PickableBundle::default(),
                On::<Pointer<Over>>::run(on_hover_enter),
                On::<Pointer<Out>>::run(on_hover_out),
                On::<Pointer<Down>>::run(on_pressed),
                On::<Pointer<Up>>::run(on_released),
            ))
            .id()
    }
}

fn on_hover_enter(
    event: Listener<Pointer<Over>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_q: Query<(&Cell, &Handle<ColorMaterial>)>,
    mut cell_event: EventWriter<CellEvent>,
) {
    // Update the color of the cell
    if let Ok((cell, mat)) = cell_q.get(event.target) {
        let material = materials.get_mut(mat).unwrap();
        material.color = cell.color + vec4(-0.2, -0.2, -0.2, 0.0);
    }

    cell_event.send(CellEvent::Over(event.target));
}

fn on_hover_out(
    event: Listener<Pointer<Out>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_q: Query<(&Cell, &Handle<ColorMaterial>)>,
) {
    // Revert the color of the cell
    if let Ok((cell, mat)) = cell_q.get(event.target) {
        let material = materials.get_mut(mat).unwrap();
        material.color = cell.color;
    }
}

fn on_pressed(event: Listener<Pointer<Down>>, mut cell_event: EventWriter<CellEvent>) {
    if event.button != PointerButton::Primary {
        return;
    }

    cell_event.send(CellEvent::Pressed(event.target));
}

fn on_released(event: Listener<Pointer<Up>>, mut cell_event: EventWriter<CellEvent>) {
    if event.button != PointerButton::Primary {
        return;
    }

    cell_event.send(CellEvent::Released(event.target));
}
