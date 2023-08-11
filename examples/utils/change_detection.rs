use std::mem;

use bevy::ecs::change_detection::DetectChangesMut;

pub trait DetectChangesMutExt: DetectChangesMut {
    /// Overwrites this smart pointer with the given value, if and only if `*self != value`
    /// returning the previous value if this occurs.
    ///
    /// This is useful to ensure change detection is only triggered when the underlying value
    /// changes, instead of every time it is mutably accessed.
    ///
    /// If you don't need to handle the previous value, use [`set_if_neq`](DetectChangesMut::set_if_neq).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use bevy_ecs::{prelude::*, schedule::common_conditions::{resource_changed, on_event}};
    /// #[derive(Resource, PartialEq, Eq)]
    /// pub struct Score(u32);
    ///
    /// #[derive(Event, PartialEq, Eq)]
    /// pub struct ScoreChanged {
    ///     current: u32,
    ///     previous: u32,
    /// }
    ///
    /// fn reset_score(mut score: ResMut<Score>, mut score_changed: EventWriter<ScoreChanged>) {
    ///     // Set the score to zero, unless it is already zero.
    ///     let new_score = 0;
    ///     if let Some(Score(previous_score)) = score.replace_if_neq(Score(new_score)) {
    ///         // If `score` change, emit a `ScoreChanged` event.
    ///         score_changed.send(ScoreChanged {
    ///             current: new_score,
    ///             previous: previous_score,
    ///         });
    ///     }
    /// }
    /// # let mut world = World::new();
    /// # world.insert_resource(Events::<ScoreChanged>::default());
    /// # world.insert_resource(Score(1));
    /// # let mut score_changed = IntoSystem::into_system(resource_changed::<Score>());
    /// # score_changed.initialize(&mut world);
    /// # score_changed.run((), &mut world);
    /// #
    /// # let mut score_changed_event = IntoSystem::into_system(on_event::<ScoreChanged>());
    /// # score_changed_event.initialize(&mut world);
    /// # score_changed_event.run((), &mut world);
    /// #
    /// # let mut schedule = Schedule::new();
    /// # schedule.add_systems(reset_score);
    /// #
    /// # // first time `reset_score` runs, the score is changed.
    /// # schedule.run(&mut world);
    /// # assert!(score_changed.run((), &mut world));
    /// # assert!(score_changed_event.run((), &mut world));
    /// # // second time `reset_score` runs, the score is not changed.
    /// # schedule.run(&mut world);
    /// # assert!(!score_changed.run((), &mut world));
    /// # assert!(!score_changed_event.run((), &mut world));
    /// ```
    #[inline]
    #[must_use = "If you don't need to handle the previous value, use `set_if_neq` instead."]
    fn replace_if_neq(&mut self, value: Self::Inner) -> Option<Self::Inner>
    where
        Self::Inner: Sized + PartialEq,
    {
        let old = self.bypass_change_detection();
        if *old != value {
            let previous = mem::replace(old, value);
            self.set_changed();
            Some(previous)
        } else {
            None
        }
    }
}

impl<T: DetectChangesMut> DetectChangesMutExt for T {}
