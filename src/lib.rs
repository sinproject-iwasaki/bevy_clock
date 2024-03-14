#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use bevy::prelude::*;

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_time)
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

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("00:00:00", text_style.clone())
                .with_justify(text_justification),
            ..default()
        })
        .insert(TimeText);
}

fn update_time(mut query: Query<&mut Text, With<TimeText>>, windows: Query<&Window>) {
    use chrono::Local;

    let window = windows.single();
    let font_size_by_width = window.width() / 3.5;
    let font_size_by_height = window.height();
    let font_size = font_size_by_width.min(font_size_by_height) / 1.5;

    for mut text in query.iter_mut() {
        let now = Local::now();
        text.sections[0].value = now.format("%H:%M:%S.%3f").to_string();
        text.sections[0].style.font_size = font_size;
    }
}
