use bevy::prelude::*;


#[derive(Clone, Debug)]
pub struct CameraState {
    pub transform: Transform,
    pub projection: Projection,
}


impl CameraState {
    /// Interpolates between two camera states.
    pub fn interpolate(from: Self, to: Self, t: f32) -> Self {
        // Interpolate transform
        let transform = Transform {
            translation: from.transform.translation.lerp(to.transform.translation, t),
            rotation: from.transform.rotation.slerp(to.transform.rotation, t),
            scale: from.transform.scale.lerp(to.transform.scale, t),
        };

        // Interpolate projection
        let projection = match (&from.projection, &to.projection) {
            (Projection::Perspective(a), Projection::Perspective(b)) => {
                Projection::Perspective(bevy::prelude::PerspectiveProjection {
                    fov: a.fov + (b.fov - a.fov) * t,
                    near: a.near + (b.near - a.near) * t,
                    far: a.far + (b.far - a.far) * t,
                    aspect_ratio: a.aspect_ratio + (b.aspect_ratio - a.aspect_ratio) * t,
                })
            }
            (Projection::Orthographic(a), Projection::Orthographic(b)) => {
                Projection::Orthographic(OrthographicProjection {
                    scale: a.scale + (b.scale - a.scale) * t,
                    near: a.near + (b.near - a.near) * t,
                    far: a.far + (b.far - a.far) * t,
                    viewport_origin: a.viewport_origin.lerp(b.viewport_origin, t),
                    scaling_mode: a.scaling_mode.clone(),
                    area: a.area,
                })
            }
            (Projection::Custom(_), _) | (_, Projection::Custom(_)) => {
                // Fallback: pick the target projection
                to.projection.clone()
            }
            _ => to.projection.clone(),
        };

        Self { transform, projection }
    }
}
