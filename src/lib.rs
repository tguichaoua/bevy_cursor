#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! A [`bevy`] plugin to track information about the cursor.
//!
//! - The [entity id] of the window on which the cursor is currently;
//! - The [entity id] of the camera on which the cursor is currently;
//! - The position of the cursor on the window (logical position);
//! - The 2D world position of the cursor (if the feature `2d` is enabled);
//! - The [ray] emitted by the cursor through the camera (if the feature `3d` is enabled);
//!
//! # Bevy compatible version
//!
//! | bevy | bevy_cursor |
//! | ---- | ----------- |
//! | 0.14 | 0.4         |
//! | 0.13 | 0.3         |
//! | 0.12 | 0.2         |
//! | 0.11 | 0.1         |
//!
//! [`bevy`]: https://github.com/bevyengine/bevy
//! [entity id]: https://docs.rs/bevy/0.14.0/bevy/ecs/entity/struct.Entity.html
//! [ray]: https://docs.rs/bevy/0.14.0/bevy/math/struct.Ray3d.html

use bevy::ecs::query::Has;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowRef};
use smallvec::SmallVec;

/* -------------------------------------------------------------------------- */

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{CursorLocation, TrackCursorPlugin, UpdateCursorLocation};
}

/* -------------------------------------------------------------------------- */

/// This plugin adds support to track the cursor's position, window, and camera.
///
/// Those values are provided by the [`CursorLocation`] resource.
pub struct TrackCursorPlugin;

impl Plugin for TrackCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorLocation>().add_systems(
            First,
            update_cursor_location_res.in_set(UpdateCursorLocation),
        );
    }
}

/* -------------------------------------------------------------------------- */

/// A [`SystemSet`] in which [`CursorLocation`] is updated during the [`First`] schedule.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_cursor::prelude::*;
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(First, print_cursor_location.after(UpdateCursorLocation));
///     }
/// }
///
/// // Runs just after `CursorLocation` has been updated.
/// fn print_cursor_location(cursor: Res<CursorLocation>) {
///     /* ... */
/// }
/// ```
///
/// [`SystemSet`]: https://docs.rs/bevy/0.14.0/bevy/ecs/schedule/trait.SystemSet.html
/// [`First`]: https://docs.rs/bevy/0.14.0/bevy/app/struct.First.html
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct UpdateCursorLocation;

/* -------------------------------------------------------------------------- */

/// A resource that provides the [`Location`] data of the cursor.
///
/// The [`Location`] is available only if the cursor is currently inside one
/// of the windows area.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_cursor::prelude::*;
/// fn print_cursor_location(cursor: Res<CursorLocation>) {
///     if let Some(position) = cursor.position() {
///         info!("Cursor position: {position:?}");
///     } else {
///         info!("The cursor is not in any window");
///     }
/// }
///
/// # let _ = IntoSystem::into_system(print_cursor_location);
/// ```
#[derive(Resource, Default)]
pub struct CursorLocation(Option<Location>);

/// The location of the cursor (its position, window, and camera).
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    /// The cursor position in the window in logical pixels.
    ///
    /// See [`Window::cursor_position`].
    ///
    /// [`Window::cursor_position`]: https://docs.rs/bevy/0.14.0/bevy/window/struct.Window.html#method.cursor_position
    pub position: Vec2,

    /// The entity id of the window that contains the cursor.
    pub window: Entity,

    /// The entity id of the camera used to compute the world position of the cursor.
    pub camera: Entity,

    /// The position of the cursor in the world coordinates.
    ///
    /// This value is computed with [`Camera::viewport_to_world_2d`].
    ///
    /// [`Camera::viewport_to_world_2d`]: https://docs.rs/bevy/0.14.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world_2d
    #[cfg(feature = "2d")]
    pub world_position: Vec2,

    /// The [`Ray3d`] emitted by the cursor from the camera.
    ///
    /// This value is computed with [`Camera::viewport_to_world`].
    ///
    /// [`Ray3d`]: https://docs.rs/bevy/0.14.0/bevy/math/struct.Ray3d.html
    /// [`Camera::viewport_to_world`]: https://docs.rs/bevy/0.14.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world
    #[cfg(feature = "3d")]
    pub ray: Ray3d,
}

impl CursorLocation {
    /// The [`Location`] of the cursor.
    ///
    /// Returns [`None`] if the cursor is outside any window area.
    #[inline]
    pub fn get(&self) -> Option<&Location> {
        self.0.as_ref()
    }

    /// The cursor position in the window in logical pixels.
    ///
    /// See [`Window::cursor_position`].
    ///
    /// Returns [`None`] if the cursor is outside any window area.
    ///
    /// [`Window::cursor_position`]: https://docs.rs/bevy/0.14.0/bevy/window/struct.Window.html#method.cursor_position
    #[inline]
    pub fn position(&self) -> Option<Vec2> {
        self.get().map(|data| data.position)
    }

