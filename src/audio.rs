use bevy::prelude::*;

pub fn spawn_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    let source = asset_server.load("sounds/clock-ticking-60-second-countdown-118231.ogg");

    commands.spawn(AudioBundle {
        source,
        settings: PlaybackSettings::LOOP,
    });
}
