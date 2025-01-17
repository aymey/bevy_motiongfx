use bevy::prelude::*;

use crate::{
    ease::{cubic, EaseFn},
    f32lerp::F32Lerp,
    prelude::MultiSequenceOrdering,
    sequence::Sequence,
};

/// Function for interpolating a type based on a [`f32`] time.
pub type InterpFn<T> = fn(start: &T, end: &T, t: f32) -> T;
/// Function for getting a mutable reference of a field (or itself) of type `T` in type `U`.
pub type GetFieldMut<T, U> = fn(comp: &mut U) -> &mut T;

#[macro_export]
macro_rules! act {
    (
        ($target_id:expr, $comp_ty:ty),
        start = { $root:expr }.$($path:tt).+,
        end = $value:expr,
    ) => {
        {
            let action = $crate::action::Action::new_f32lerp(
                $target_id,
                $root.$($path).+.clone(),
                $value.clone(),
                |source: &mut $comp_ty| &mut source.$($path).+
            );

            $root.$($path).+ = $value;

            action
        }
    };
    (
        ($target_id:expr, $comp_ty:ty),
        start = { $root:expr },
        end = $value:expr,
    ) => {
        {
            let action = $crate::action::Action::new_f32lerp(
                $target_id,
                $root.clone(),
                $value.clone(),
                |source: &mut $comp_ty| source
            );

            #[allow(unused_assignments)]
            {
                $root = $value;
            }

            action
        }
    };
}

pub use act;

/// Basic data structure to describe an animation action.
#[derive(Component, Clone, Copy)]
pub struct Action<T, U> {
    /// Target [`Entity`] for [`Component`] manipulation.
    pub(crate) target_id: Entity,
    /// Initial value of the action.
    pub(crate) start: T,
    /// Final value of the action.
    pub(crate) end: T,
    /// Function for interpolating the value based on a [`f32`] time.
    pub(crate) interp_fn: InterpFn<T>,
    /// Function for getting a mutable reference of a field (or itself) from the component.
    pub(crate) get_field_fn: GetFieldMut<T, U>,
    /// Function for easing the [`f32`] time value for the action.
    pub(crate) ease_fn: EaseFn,
}

impl<T, U> Action<T, U> {
    pub fn new(
        target_id: Entity,
        start: T,
        end: T,
        interp_fn: InterpFn<T>,
        get_field_fn: GetFieldMut<T, U>,
    ) -> Self {
        Self {
            target_id,
            start,
            end,
            interp_fn,
            get_field_fn,
            ease_fn: cubic::ease_in_out,
        }
    }

    pub fn with_ease(mut self, ease_fn: EaseFn) -> Self {
        self.ease_fn = ease_fn;
        self
    }

    pub fn with_interp(mut self, interp_fn: InterpFn<T>) -> Self {
        self.interp_fn = interp_fn;
        self
    }

    pub fn animate(self, duration: f32) -> Motion<T, U> {
        Motion {
            action: self,
            duration,
        }
    }
}

impl<T, U> Action<T, U>
where
    T: F32Lerp,
{
    pub fn new_f32lerp(
        target_id: Entity,
        start: T,
        end: T,
        get_field_fn: GetFieldMut<T, U>,
    ) -> Self {
        Self {
            target_id,
            start,
            end,
            interp_fn: T::f32lerp,
            get_field_fn,
            ease_fn: cubic::ease_in_out,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ActionMeta {
    /// Target `Entity` for `Action`.
    action_id: Entity,
    /// Time at which animation should begin.
    pub(crate) start_time: f32,
    /// Duration of animation in seconds.
    pub(crate) duration: f32,
    /// Slide that this action belongs to.
    pub(crate) slide_index: usize,
}

impl ActionMeta {
    pub fn new(action_id: Entity) -> Self {
        Self {
            action_id,
            start_time: 0.0,
            duration: 0.0,
            slide_index: 0,
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
    pub fn end_time(&self) -> f32 {
        self.start_time + self.duration
    }
}

#[derive(Clone, Copy)]
pub struct Motion<T, U> {
    pub action: Action<T, U>,
    pub duration: f32,
}

pub struct SequenceBuilder<'w, 's> {
    commands: Commands<'w, 's>,
    sequences: Vec<Sequence>,
}

impl<'a> SequenceBuilder<'a, 'a> {
    /// Converts a [`Motion`] into a [`SequenceBuilder`].
    pub fn add_motion<T, U>(mut self, motion: Motion<T, U>) -> Self
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        self.sequences.push(self.commands.play_motion(motion));
        self
    }

    pub fn build(self) -> Vec<Sequence> {
        self.sequences
    }
}

impl MultiSequenceOrdering for SequenceBuilder<'_, '_> {
    fn chain(self) -> Sequence {
        self.sequences.chain()
    }

    fn all(self) -> Sequence {
        self.sequences.all()
    }

    fn any(self) -> Sequence {
        self.sequences.any()
    }

    fn flow(self, delay: f32) -> Sequence {
        self.sequences.flow(delay)
    }
}

pub trait SequenceBuilderExt<'w> {
    /// Converts a [`Motion`] into a [`Sequence`].
    fn play_motion<T, U>(&mut self, motion: Motion<T, U>) -> Sequence
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static;

    /// Converts a [`Motion`] into a [`SequenceBuilder`].
    fn add_motion<T, U>(&mut self, motion: Motion<T, U>) -> SequenceBuilder<'w, '_>
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static;

    fn sleep(&mut self, duration: f32) -> Sequence;
}

impl<'w> SequenceBuilderExt<'w> for Commands<'w, '_> {
    fn play_motion<T, U>(&mut self, motion: Motion<T, U>) -> Sequence
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let action_id = self.spawn(motion.action).id();
        let mut action_meta = ActionMeta::new(action_id);
        action_meta.duration = motion.duration;

        Sequence::single(action_meta)
    }

    fn add_motion<T, U>(&mut self, motion: Motion<T, U>) -> SequenceBuilder<'w, '_>
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let mut commands = self.reborrow();
        let sequences = vec![commands.play_motion(motion)];
        SequenceBuilder {
            commands,
            sequences,
        }
    }

    fn sleep(&mut self, duration: f32) -> Sequence {
        Sequence::empty(duration)
    }
}
