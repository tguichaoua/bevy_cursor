//! An example using two windows with multiple camera per window.

use bevy::color::palettes;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, Viewport};
use bevy::window::{ExitCondition, WindowRef, WindowResized};
use bevy_cursor::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::new(600.0, 400.0);

fn main() {
    App::new()
        //
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Primary"),
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            ..default()
        }))
        .add_plugins(TrackCursorPlugin)
        //
        .add_systems(Startup, setup)
        .add_systems(Update, set_camera_viewports)
        .add_systems(
            Update,
            print_cursor_location.run_if(resource_changed::<CursorLocation>),
        )
        .run();
}

// =============================================================================

/// A component to give a name to our cameras.
#[derive(Component)]
struct Name(String);

/// A marker for the left camera.
#[derive(Component)]
struct LeftCamera;

/// A marker for the right camera.
#[derive(Component)]
struct RightCamera;

/// A marker for the secondary window.
#[derive(Component)]
struct SecondaryWindow;

#[derive(Resource)]
struct TextEntities {
    window: Entity,
    camera: Entity,
    window_position: Entity,
    world_position: Entity,
}

// =============================================================================

fn setup(mut commands: Commands) {
    // Spawn a camera to render to the primary window.
    commands.spawn((Camera2d, Name(String::from("The default one"))));

    // Spawn a second window and two other cameras to render into.

    let secondary_window_ref = commands
        .spawn((
            Window {
                title: String::from("Secondary"),
                resolution: WINDOW_SIZE.into(),
                ..default()
            },
            SecondaryWindow,
        ))
        .id();

    // The left camera
    {
        commands.spawn((
            Camera2d,
            Transform::from_xyz(1000.0, 0.0, 0.0),
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(secondary_window_ref)),
                ..default()
            },
            Name(String::from("The left one")),
            LeftCamera,
        ));

        commands.spawn((
            Transform::from_xyz(1000.0, 0.0, 0.0),
            Text2d::new("Left"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor::WHITE,
        ));

        commands.spawn((
            Sprite {
                color: palettes::css::VIOLET.into(),
                custom_size: Some(Vec2::splat(1000.0)),
                ..default()
            },
            Transform::from_xyz(1000.0, 0.0, -1.0),
        ));
    }

    // The right camera
    {
        commands.spawn((
            Camera2d,
            Transform::from_xyz(2000.0, 0.0, 0.0),
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(secondary_window_ref)),
                order: 1,
                // don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
                ..default()
            },
            Name(String::from("The right one")),
            RightCamera,
        ));

        commands.spawn((
            Transform::from_xyz(2000.0, 0.0, 0.0),
            Text2d::new("Right"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor::WHITE,
        ));

        commands.spawn((
            Sprite {
                color: palettes::css::LIME.into(),
                custom_size: Some(Vec2::splat(1000.0)),
                ..default()
            },
            Transform::from_xyz(2000.0, 0.0, -1.0),
        ));
    }

    // Spawn ui texts

    const FONT_SIZE: f32 = 20.0;

    let mut text_entities = None;

    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            let window = parent
                .spawn(Text::new("Windows: "))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: FONT_SIZE,
                        ..default()
                    },
                    TextColor(palettes::css::GOLD.into()),
                ))
                .id();

            let camera = parent
                .spawn(Text::new("Camera: "))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: FONT_SIZE,
                        ..default()
                    },
                    TextColor(palettes::css::GOLD.into()),
                ))
                .id();

            let window_position = parent
                .spawn(Text::new("Window position: "))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: FONT_SIZE,
                        ..default()
                    },
                    TextColor(palettes::css::GOLD.into()),
                ))
                .id();

            let world_position = parent
                .spawn(Text::new("World Position: "))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: FONT_SIZE,
                        ..default()
                    },
                    TextColor(palettes::css::GOLD.into()),
                ))
                .id();

            text_entities = Some(TextEntities {
                window,
                camera,
                window_position,
                world_position,
            });
        });

    commands.insert_resource(text_entities.unwrap());
}

// =============================================================================

/// Update the viewport of the cameras on the secondary window when this one is resized.
fn set_camera_viewports(
    secondary_window_q: Query<&Window, With<SecondaryWindow>>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera_q: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera_q: Query<&mut Camera, With<RightCamera>>,
) {
    for resize_event in resize_events.read() {
        let Ok(window) = secondary_window_q.get(resize_event.window) else {
            continue;
        };

        let mut left_camera = left_camera_q.single_mut();
        left_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() / 2,
                window.resolution.physical_height(),
            ),
            ..default()
        });

        let mut right_camera = right_camera_q.single_mut();
        right_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() / 2,
                window.resolution.physical_height(),
            ),
            ..default()
        });
    }
}

// =============================================================================

/// Update the texts with the cursor location data.
#[allow(clippy::type_complexity)]
fn print_cursor_location(
    cursor: Res<CursorLocation>,

    mut text_writer: TextUiWriter,
    text_entities: Res<TextEntities>,

    window_q: Query<&Window>,
    name_q: Query<&Name>,
) {
    // A closure that update the `Text`s' value.
    let mut set_texts = |window_str, camera_str, viewport_str, world_pos_str| {
        *text_writer.text(text_entities.window, 1) = window_str;
        *text_writer.text(text_entities.camera, 1) = camera_str;
        *text_writer.text(text_entities.window_position, 1) = viewport_str;
        *text_writer.text(text_entities.world_position, 1) = world_pos_str;
    };

    if let Some(cursor) = cursor.get() {
        set_texts(
            format!(
                "{:?} ({:?})",
                window_q.get(cursor.window).unwrap().title,
                cursor.window,
            ),
            format!(
                "{:?} ({:?})",
                name_q.get(cursor.camera).unwrap().0,
                cursor.camera,
            ),
            cursor.position.to_string(),
            cursor.world_position.to_string(),
        );
    } else {
        set_texts(
            String::default(),
            String::default(),
            String::default(),
            String::default(),
        );
    }
}
