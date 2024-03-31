use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{prelude::*, vello::kurbo};

use crate::{stroke_style::StrokeStyle, vello_vector::VelloVector};

#[derive(Bundle, Default, Clone)]
pub struct VelloLineBundle {
    pub line: VelloLine,
    pub stroke: StrokeStyle,
    pub scene_bundle: VelloSceneBundle,
}

#[derive(VelloVector, Component, Clone)]
pub struct VelloLine {
    #[shape]
    pub line: kurbo::Line,
}

impl VelloLine {
    #[inline]
    pub fn new(line: kurbo::Line) -> Self {
        Self { line, ..default() }
    }

    pub fn from_points(p0: DVec2, p1: DVec2) -> Self {
        Self {
            line: kurbo::Line::new(kurbo::Point::new(p0.x, p0.y), kurbo::Point::new(p1.x, p1.y)),
            ..default()
        }
    }

    pub fn origin_to(to: DVec2) -> Self {
        Self {
            line: kurbo::Line::new(kurbo::Point::default(), kurbo::Point::new(to.x, to.y)),
            ..default()
        }
    }
}

impl Default for VelloLine {
    fn default() -> Self {
        Self {
            line: kurbo::Line::new(kurbo::Point::default(), kurbo::Point::default()),
        }
    }
}
