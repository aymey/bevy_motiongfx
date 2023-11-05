use crate::ease::{quad, EaseFn};
use bevy_ecs::prelude::*;

pub type InterpFn<C, T, R> =
    fn(component: &mut C, begin: &T, end: &T, t: f32, resource: &mut ResMut<R>);

/// Basic data structure to describe an animation action.
#[derive(Component, Clone, Copy)]
pub struct Action<C, T, R>
where
    C: Component,
    T: Send + Sync + 'static,
    R: Resource,
{
    /// Target `Entity` for `Component` manipulation.
    pub(crate) target_id: Entity,
    /// Initial state of the animation.
    pub(crate) begin: T,
    /// Final state of the animation.
    pub(crate) end: T,
    /// Interpolation function to be used for animation.
    pub(crate) interp_fn: InterpFn<C, T, R>,
}

impl<C, T, R> Action<C, T, R>
where
    C: Component,
    T: Send + Sync + 'static,
    R: Resource,
{
    pub fn new(target_id: Entity, begin: T, end: T, interp_fn: InterpFn<C, T, R>) -> Self {
        Self {
            target_id,
            begin,
            end,
            interp_fn,
        }
    }
}

#[derive(Clone)]
pub struct ActionMeta {
    /// Target `Entity` for `Action`.
    action_id: Entity,
    /// Time at which animation should begin.
    start_time: f32,
    /// Duration of animation in seconds.
    duration: f32,
    /// Easing function to be used for animation.
    pub(crate) ease_fn: EaseFn,
}

impl ActionMeta {
    pub fn new(action_id: Entity) -> Self {
        Self {
            action_id,
            start_time: 0.0,
            duration: 0.0,
            ease_fn: quad::ease_in_out,
        }
    }

    pub fn id(&self) -> Entity {
        self.action_id
    }

    #[inline]
    pub fn with_start_time(mut self, start_time: f32) -> Self {
        self.start_time = start_time;
        self
    }

    #[inline]
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    #[inline]
    pub fn with_ease(mut self, ease_fn: EaseFn) -> Self {
        self.ease_fn = ease_fn;
        self
    }

    #[inline]
    pub fn start_time(&self) -> f32 {
        self.start_time
    }

    #[inline]
    pub fn end_time(&self) -> f32 {
        self.start_time + self.duration
    }

    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }
}

pub struct ActionBuilder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's> ActionBuilder<'a, 'w, 's> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self { commands }
    }

    pub fn play(
        &mut self,
        action: Action<impl Component, impl Send + Sync + 'static, impl Resource>,
        duration: f32,
    ) -> ActionMetaGroup {
        let action_id: Entity = self.commands.spawn(action).id();
        let action_meta: ActionMeta = ActionMeta::new(action_id).with_duration(duration);

        ActionMetaGroup::single(action_meta)
    }
}

#[derive(Clone)]
pub struct ActionMetaGroup {
    pub(crate) action_metas: Vec<ActionMeta>,
    pub(crate) duration: f32,
}

impl ActionMetaGroup {
    pub fn new() -> Self {
        Self {
            action_metas: Vec::new(),
            duration: 0.0,
        }
    }

    /// Create an `ActionMetaGroup` with only a single `ActionMeta` in it.
    pub fn single(action_meta: ActionMeta) -> Self {
        let duration: f32 = action_meta.duration();

        Self {
            action_metas: vec![action_meta],
            duration,
        }
    }

    pub fn with_ease(mut self, ease_fn: EaseFn) -> Self {
        for action_meta in &mut self.action_metas {
            action_meta.ease_fn = ease_fn;
        }

        self
    }
}