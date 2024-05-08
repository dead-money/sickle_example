use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the WorldInspectorPlugin or a custom inspector plugin to your app
        // so that you can tweak the layout of widgets at runtime.
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
