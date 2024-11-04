use freecs::world;

world! {
    World {
        local_transforms: LocalTransform => LOCAL_TRANSFORM,
        global_transforms: GlobalTransform => GLOBAL_TRANSFORM,
        parents: Parent => PARENT,
        names: Name => NAME,
        colors: Color => COLOR,
    }
}

pub type GlobalTransform = nalgebra_glm::Mat4;
pub type LocalTransform = Transform;

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Default, Debug, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Parent(pub EntityId);

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Name(pub String);

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Color(pub nalgebra_glm::Vec4);
