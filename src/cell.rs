use crate::grid::{self, *};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{prelude::*, PickableBundle};

lazy_static! {
    static ref HEX_COLOR: Color = Color::Rgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0
    };
}

#[derive(Component)]
pub struct Cell {
    _size: f32,

    pub color_managed: bool,
}

impl Cell {
    pub fn create(
        position: Vec2,
        size: f32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Entity {
        let c = Cell {
            _size: size,
            color_managed: false,
        };

        let mesh = MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::RegularPolygon::new(size, 6)))
                .into(),
            material: materials.add((*HEX_COLOR).into()),
            transform: Transform::default().with_translation(position.extend(0.1)),
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
                On::<Pointer<Down>>::run(grid::grid_selection_down),
                On::<Pointer<Up>>::run(grid::grid_selection_up),
            ))
            .id()
    }
}

fn on_hover_enter(
    event: Listener<Pointer<Over>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_q: Query<(&Cell, &Handle<ColorMaterial>)>,
) {
    if let Ok((cell, mat)) = cell_q.get(event.target) {
        if !cell.color_managed {
            let material = materials.get_mut(mat).unwrap();
            material.color = *HEX_HOVER_COLOR;
        }
    }
}

fn on_hover_out(
    event: Listener<Pointer<Out>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_q: Query<(&Cell, &Handle<ColorMaterial>)>,
) {
    if let Ok((cell, mat)) = cell_q.get(event.target) {
        if !cell.color_managed {
            let material = materials.get_mut(mat).unwrap();
            material.color = *HEX_COLOR;
        }
    }
}
