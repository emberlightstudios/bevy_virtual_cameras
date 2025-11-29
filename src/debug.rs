use bevy::prelude::*;
use crate::prelude::VirtualCamera;

#[derive(Component)]
pub struct FrustumGizmo;

pub(crate) fn draw_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &Projection), (With<VirtualCamera>, With<FrustumGizmo>)>,
) {
    for (transform, projection) in &query {
        if let Projection::Perspective(p) = projection {
            draw_perspective_frustum(&mut gizmos, transform, p);
        }
        if let Projection::Orthographic(o) = projection {
            draw_orthographic_frustum(&mut gizmos, transform, o);
        }
    }
}

fn draw_perspective_frustum(
    gizmos: &mut Gizmos,
    camera_transform: &GlobalTransform,
    projection: &PerspectiveProjection,
) {
    let origin = camera_transform.translation();
    let forward = camera_transform.forward();
    let right = camera_transform.right();
    let up = camera_transform.up();

    let near = projection.near;
    let far = projection.far;
    let fov_y = projection.fov;
    let aspect = projection.aspect_ratio;

    // Calculate plane sizes
    let near_height = (fov_y * 0.5).tan() * near;
    let near_width = near_height * aspect;

    let far_height = (fov_y * 0.5).tan() * far;
    let far_width = far_height * aspect;

    // Centers
    let near_center = origin + forward * near;
    let far_center = origin + forward * far;

    // Corners
    let nc = |x: f32, y: f32| near_center + right * (near_width * x) + up * (near_height * y);
    let fc = |x: f32, y: f32| far_center + right * (far_width * x) + up * (far_height * y);

    let near_corners = [
        nc(-1.0, -1.0),
        nc( 1.0, -1.0),
        nc( 1.0,  1.0),
        nc(-1.0,  1.0),
    ];

    let far_corners = [
        fc(-1.0, -1.0),
        fc( 1.0, -1.0),
        fc( 1.0,  1.0),
        fc(-1.0,  1.0),
    ];

    // Draw near rectangle
    for i in 0..4 {
        gizmos.line(near_corners[i], near_corners[(i + 1) % 4], LinearRgba::GREEN);
    }

    // Draw far rectangle
    for i in 0..4 {
        gizmos.line(far_corners[i], far_corners[(i + 1) % 4], LinearRgba::BLUE);
    }

    // Connect near → far
    for i in 0..4 {
        gizmos.line(near_corners[i], far_corners[i], Color::WHITE);
    }
}

fn draw_orthographic_frustum(
    gizmos: &mut Gizmos,
    transform: &GlobalTransform,
    ortho: &OrthographicProjection,
) {
    let origin = transform.translation();
    let forward = transform.forward();
    let right = transform.right();
    let up = transform.up();

    let near = ortho.near;
    let far = ortho.far;

    let half_w = ortho.area.width() * 0.5;
    let half_h = ortho.area.height() * 0.5;

    let near_center = origin + forward * near;
    let far_center = origin + forward * far;

    let nc = |x: f32, y: f32| near_center + right * (half_w * x) + up * (half_h * y);
    let fc = |x: f32, y: f32| far_center + right * (half_w * x) + up * (half_h * y);

    let near_corners = [
        nc(-1., -1.),
        nc( 1., -1.),
        nc( 1.,  1.),
        nc(-1.,  1.),
    ];

    let far_corners = [
        fc(-1., -1.),
        fc( 1., -1.),
        fc( 1.,  1.),
        fc(-1.,  1.),
    ];

    for i in 0..4 {
        gizmos.line(near_corners[i], near_corners[(i + 1) % 4], LinearRgba::GREEN);
        gizmos.line(far_corners[i], far_corners[(i + 1) % 4], LinearRgba::BLUE);
        gizmos.line(near_corners[i], far_corners[i], Color::WHITE);
    }
}
