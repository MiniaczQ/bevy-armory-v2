use bevy::{math::Vec3A, prelude::*, ui::UiSystem, window::PrimaryWindow};

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        window_clamp.after(TransformSystem::TransformPropagate),
    );
    app.add_systems(
        PostUpdate,
        center_position
            .after(UiSystem::Layout)
            .before(TransformSystem::TransformPropagate),
    );
}

/// UI nodes with this component will position their center at the specified position.
#[derive(Component)]
pub struct CenterPosition {
    pub position: Vec2,
}

/// UI nodes with this component will be moved to fit within the window size.
#[derive(Component)]
pub struct WindowClamp;

pub fn center_position(mut nodes: Query<(&mut Transform, &CenterPosition)>) {
    for (mut transform, center) in &mut nodes {
        transform.translation.x = center.position.x;
        transform.translation.y = center.position.y;
    }
}

pub fn window_clamp(
    mut nodes: Query<(&mut GlobalTransform, &ComputedNode), With<WindowClamp>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let size = window.size();
    for (mut transform, node) in &mut nodes {
        let mut affine = transform.affine();
        let half_size = node.size() / 2.0;
        let min = (affine.translation.xy() - half_size).min(Vec2::ZERO);
        let max = size - (affine.translation.xy() + half_size).max(size);
        affine.translation += Vec3A::from((min + max).extend(0.0));
        *transform = GlobalTransform::from(affine);
    }
}
