#![doc = include_str!("../../../readme.md")]

use bevy::prelude::*;

mod collider_to_trimesh;
#[cfg(feature = "editor_integration")]
pub mod editor_integration;
pub use collider_to_trimesh::ToTriMesh;

pub use recast;

/// Everything you need to get started with the NavMesh plugin.
pub mod prelude {
    pub use crate::NavMeshPlugin;
}

/// The plugin of the crate.
#[non_exhaustive]
#[derive(Default)]
pub struct NavMeshPlugin;

impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor_integration")]
        app.add_plugins(editor_integration::plugin);
    }
}
