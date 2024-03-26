//! [`VelloBezPathMotion`]: crate::vello_motion::bezpath_motion::VelloBezPathMotion

use bevy_ecs::prelude::*;
use bevy_utils::prelude::*;
use bevy_vello_renderer::{prelude::*, vello::kurbo};

use crate::{fill_style::FillStyle, stroke_style::StrokeStyle, vello_vector::VelloVector};

/// Vello Bézier path bundle.
#[derive(Bundle, Clone, Default)]
pub struct VelloBezPathBundle {
    pub path: VelloBezPath,
    pub fill: FillStyle,
    pub stroke: StrokeStyle,
    pub scene_bundle: VelloSceneBundle,
}

/// Vello Bézier path component.
#[derive(VelloVector, Component, Default, Clone)]
pub struct VelloBezPath {
    /// The Bézier path that [`VelloBezPathMotion`] reference to when performing motions.
    pub origin_path: kurbo::BezPath,
    #[shape]
    pub path: kurbo::BezPath,
}

impl VelloBezPath {
    pub fn new(path: kurbo::BezPath) -> Self {
        Self {
            origin_path: path.clone(),
            path,
            ..default()
        }
    }
}
