use bevy::prelude::*;

use crate::{blend::CameraBlendState, virtual_camera::VirtualCamera};

#[derive(Component, Clone)]
pub struct Director {
    pub active: Option<Entity>,            // current virtual camera
    pub(crate) blend: Option<CameraBlendState>,   // current blend (if between two)
    pub(crate) camera_entity: Entity,
}

impl Director {
    pub fn new(camera_entity: Entity) -> Self {
        Self { camera_entity, active: None, blend: None }
    }
}

#[derive(Message, Copy, Clone, Debug)]
pub struct StartedCameraBlend {
    pub from: Entity,
    pub to: Entity,
}

#[derive(Message, Copy, Clone, Debug)]
pub struct FinishedCameraBlend {
    pub to: Entity,
}

pub(crate) fn update_active_camera_system(
    mut directors: Query<(Entity, &mut Director)>,
    vcams: Query<(Entity, &VirtualCamera)>,
    updates: Query<Entity, Changed<VirtualCamera>>,
    mut message_writer: MessageWriter<StartedCameraBlend>,
) {
    if updates.count() == 0 { return }

    for (director_entity, mut director) in directors.iter_mut() {
        let mut max_priority = i32::MIN;
        let mut active_cam = Entity::PLACEHOLDER;
        for (vcam_entity, vcam) in vcams {
            if vcam.director != director_entity { continue }
            if vcam.priority > max_priority {
                max_priority = vcam.priority;
                active_cam = vcam_entity;
            }
        }

        match director.active {
            Some(current) if current == active_cam => {
            }
            Some(current) => {
                // Start blending from current -> new.
                if let Some(previous) = director.active {
                    message_writer.write(StartedCameraBlend { from: previous, to: active_cam });
                }
                let (_, new_vcam) = vcams.get(active_cam).unwrap();
                director.blend = Some(new_vcam.blend_in.create(current, active_cam));
                director.active = Some(active_cam);
            }
            None => {
                // No current active; just set it directly.
                director.active = Some(active_cam);
                director.blend = None;
            }
        }
    }
}
