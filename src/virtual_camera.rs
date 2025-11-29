use bevy::{camera::RenderTarget, prelude::*, window::{PrimaryWindow, WindowRef, WindowResized}};
use crate::{blend::CameraBlendDefinition, prelude::Director};


#[derive(Component)]
#[require(Transform, Projection)]
pub struct VirtualCamera {
    pub priority: i32,
    pub blend_in: CameraBlendDefinition,
    pub director: Entity,
}

pub(crate) fn camera_apply_system(
    directors: Query<&Director>,
    vcams: Query<(&Transform, &mut Projection), With<VirtualCamera>>,
    mut cameras: Query<(&mut Transform, &mut Projection), Without<VirtualCamera>>,
) {
    for director in directors.iter() {
        if director.blend.is_some() {
            return;
        }

        let active_vcam = match director.active {
            Some(e) => e,
            None => continue,
        };

        let Ok((vcam_tf, projection)) = vcams.get(active_vcam) else { continue };

        if let Ok((mut cam_tf, mut cam_proj)) = cameras.get_mut(director.camera_entity) {
            *cam_tf = *vcam_tf;
            *cam_proj = projection.clone();
        }
    }
}

fn set_aspect_ratio(proj: &mut Projection, aspect_ratio: f32) {
    match &mut *proj {
        Projection::Perspective(persp) => {
            persp.aspect_ratio = aspect_ratio;
        }
        Projection::Orthographic(ortho) => {
            let center = ortho.area.center();
            let target_aspect = aspect_ratio;
            let height = ortho.area.height();
            let width = height * target_aspect;

            // Build new area around center
            ortho.area.min.x = center.x - width * ortho.viewport_origin.x;
            ortho.area.max.x = center.x + width * (1.0 - ortho.viewport_origin.x);
            ortho.area.min.y = center.y - height * ortho.viewport_origin.y;
            ortho.area.max.y = center.y + height * (1.0 - ortho.viewport_origin.y);

        }
        Projection::Custom(_) => {
            unimplemented!("Not sure how to set aspect ratio for custom projection.")
        }
    }
}

pub(crate) fn on_window_resize(
    mut resize_events: MessageReader<WindowResized>,
    mut vcams: Query<(&VirtualCamera, &mut Projection)>,
    directors: Query<&Director>,
    cameras: Query<&Camera>,
    primary_window: Query<&PrimaryWindow>,
) {
    for event in resize_events.read() {
        let WindowResized { window, width, height } = event;
        let primary_window = primary_window.get(*window);

        for (vcam, mut proj) in vcams.iter_mut() {
            let Ok(director) = directors.get(vcam.director) else { continue };
            let Ok(camera) = cameras.get(director.camera_entity) else { continue };
            let rt = &camera.target;
            let RenderTarget::Window(window_ref) = rt else { continue };

            match window_ref {
                WindowRef::Entity(e) => {
                    if e == window {
                        set_aspect_ratio(&mut *proj, width / height);
                    }
                }
                WindowRef::Primary => {
                    if primary_window.is_ok() {
                        set_aspect_ratio(&mut *proj, width / height);
                    }
                }
            };
        }
    }
}

pub(crate) fn sync_aspect_ratios(
    mut vcams: Query<(Entity, &VirtualCamera, &mut Projection), Changed<Projection>>,
    directors: Query<&Director>,
    cameras: Query<&Camera>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    windows: Query<&Window>,
) {
    if vcams.count() == 0 { return }

    let mut to_update = vec![];

    // Collect updates first
    for (vcam_entity, vcam, _proj) in &vcams {
        let Ok(director) = directors.get(vcam.director) else { continue };
        let Ok(camera) = cameras.get(director.camera_entity) else { continue };
        let rt = &camera.target;
        let RenderTarget::Window(window_ref) = rt else { continue };

        match window_ref {
            WindowRef::Entity(e) => {
                let Ok(window) = windows.get(*e) else { continue };
                let aspect_ratio = window.resolution.physical_width() as f32 / window.resolution.physical_height() as f32;
                to_update.push((vcam_entity, aspect_ratio))
            }
            WindowRef::Primary => {
                let Ok(pw) = primary_window.single() else { continue };
                let Ok(window) = windows.get(pw) else { continue };
                let aspect_ratio = window.resolution.physical_width() as f32 / window.resolution.physical_height() as f32;
                to_update.push((vcam_entity, aspect_ratio))
            }
        };
    }

    // Apply updates separately
    for (vcam_entity, aspect_ratio) in to_update {
        if let Ok((_, _, mut proj)) = vcams.get_mut(vcam_entity) {
            set_aspect_ratio(&mut *proj, aspect_ratio)
        }
    }
}
