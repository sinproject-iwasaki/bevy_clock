#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use bevy::{asset::AssetMetaCheck, prelude::*};

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_time)
        // .add_systems(Update, animate_transform)
        .add_systems(Update, animate_rotation)
        .add_systems(Update, animate_scale)
        .run();
}

#[derive(Component)]
struct TimeText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_justification = JustifyText::Center;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("00:00:00", text_style.clone())
                .with_justify(text_justification),
            ..default()
        },
        TimeText,
    ));
}

fn update_time(mut query: Query<&mut Text, With<TimeText>>, windows: Query<&Window>) {
    use chrono::Local;

    let window = windows.single();
    let font_size_by_width = window.width() / 3.5;
    let font_size_by_height = window.height();
    let font_size = font_size_by_width.min(font_size_by_height) / 1.5;

    for mut text in &mut query {
        let now = Local::now();
        text.sections[0].value = now.format("%H:%M:%S.%3f").to_string();
        text.sections[0].style.font_size = font_size;
    }
}

// fn animate_transform(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, (With<Text>, With<TimeText>)>,
// ) {
//     for mut transform in query.iter_mut() {
//         transform.translation.x = 50.0 * time.elapsed_seconds().sin();
//         transform.translation.y = 50.0 * time.elapsed_seconds().cos();
//     }
// }

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<TimeText>)>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos() * 0.1);
    }
}

fn animate_scale(time: Res<Time>, mut query: Query<&mut Transform, (With<Text>, With<TimeText>)>) {
    for mut transform in query.iter_mut() {
        // transform.translation = Vec3::new(400.0, 0.0, 0.0);

        let scale = (time.elapsed_seconds().sin() + 1.1) * 1.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
