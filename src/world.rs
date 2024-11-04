use freecs::world;

world! {
    World {
        components {
            local_transform: LocalTransform => LOCAL_TRANSFORM,
            global_transform: GlobalTransform => GLOBAL_TRANSFORM,
            parent: Parent => PARENT,
            name: Name => NAME,
            color: Color => COLOR,
            camera: Camera => CAMERA,
            active_camera: ActiveCamera => ACTIVE_CAMERA,
            player: Player => PLAYER,
        },
        Resources {
           delta_time: f32,
            keyboard: Keyboard,
            mouse: Mouse,
            viewport_width: u32,
            viewport_height: u32,
        }
    }
}

use components::*;
mod components {
    use serde::{Deserialize, Serialize};

    pub type GlobalTransform = nalgebra_glm::Mat4;
    pub type LocalTransform = Transform;

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct Transform {
        pub translation: nalgebra_glm::Vec3,
        pub rotation: nalgebra_glm::Quat,
        pub scale: nalgebra_glm::Vec3,
    }

    impl Default for Transform {
        fn default() -> Self {
            Self {
                translation: nalgebra_glm::Vec3::new(0.0, 0.0, 0.0),
                rotation: nalgebra_glm::Quat::identity(),
                scale: nalgebra_glm::Vec3::new(1.0, 1.0, 1.0),
            }
        }
    }

    #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Parent(pub super::EntityId);

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    pub struct Name(pub String);

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    pub struct Color(pub nalgebra_glm::Vec4);

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    pub struct Player(pub u8);

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    pub struct ActiveCamera;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Camera {
        pub projection: Projection,
        pub sensitivity: nalgebra_glm::Vec2,
    }

    pub fn projection_matrix(camera: &Camera, aspect_ratio: f32) -> nalgebra_glm::Mat4 {
        match &camera.projection {
            Projection::Perspective(camera) => camera.matrix(aspect_ratio),
            Projection::Orthographic(camera) => camera.matrix(),
        }
    }

