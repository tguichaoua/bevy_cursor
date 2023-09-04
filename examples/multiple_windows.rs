use bevy::core_pipeline::clear_color::ClearColorConfig;
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
        .add_plugins(CursorInfoPlugin)
        //
        .add_systems(Startup, setup)
        .add_systems(Update, set_camera_viewports)
        .add_systems(
            Update,
            print_cursor_data.run_if(resource_changed::<CursorInfo>()),
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

// A bunch of marker components to identify each text.

#[derive(Component)]
struct TextWindow;

#[derive(Component)]
struct TextCamera;

#[derive(Component)]
struct TextWindowPosition;

#[derive(Component)]
struct TextWorldPosition;

// =============================================================================

fn setup(mut commands: Commands) {
    // Spawn a camera to render to the primary window.
    commands.spawn((
        Camera2dBundle::default(),
        Name(String::from("The default one")),
    ));

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
            Camera2dBundle {
                transform: Transform::from_xyz(1000.0, 0.0, 0.0),
                camera: Camera {
                    target: RenderTarget::Window(WindowRef::Entity(secondary_window_ref)),
                    ..default()
                },
                ..default()
            },
            Name(String::from("The left one")),
            LeftCamera,
        ));

        commands.spawn(Text2dBundle {
            transform: Transform::from_xyz(1000.0, 0.0, 0.0),
            text: Text::from_section(
                "Left",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ..default()
        });

        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(1000.0, 0.0, -1.0),
            sprite: Sprite {
                color: Color::VIOLET,
                custom_size: Some(Vec2::splat(1000.0)),
                ..default()
            },
            ..default()
        });
    }

    // The right camera
    {
        commands.spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(2000.0, 0.0, 0.0),
                camera: Camera {
                    target: RenderTarget::Window(WindowRef::Entity(secondary_window_ref)),
                    order: 1,
                    ..default()
                },
                camera_2d: Camera2d {
                    // don't clear on the second camera because the first camera already cleared the window
                    clear_color: ClearColorConfig::None,
                },
                ..default()
            },
            Name(String::from("The right one")),
            RightCamera,
        ));

        commands.spawn(Text2dBundle {
            transform: Transform::from_xyz(2000.0, 0.0, 0.0),
            text: Text::from_section(
                "Right",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ..default()
        });

        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(2000.0, 0.0, -1.0),
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::splat(1000.0)),
                ..default()
            },
            ..default()
        });
    }

    // Spawn ui texts

    const FONT_SIZE: f32 = 30.0;

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Window: ",
                        TextStyle {
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: FONT_SIZE,
                        color: Color::GOLD,
                        ..default()
                    }),
                ]),
                TextWindow,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Camera: ",
                        TextStyle {
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: FONT_SIZE,
                        color: Color::GOLD,
                        ..default()
                    }),
                ]),
                TextCamera,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Window position: ",
                        TextStyle {
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: FONT_SIZE,
                        color: Color::GOLD,
                        ..default()
                    }),
                ]),
                TextWindowPosition,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "World Position: ",
                        TextStyle {
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: FONT_SIZE,
                        color: Color::GOLD,
                        ..default()
                    }),
                ]),
                TextWorldPosition,
            ));
        });
}

// =============================================================================

/// Update the viewport of the camera on the secondary window when this one is resized.
fn set_camera_viewports(
    secondary_window_q: Query<&Window, With<SecondaryWindow>>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera_q: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera_q: Query<&mut Camera, With<RightCamera>>,
) {
    for resize_event in resize_events.iter() {
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

/// Update the textes with the cursor informations.
#[allow(clippy::type_complexity)]
fn print_cursor_data(
    cursor: Res<CursorInfo>,

    mut set: ParamSet<(
        Query<&mut Text, With<TextWindow>>,
        Query<&mut Text, With<TextCamera>>,
        Query<&mut Text, With<TextWindowPosition>>,
        Query<&mut Text, With<TextWorldPosition>>,
    )>,

    window_q: Query<&Window>,
    name_q: Query<&Name>,
) {
    // A closure that update the `Text`s' value.
    let mut set_texts = |a, b, c, d| {
        let mut window_text_q = set.p0();
        let mut window_text = window_text_q.single_mut();
        window_text.sections[1].value = a;

        let mut camera_text_q = set.p1();
        let mut camera_text = camera_text_q.single_mut();
        camera_text.sections[1].value = b;

        let mut viewport_position_text_q = set.p2();
        let mut viewport_position_text = viewport_position_text_q.single_mut();
        viewport_position_text.sections[1].value = c;

        let mut world_position_text_q = set.p3();
        let mut world_position_text = world_position_text_q.single_mut();
        world_position_text.sections[1].value = d;
    };

    if let Some(cursor) = cursor.get() {
        set_texts(
            format!(
                "{:?} {:?}",
                cursor.window,
                window_q.get(cursor.window).unwrap().title
            ),
            format!(
                "{:?} {:?}",
                cursor.camera,
                name_q.get(cursor.camera).unwrap().0
            ),
            cursor.window_position.to_string(),
            cursor.position.to_string(),
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
