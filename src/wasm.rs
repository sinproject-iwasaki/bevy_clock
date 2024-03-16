use bevy::app::PluginGroupBuilder;
use bevy::{prelude::*, window::WindowResolution};

pub fn get_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800., 300.).with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    })
}
