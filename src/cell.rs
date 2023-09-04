use crate::grid::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{prelude::*, PickableBundle};

#[derive(Component)]
pub struct Cell {
    size: f32,
}

impl Cell {
    pub fn create(
        position: Vec2,
        size: f32,
        color: Color,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Entity {
        let c = Cell { size };

        let mesh = MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::RegularPolygon::new(size, 6)))
                .into(),
            material: materials.add(color.into()),
            transform: Transform::default().with_translation(position.extend(0.1)),
            ..Default::default()
        };

        commands
            .spawn((
                mesh,
                c,
                RaycastPickTarget::default(),
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ))
            .id()
    }
}

pub const HIGHLIGHT_TINT: Highlight<ColorMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| ColorMaterial {
        color: *HEX_HOVER_COLOR,
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| ColorMaterial {
        color: *HEX_PRESSED_COLOR,
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| ColorMaterial {
        ..matl.to_owned()
    })),
};
