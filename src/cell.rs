use bevy::{math::vec4, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{prelude::*, PickableBundle};

use crate::hex::HexCoord;

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
    // Distance from the starting cell
    Released(Vec2),
    Over(Entity),
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
        mut commands: &mut Commands,
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
                On::<Pointer<DragStart>>::run(on_drag_start),
                On::<Pointer<DragEnd>>::run(on_drag_end),
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

fn on_drag_start(event: Listener<Pointer<DragStart>>, mut cell_event: EventWriter<CellEvent>) {
    if event.button != PointerButton::Primary {
        return;
    }

    cell_event.send(CellEvent::Pressed(event.target));
}

fn on_drag_end(
    event: Listener<Pointer<DragEnd>>,
    mut cell_event: EventWriter<CellEvent>,
    cam_q: Query<&OrthographicProjection, With<Camera>>,
) {
    if event.button != PointerButton::Primary {
        return;
    }

    let cam = cam_q.single();
    let dst = Vec2 {
        x: event.distance.x,
        y: -event.distance.y,
    } * cam.scale;
    cell_event.send(CellEvent::Released(dst));
}
