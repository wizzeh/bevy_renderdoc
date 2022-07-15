//! Wrapper around [`RenderDoc`] for Bevy.
//!
//! Provides an easy way to register RenderDoc with a Bevy application.
//! Allows the user to launch the RenderDoc UI on capture, which makes
//! taking captures more convenient.
#![warn(missing_docs)]
use bevy::prelude::*;
pub use renderdoc::*;
use sysinfo::{Pid, ProcessRefreshKind, SystemExt};

/// Trait for creating a Bevy [`App`] with RenderDoc properly initialized.
///
/// We need load RenderDoc before any windows or devices have been created.
/// From what I could tell, there's no way of doing that using a [`Plugin`], so
/// having the user create the [`App`] using this trait is the next best thing.
pub trait AddRenderDocPlugin {
    /// Initializes [`RenderDoc`] and registers the plugin with an [`App`].
    /// The app is created using [`App::new()`].
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_renderdoc::*;
    ///
    /// App::with_renderdoc::<V110>().run();
    /// ```
    fn with_renderdoc<V: Version + 'static>() -> App {
        App::with_renderdoc_custom::<V>(App::new)
    }

    /// Initializes [`RenderDoc`] and registers the plugin with an [`App`] returned from `f`.
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_renderdoc::*;    
    ///
    /// App::with_renderdoc_custom::<V110>(App::new).run();
    /// ```
    fn with_renderdoc_custom<V: Version + 'static>(f: fn() -> App) -> App;
}

impl AddRenderDocPlugin for App {
    fn with_renderdoc_custom<V: Version + 'static>(f: fn() -> App) -> App {
        // This needs to happen before App::new()
        let rd = RenderDoc::<V>::new();
        let mut app = f();

        // TODO: The tracing crate doesn't work here. Figure out why, and use info!/error! macros here.
        match rd {
            Ok(rd) => {
                app.insert_non_send_resource(rd);
                println!("Initialized RenderDoc successfully!");
            }
            Err(e) => {
                println!("Failed to initialize RenderDoc: {:?}", e);
            }
        };

        app.add_plugin(RenderDocPlugin);
        app
    }
}

struct RenderDocPlugin;
impl Plugin for RenderDocPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_renderdoc)
            .add_system(trigger_capture);
    }
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
    key: Option<Res<Input<KeyCode>>>,
    rd: Option<NonSend<RenderDoc<V110>>>,
    mut replay_pid: Local<usize>,
    mut system: Local<sysinfo::System>,
) {
    if rd.is_none() || key.is_none() {
        return;
    }

    let rd = rd.unwrap();
    if key.unwrap().just_pressed(KeyCode::F12) {
        // Avoid launching multiple instances of the replay ui
        if system
            .refresh_process_specifics(Pid::from(*replay_pid), ProcessRefreshKind::new().with_cpu())
        {
            return;
        }

        match rd.launch_replay_ui(true, None) {
            Ok(pid) => {
                *replay_pid = pid as usize;
                info!("Launching RenderDoc Replay UI");
            }
            Err(e) => error!("Failed to launch RenderDoc Replay UI: {:?}", e),
        }
    }
}
