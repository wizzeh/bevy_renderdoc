# bevy-renderdoc

Bevy plugin for [RenderDoc], wrapping [renderdoc-rs].

[RenderDoc]: https://renderdoc.org/
[renderdoc-rs]: https://github.com/ebkalderon/renderdoc-rs

This plugin requires that RenderDoc be installed on the target machine, with
either `renderdoc.dll` or `librenderdoc.so` visible from your `$PATH`.

## Example

```rust
use bevy::prelude::*;
use bevy_renderdoc::*;

fn main() {
    App::with_renderdoc()
        .add_plugins(DefaultPlugins)
        .run();
}
```
Check the [examples](/examples) for more working examples.

## License

`bevy-renderdoc` is free and open source software distributed under the terms of
either the [MIT](LICENSE-MIT) or the [Apache 2.0](LICENSE-APACHE) license, at
your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.