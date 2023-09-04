use crate::grid::{self, *};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{prelude::*, PickableBundle};

#[derive(Component)]
pub struct Cell {
    _size: f32,
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
        let c = Cell { _size: size };

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
                On::<Pointer<Down>>::run(grid::grid_selection_down),
                On::<Pointer<Up>>::run(grid::grid_selection_up),
            ))
            .id()
    }
}

#[derive(Event)]
pub struct DragEvent(Entity);
impl From<ListenerInput<Pointer<Drag>>> for DragEvent {
    fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
        DragEvent(event.target)
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