    /// The entity id of the window that contains the cursor.
    ///
    /// Returns [`None`] if the cursor is outside any window area.
    #[inline]
    pub fn window(&self) -> Option<Entity> {
        self.get().map(|data| data.window)
    }

    /// The entity id of the camera used to compute the world position of the cursor.
    ///
    /// Returns [`None`] if the cursor is outside any window area.
    #[inline]
    pub fn camera(&self) -> Option<Entity> {
        self.get().map(|data| data.camera)
    }

    /// The position of the cursor in the world coordinates.
    ///
    /// This value is computed with [`Camera::viewport_to_world_2d`].
    ///
    /// Returns [`None`] if the cursor is outside any window area.
    ///
    /// [`Camera::viewport_to_world_2d`]: https://docs.rs/bevy/0.14.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world_2d
    #[cfg(feature = "2d")]
    #[inline]
    pub fn world_position(&self) -> Option<Vec2> {
        self.get().map(|data| data.world_position)
    }

    /// The [`Ray3d`] emitted by the cursor from the camera.
    ///
    /// This value is computed with [`Camera::viewport_to_world`].
    ///
    /// Returns [`None`] if the cursor is outside any window area.
    ///
    /// [`Ray3d`]: https://docs.rs/bevy/0.14.0/bevy/math/struct.Ray3d.html
    /// [`Camera::viewport_to_world`]: https://docs.rs/bevy/0.14.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world
    #[cfg(feature = "3d")]
    #[inline]
    pub fn ray(&self) -> Option<Ray3d> {
        self.get().map(|data| data.ray)
    }
}

/* -------------------------------------------------------------------------- */

/// Reads the current cursor position and update the [`CursorLocation`] resource.
fn update_cursor_location_res(
    window_q: Query<(Entity, &Window, Has<PrimaryWindow>)>,
    camera_q: Query<(Entity, &GlobalTransform, &Camera)>,
    cursor: ResMut<CursorLocation>,
) {
    let mut cursor = cursor.map_unchanged(|cursor| &mut cursor.0);

    for (win_ref, window, is_primary) in &window_q {
        // Get the window that contains the cursor.
        let Some(cursor_position) = window.cursor_position() else {
            continue;
        };
        let Some(physical_cursor_position) = window.physical_cursor_position() else {
            continue;
        };

        // Get the cameras that render into the current window.
        let mut cameras = camera_q
            .iter()
            .filter(|&(_, _, camera)| match camera.target {
                RenderTarget::Window(WindowRef::Primary) => is_primary,
                RenderTarget::Window(WindowRef::Entity(target_ref)) => target_ref == win_ref,
                RenderTarget::Image(_) | RenderTarget::TextureView(_) => false,
            })
            // PERF: this is unlikely to have more than 4 cameras on the same window.
            .collect::<SmallVec<[_; 4]>>();

        // Cameras with a higher order are rendered later, and thus on top of lower order cameras.
        // We want to handle them first.
        cameras.sort_unstable_by_key(|&(_, _, camera)| camera.order);
        let cameras = cameras.into_iter().rev();

        for (camera_ref, cam_t, camera) in cameras {
            let _ = cam_t; // Note: disable the `unused_variables` warning in no-default-feature.

            // Does the camera viewport contain the cursor ?
            let contain_cursor = match camera.viewport {
                Some(ref viewport) => {
                    let Vec2 { x, y } = physical_cursor_position;
                    let Vec2 { x: vx, y: vy } = viewport.physical_position.as_vec2();
                    let Vec2 { x: vw, y: vh } = viewport.physical_size.as_vec2();
                    x >= vx && x <= (vx + vw) && y >= vy && y <= (vy + vh)
                }
                None => true,
            };

            if !contain_cursor {
                continue;
            }

            #[cfg(feature = "2d")]
            let Ok(world_position) = camera.viewport_to_world_2d(cam_t, cursor_position) else {
                continue;
            };

            #[cfg(feature = "3d")]
            let Ok(ray) = camera.viewport_to_world(cam_t, cursor_position) else {
                continue;
            };

            cursor.set_if_neq(Some(Location {
                position: cursor_position,
                window: win_ref,
                camera: camera_ref,

                #[cfg(feature = "2d")]
                world_position,

                #[cfg(feature = "3d")]
                ray,
            }));

            // We found the correct window and camera, we can stop here.
            return;
        }
    }

    // The cursor is outside of every windows.
    cursor.set_if_neq(None);
}

/* -------------------------------------------------------------------------- */
