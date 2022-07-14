use bevy::prelude::*;
pub use renderdoc::*;

pub struct RenderDocPlugin;

pub struct RenderDocReplayUi(pub u32);

// We need to inject RenderDoc before any windows or devices have been created.
// From what I could tell, there's no way of doing that, so the next best thing
// for having this plugin be a simple drag and drop is by modifying the `new()` function.
pub trait AddRenderDocPlugin {
    fn with_renderdoc<V: Version + 'static>() -> Self;
}

impl AddRenderDocPlugin for App {
    fn with_renderdoc<V: Version + 'static>() -> Self {
        // This needs to happen before App::new()
        let rd = RenderDoc::<V>::new();
        let mut app = App::new();

        app.insert_non_send_resource(rd.unwrap())
            .add_startup_system(setup_renderdoc)
            .add_system(trigger_capture);

        app
    }
}

impl Plugin for RenderDocPlugin {
    fn build(&self, _: &mut App) {}
}

fn setup_renderdoc(rd: Option<NonSendMut<RenderDoc<V110>>>) {
    if rd.is_none() {
        return;
    }

    let mut rd = rd.unwrap();
    rd.set_log_file_path_template("renderdoc/bevy_capture");
    rd.mask_overlay_bits(OverlayBits::NONE, OverlayBits::NONE);
}

fn trigger_capture(
    key: Res<Input<KeyCode>>,
    rd: Option<NonSend<RenderDoc<V110>>>,
    mut replay_pid: Local<u32>,
) {
    // avoid launching multiple instances of replay ui
    if rd.is_none() || *replay_pid != 0 {
        return;
    }

    let rd = rd.unwrap();
    if key.just_pressed(KeyCode::F12) {
        *replay_pid = rd.launch_replay_ui(true, None).unwrap();
    }
}