    impl Default for Camera {
        fn default() -> Self {
            Self {
                projection: Projection::default(),
                sensitivity: nalgebra_glm::vec2(1.0, 1.0),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Projection {
        Perspective(PerspectiveCamera),
        Orthographic(OrthographicCamera),
    }

    impl Default for Projection {
        fn default() -> Self {
            Self::Perspective(PerspectiveCamera::default())
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PerspectiveCamera {
        pub aspect_ratio: Option<f32>,
        pub y_fov_rad: f32,
        pub z_far: Option<f32>,
        pub z_near: f32,
    }

    impl Default for PerspectiveCamera {
        fn default() -> Self {
            Self {
                aspect_ratio: None,
                y_fov_rad: 90_f32.to_radians(),
                z_far: None,
                z_near: 0.01,
            }
        }
    }

    impl PerspectiveCamera {
        pub fn matrix(&self, viewport_aspect_ratio: f32) -> nalgebra_glm::Mat4 {
            let aspect_ratio = if let Some(aspect_ratio) = self.aspect_ratio {
                aspect_ratio
            } else {
                viewport_aspect_ratio
            };

            if let Some(z_far) = self.z_far {
                nalgebra_glm::perspective_zo(aspect_ratio, self.y_fov_rad, self.z_near, z_far)
            } else {
                nalgebra_glm::infinite_perspective_rh_zo(aspect_ratio, self.y_fov_rad, self.z_near)
            }
        }
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
    pub struct OrthographicCamera {
        pub x_mag: f32,
        pub y_mag: f32,
        pub z_far: f32,
        pub z_near: f32,
    }

    impl OrthographicCamera {
        pub fn matrix(&self) -> nalgebra_glm::Mat4 {
            nalgebra_glm::ortho(
                -self.x_mag,
                self.x_mag,
                -self.y_mag,
                self.y_mag,
                self.z_near,
                self.z_far,
            )
        }
    }
}

use resources::*;
mod resources {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    pub struct Keyboard {
        pub keystates:
            std::collections::HashMap<winit::keyboard::KeyCode, winit::event::ElementState>,
    }

    #[allow(dead_code)]
    pub fn is_key_pressed(keyboard: &Keyboard, keycode: winit::keyboard::KeyCode) -> bool {
        keyboard.keystates.contains_key(&keycode)
            && keyboard.keystates[&keycode] == winit::event::ElementState::Pressed
    }

    bitflags::bitflags! {
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        pub struct MouseButtons: u8 {
            const LEFT_CLICKED = 0b0000_0001;
            const MIDDLE_CLICKED = 0b0000_0010;
            const RIGHT_CLICKED = 0b0000_0100;
            const MOVED = 0b0000_1000;
            const SCROLLED = 0b0001_0000;
        }
    }

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    pub struct Mouse {
        pub buttons: MouseButtons,
        pub position: nalgebra_glm::Vec2,
        pub position_delta: nalgebra_glm::Vec2,
        pub offset_from_center: nalgebra_glm::Vec2,
        pub wheel_delta: nalgebra_glm::Vec2,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Image {
        pub pixels: Vec<u8>,
        pub format: ImageFormat,
        pub width: u32,
        pub height: u32,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
    pub enum ImageFormat {
        R8,
        R8G8,
        R8G8B8,
        R8G8B8A8,
        B8G8R8,
        B8G8R8A8,
        R16,
        R16G16,
        R16G16B16,
        R16G16B16A16,
        R16F,
        R16G16F,
        R16G16B16F,
        R16G16B16A16F,
        R32,
        R32G32,
        R32G32B32,
        R32G32B32A32,
        R32F,
        R32G32F,
        R32G32B32F,
        R32G32B32A32F,
    }

    #[derive(Default, Clone, Debug, Serialize, Deserialize)]
    pub struct Sampler {
        pub min_filter: MinFilter,
        pub mag_filter: MagFilter,
        pub wrap_s: WrappingMode,
        pub wrap_t: WrappingMode,
    }

    #[derive(Default, Clone, Debug, Serialize, Deserialize)]
    pub enum MagFilter {
        Nearest = 1,
        #[default]
        Linear,
    }

    #[derive(Default, Clone, Debug, Serialize, Deserialize)]
    pub enum MinFilter {
        Nearest = 1,
        #[default]
        Linear,
        NearestMipmapNearest,
        LinearMipmapNearest,
        NearestMipmapLinear,
        LinearMipmapLinear,
    }

    #[derive(Default, Clone, Debug, Serialize, Deserialize)]
    pub enum WrappingMode {
        ClampToEdge,
        MirroredRepeat,
        #[default]
        Repeat,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Texture {
        pub image_index: usize,
        pub sampler_index: Option<usize>,
    }
}

use systems::*;
mod systems {
    use super::*;
    use freecs::has_components;
    use rayon::prelude::*;

    pub fn run_systems(world: &mut World) {
        let delta_time = world.resources.delta_time;
        world.tables.par_iter_mut().for_each(|table| {
            // if has_components!(table, POSITION) {
            //     update_positions_system(&mut table.positions, &table.velocities, delta_time);
            // }
        });
    }

    // // The system itself can also access components in parallel
    // #[inline]
    // pub fn update_positions_system(positions: &mut [Position], velocities: &[Velocity], dt: f32) {
    //     positions
    //         .par_iter_mut()
    //         .zip(velocities.par_iter())
    //         .for_each(|(pos, vel)| {
    //             pos.x += vel.x * dt;
    //             pos.y += vel.y * dt;
    //         });
    // }
}

use queries::*;
mod queries {
    use super::*;

    #[derive(Default, Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
    pub struct CameraMatrices {
        pub camera_position: nalgebra_glm::Vec3,
        pub projection: nalgebra_glm::Mat4,
        pub view: nalgebra_glm::Mat4,
    }

    pub fn query_active_camera_matrices(
        world: &World,
        resources: &Resources,
    ) -> Option<(EntityId, CameraMatrices)> {
        let Some(camera_entity) =
            query_first_entity(&world, ACTIVE_CAMERA | CAMERA | LOCAL_TRANSFORM)
        else {
            return None;
        };

        let (Some(camera), Some(local_transform), Some(global_transform)) = (
            get_component::<Camera>(world, camera_entity, CAMERA),
            get_component::<LocalTransform>(world, camera_entity, LOCAL_TRANSFORM),
            get_component::<GlobalTransform>(world, camera_entity, GLOBAL_TRANSFORM),
        ) else {
            return None;
        };

        let normalized_rotation = local_transform.rotation.normalize();
        let camera_translation = global_transform.column(3).xyz();
        let target = camera_translation
            + nalgebra_glm::quat_rotate_vec3(&normalized_rotation, &(-nalgebra_glm::Vec3::z()));
        let up = nalgebra_glm::quat_rotate_vec3(&normalized_rotation, &nalgebra_glm::Vec3::y());
        let aspect_ratio =
            resources.viewport_width as f32 / resources.viewport_height.max(1) as f32;

        Some((
            camera_entity,
            CameraMatrices {
                camera_position: camera_translation,
                projection: projection_matrix(camera, aspect_ratio),
                view: nalgebra_glm::look_at(&camera_translation, &target, &up),
            },
        ))
    }
}
