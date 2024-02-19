#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//!
#![doc = include_str!("../README.md")]

/* -------------------------------------------------------------------------- */

use bevy::ecs::query::Has;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowRef};
use smallvec::SmallVec;

/* -------------------------------------------------------------------------- */

/// Export common types.
pub mod prelude {
    pub use crate::{CursorInfo, CursorInfoPlugin, UpdateCursorInfo};
}

/* -------------------------------------------------------------------------- */

/// This plugin adds support to get information about the cursor.
pub struct CursorInfoPlugin;

impl Plugin for CursorInfoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorInfo>()
            .add_systems(First, update_cursor_info.in_set(UpdateCursorInfo));
    }
}

/* -------------------------------------------------------------------------- */

/// A [`SystemSet`] in which [`CursorInfo`] is updated.
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
///         app.add_systems(First, foo.after(UpdateCursorInfo));
///     }
/// }
///
/// // Runs just after `CursorInfo` has been updated.
/// fn foo(cursor: Res<CursorInfo>) {
///     /* ... */
/// }
/// ```
///
/// [`SystemSet`]: https://docs.rs/bevy/0.13.0/bevy/ecs/schedule/trait.SystemSet.html
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct UpdateCursorInfo;

/* -------------------------------------------------------------------------- */

/// A resource that provides information about the cursor.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_cursor::prelude::*;
/// fn foo(cursor: Res<CursorInfo>) {
///     if let Some(position) = cursor.position() {
///         info!("Cursor position: {position:?}");
///     } else {
///         info!("The cursor is not in any window");
///     }
/// }
///
/// # let _ = IntoSystem::into_system(foo);
/// ```
#[derive(Resource, Default)]
pub struct CursorInfo(Option<CursorData>);

/// Information about the cursor, provided by [`CursorInfo`].
#[derive(Debug, Clone, PartialEq)]
pub struct CursorData {
    /// The position of the cursor in the world.
    ///
    /// See [`Camera::viewport_to_world_2d`].
    ///
    /// [`Camera::viewport_to_world_2d`]: https://docs.rs/bevy/0.13.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world_2d
    #[cfg(feature = "2d")]
    pub position: Vec2,
    /// The [`Ray3d`] emitted by the cursor from the camera.
    ///
    /// See [`Camera::viewport_to_world`].
    ///
    /// [`Ray3d`]: https://docs.rs/bevy/0.13.0/bevy/math/struct.Ray3d.html
    /// [`Camera::viewport_to_world`]: https://docs.rs/bevy/0.13.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world
    #[cfg(feature = "3d")]
    pub ray: Ray3d,
    /// The cursor position in the window in logical pixels.
    ///
    /// See [`Window::cursor_position`].
    ///
    /// [`Window::cursor_position`]: https://docs.rs/bevy/0.13.0/bevy/window/struct.Window.html#method.cursor_position
    pub window_position: Vec2,
    /// The entity id of the window that contains the cursor.
    pub window: Entity,
    /// The entity id of the camera used to compute the world position of the cursor.
    pub camera: Entity,
}

impl CursorInfo {
    /// The information about the cursor.
    ///
    /// The value is `None` if the cursor is not in any window.
    #[inline]
    pub fn get(&self) -> Option<&CursorData> {
        self.0.as_ref()
    }

    /// The position of the cursor in the world.
    ///
    /// See [`Camera::viewport_to_world_2d`].
    ///
    /// The value is `None` if the cursor is not in any window.
    ///
    /// [`Camera::viewport_to_world_2d`]: https://docs.rs/bevy/0.13.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world_2d
    #[cfg(feature = "2d")]
    #[inline]
    pub fn position(&self) -> Option<Vec2> {
        self.get().map(|data| data.position)
    }

    /// The [`Ray3d`] emitted by the cursor from the camera.
    ///
    /// See [`Camera::viewport_to_world`].
    ///
    /// The value is `None` if the cursor is not in any window.
    ///
    /// [`Ray3d`]: https://docs.rs/bevy/0.13.0/bevy/math/struct.Ray3d.html
    /// [`Camera::viewport_to_world`]: https://docs.rs/bevy/0.13.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world
    #[cfg(feature = "3d")]
    #[inline]
    pub fn ray(&self) -> Option<Ray3d> {
        self.get().map(|data| data.ray)
    }

    /// The cursor position in the window in logical pixels.
    ///
    /// See [`Window::cursor_position`].
    ///
    /// The value is `None` if the cursor is not in any window.
    ///
    /// [`Window::cursor_position`]: https://docs.rs/bevy/0.13.0/bevy/window/struct.Window.html#method.cursor_position
    #[inline]
    pub fn window_position(&self) -> Option<Vec2> {
        self.get().map(|data| data.window_position)
    }

    /// The entity id of the window that contains the cursor.
    ///
    /// The value is `None` if the cursor is not in any window.
    #[inline]
    pub fn window(&self) -> Option<Entity> {
        self.get().map(|data| data.window)
    }

    /// The entity id of the camera used to compute the world position of the cursor.
    ///
    /// The value is `None` if the cursor is not in any window.
    #[inline]
    pub fn camera(&self) -> Option<Entity> {
        self.get().map(|data| data.camera)
    }
}

/* -------------------------------------------------------------------------- */

/// Reads the current cursor position and update the [`CursorInfo`] resource.
fn update_cursor_info(
    window_q: Query<(Entity, &Window, Has<PrimaryWindow>)>,
    camera_q: Query<(Entity, &GlobalTransform, &Camera)>,
    cursor: ResMut<CursorInfo>,
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
            let Some(position) = camera.viewport_to_world_2d(cam_t, cursor_position) else {
                continue;
            };

            #[cfg(feature = "3d")]
            let Some(ray) = camera.viewport_to_world(cam_t, cursor_position) else {
                continue;
            };

            cursor.set_if_neq(Some(CursorData {
                #[cfg(feature = "2d")]
                position,

                #[cfg(feature = "3d")]
                ray,

                window_position: cursor_position,
                window: win_ref,
                camera: camera_ref,
            }));

            // We found the correct window and camera, we can stop here.
            return;
        }
    }

    // The cursor is outside of every windows.
    cursor.set_if_neq(None);
}

/* -------------------------------------------------------------------------- */
