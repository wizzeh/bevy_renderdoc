//! Wrapper around [`RenderDoc`] for Bevy.
//!
//! Provides an easy way to register RenderDoc with a Bevy application.
//! Allows the user to launch the RenderDoc UI on capture, which makes
//! taking captures more convenient.
#![deny(missing_docs)]
use bevy::prelude::*;
use renderdoc::*;
use sysinfo::{Pid, ProcessRefreshKind, SystemExt};

pub use renderdoc;

/// The RenderDoc API version this plugin uses.
pub type RenderDocVersion = V110;

/// Trait for creating a Bevy [`App`] with [`RenderDoc`] properly initialized.
///
/// [`RenderDoc`] needs to be loaded before any windows or render devices have been created.
/// This is not possible using a [`Plugin`], since the render device
/// is loaded outside of Bevy's scheduling, inside of `RenderPlugin::build()`.
///
/// Technically, it would be possible to do *if* the user ordered their plugins in a way
/// that `RenderDocPlugin::build()` gets called before the renderer's. But since that is very
/// error prone, we instead enforce proper initialization using this trait.
pub trait WithRenderDoc {
    /// Initializes [`RenderDoc`] and registers the plugin with an [`App`].
    /// The app is created using [`App::new()`].
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_renderdoc::*;
    ///
    /// App::with_renderdoc().run();
    /// ```
    fn with_renderdoc() -> App {
        App::with_renderdoc_custom(App::new)
    }

    /// Initializes [`RenderDoc`] and registers the plugin with an [`App`] returned from `f`.
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_renderdoc::*;    
    ///
    /// App::with_renderdoc_custom(App::new).run();
    /// ```
    fn with_renderdoc_custom(f: fn() -> App) -> App;
}

/// The type of the [`NonSend`] resource used to store [`RenderDoc`] in Bevy.
///
/// It is a [`Result`] because we want to log any loading errors after
/// the Bevy app has been initialized and logging has been setup.
///
/// # Examples
/// ```
/// # use bevy::prelude::*;
/// # use bevy_renderdoc::*;
/// #
/// fn modify_renderdoc(mut rd: NonSendMut<RenderDocResource>) {
///     if let Ok(rd) = rd.as_mut() {
///         rd.set_log_file_path_template("your_path/file_prefix");
///     }
/// }
///
/// App::with_renderdoc()
///     .add_startup_system(modify_renderdoc)
///     .run();
/// ```
pub type RenderDocResource = Result<RenderDoc<RenderDocVersion>, Error>;

impl WithRenderDoc for App {
    fn with_renderdoc_custom(f: fn() -> App) -> App {
        // This needs to happen before App::new()
        let rd = RenderDoc::<RenderDocVersion>::new();
        let mut app = f();

        app.insert_non_send_resource(rd);
        app.add_startup_system(setup_renderdoc)
            .add_system(trigger_capture);

        app
    }
}

fn setup_renderdoc(mut rd: NonSendMut<RenderDocResource>) {
    match rd.as_mut() {
        Ok(rd) => {
            rd.set_log_file_path_template("renderdoc/bevy_capture");
            rd.mask_overlay_bits(OverlayBits::NONE, OverlayBits::NONE);

            info!("Initialized RenderDoc successfully!");
        }
        Err(e) => {
            error!(
                "Failed to initialize RenderDoc. Ensure RenderDoc is installed and visible from your $PATH: {}",
                e
            );
        }
    };
}

fn trigger_capture(
    key: Option<Res<Input<KeyCode>>>,
    rd: NonSend<RenderDocResource>,
    mut replay_pid: Local<usize>,
    mut system: Local<sysinfo::System>,
) {
    if rd.is_err() || key.is_none() {
        return;
    }

    // TODO: If a user were to change this hotkey on the RenderDoc instance
    // this could get mismatched.
    if key.unwrap().just_pressed(KeyCode::F12) {
        // Avoid launching multiple instances of the replay ui
        if system
            .refresh_process_specifics(Pid::from(*replay_pid), ProcessRefreshKind::new().with_cpu())
        {
            return;
        }

        let rd = rd.as_ref().unwrap();
        match rd.launch_replay_ui(true, None) {
            Ok(pid) => {
                *replay_pid = pid as usize;
                info!("Launching RenderDoc Replay UI");
            }
            Err(e) => error!("Failed to launch RenderDoc Replay UI: {}", e),
        }
    }
}
