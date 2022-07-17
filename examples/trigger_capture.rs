use bevy::prelude::*;
use bevy_renderdoc::*;

fn trigger_capture(rd: Option<NonSendMut<RenderDocResource>>) {
    if let Some(mut rd) = rd {
        rd.trigger_capture();
        rd.launch_replay_ui(true, None).unwrap();
    }
}

fn main() {
    App::new()
        .add_plugin(RenderDocPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(trigger_capture)
        .run();
}
