use crate::grid::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
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

pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

#[derive(Component)]
pub struct Cell {
    _size: f32,
    pub pos: HexCoord,
    pub color_managed: bool,
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
            color_managed: false,
            pos,
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
        if !cell.color_managed {
            let material = materials.get_mut(mat).unwrap();
            material.color = *HEX_HOVER_COLOR;
        }
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
        if !cell.color_managed {
            let material = materials.get_mut(mat).unwrap();
            material.color = *HEX_COLOR;
        }
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
