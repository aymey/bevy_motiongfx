use bevy::{
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo, peniko},
};

use crate::{fill_style::FillStyle, stroke_style::StrokeStyle, vello_vector::VelloVector};

use super::_VelloVector;

#[derive(Bundle, Clone, Default)]
pub struct VelloRectBundle {
    pub rect: VelloRect,
    pub fill: FillStyle,
    pub stroke: StrokeStyle,
    pub scene_bundle: VelloSceneBundle,
}

#[derive(Component, Clone, Default)]
pub struct VelloRect {
    /// Coordinates of the rectangle.
    pub rect: kurbo::Rect,
    /// Radius of all four corners.
    pub radii: kurbo::RoundedRectRadii,
}

impl VelloRect {
    pub fn new(rect: kurbo::Rect, radii: kurbo::RoundedRectRadii) -> Self {
        let radii: kurbo::RoundedRectRadii = radii.into();

        Self {
            rect,
            radii,
            ..default()
        }
    }

    pub fn percentage_anchor(size: DVec2, radius: DVec4, percentage: DVec2) -> Self {
        Self::new(
            kurbo::Rect::new(
                -size.x * percentage.x,
                -size.y * percentage.y,
                size.x * (1.0 - percentage.x),
                size.y * (1.0 - percentage.y),
            ),
            kurbo::RoundedRectRadii::new(radius.x, radius.y, radius.z, radius.w),
        )
    }

    #[inline]
    pub fn anchor_center(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 0.5))
    }

    #[inline]
    pub fn anchor_left(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(1.0, 0.5))
    }

    #[inline]
    pub fn anchor_right(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.0, 0.5))
    }

    #[inline]
    pub fn anchor_bottom(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 0.0))
    }

    #[inline]
    pub fn anchor_top(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 1.0))
    }
}

impl VelloVector for VelloRect {
    #[inline]
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::RoundedRect::from_rect(self.rect, self.radii)
    }
}

#[derive(Default)]
pub enum RectAnchor {
    #[default]
    Center,
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Default)]
pub struct _VelloRect {
    pub size: DVec2,
    pub anchor: RectAnchor,
    // Fill
    pub fill_brush: peniko::Brush,
    pub fill_transform: Option<kurbo::Affine>,
    // Stroke
    pub stroke: Option<kurbo::Stroke>,
    pub stroke_brush: peniko::Brush,
    pub stroke_transform: Option<kurbo::Affine>,
}

impl _VelloRect {
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size.x = width;
        self.size.y = height;

        self
    }

    pub fn with_anchor(mut self, anchor: RectAnchor) -> Self {
        self.anchor = anchor;

        self
    }

    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_brush = peniko::Brush::Solid(peniko::Color::rgba(
            color.r() as f64,
            color.g() as f64,
            color.b() as f64,
            color.a() as f64,
        ));

        self
    }

    pub fn with_stroke(mut self, stroke: kurbo::Stroke) -> Self {
        self.stroke = Some(stroke);

        self
    }

    pub fn with_stroke_color(mut self, color: Color) -> Self {
        self.stroke_brush = peniko::Brush::Solid(peniko::Color::rgba(
            color.r() as f64,
            color.g() as f64,
            color.b() as f64,
            color.a() as f64,
        ));

        self
    }

    // pub fn build(
    //     self,
    //     commands: &mut Commands,
    //     scenes: &mut Assets<VelloScene>,
    // ) -> VelloRectBundleMotion {
    //     let rect = match self.anchor {
    //         RectAnchor::Center => VelloRect::anchor_center(self.size, self.radii),
    //         RectAnchor::Left => VelloRect::anchor_left(self.size, self.radii),
    //         RectAnchor::Right => VelloRect::anchor_right(self.size, self.radii),
    //         RectAnchor::Bottom => VelloRect::anchor_bottom(self.size, self.radii),
    //         RectAnchor::Top => VelloRect::anchor_top(self.size, self.radii),
    //     };

    //     let rect_bundle = VelloRectBundle {
    //         rect,
    //         fill: FillStyle::from_brush(self.fill_brush),
    //         stroke: StrokeStyle::from_brush(self.stroke_brush),
    //         scene_bundle: VelloSceneBundle {
    //             scene: scenes.add(VelloScene::default()),
    //             ..default()
    //         },
    //     };

    //     let rect_id = commands.spawn(rect_bundle.clone()).id();

    //     VelloRectBundleMotion::new(rect_id, rect_bundle)
    // }
}

impl _VelloVector for _VelloRect {
    fn build_scene(&self) -> vello::Scene {
        let mut scene = vello::Scene::new();

        let rect = kurbo::Rect::new(0.0, 0.0, self.size.x, self.size.y);

        scene.fill(
            peniko::Fill::NonZero,
            kurbo::Affine::default(),
            &self.fill_brush,
            self.fill_transform,
            &rect,
        );

        if let Some(stroke) = &self.stroke {
            scene.stroke(
                stroke,
                kurbo::Affine::default(),
                &self.stroke_brush,
                self.stroke_transform,
                &rect,
            );
        }

        scene
    }
}
