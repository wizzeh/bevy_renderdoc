use bevy::prelude::*;
use bevy_renderdoc::*;

fn trigger_capture(mut rd: NonSendMut<RenderDocResource>) {
    if let Ok(rd) = rd.as_mut() {
        rd.trigger_capture();
        rd.launch_replay_ui(true, None).unwrap();
    }
}

fn main() {
    App::with_renderdoc()
        .add_plugins(DefaultPlugins)
        .add_startup_system(trigger_capture)
        .run();
}
