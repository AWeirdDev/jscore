# jscore (in development)
`jscore` is a relatively safe bindings to [JavaScriptCore](https://developer.apple.com/documentation/javascriptcore) written by Apple.

It is currently in development and is not yet meant for production use.

Feel free to contribute! I finally learned how to use git.

```rust
use jscore::prelude::*;

// Create a context group for managing contexts
let group = ContextGroup::new();

// Create the global context for JS interactions
let global = group.create_global_context();
let ctx = &global.as_context();

// Create a script
let content = JsString::new("(() => 'hello from js!')()");
let script = Script::builder(ctx).script(&content).build();

let result = script.evaluate(ctx).expect("failed to run script");
let result = result
    .to_string_copy(ctx)
    .unwrap()
    .to_rust_string()
    .unwrap();

println!("{result}");
// Output: hello from js!
```

## Building
The build script (`build.rs`) is a Rust port to [Bun's CMake script](https://github.com/oven-sh/bun/blob/main/cmake/tools/SetupWebKit.cmake).
While build tests are successful, it might not build on your machine due to the lack of libraries. If you encounter any problem, feel free to [create/view issues](https://github.com/AWeirdDev/jscore/issues) because other people might experience the same!

**Platform-specific behaviors**:

- **macOS**: It tries to use the macOS Framework. If nothing is found, it downloads WebKit and builds from it.
- **Linux**: It tries to find existing libraries (`javascriptcoregtk-4.0`, `javascriptcoregtk-4.1`). If nothing is found, it downloads WebKit and builds from it.
- **Windows**": It downloads WebKit. Since we need ICU on Windows, it builds ICU from source, and then builds.

<br />

**Downloading WebKit**:

The script downloads releases from [oven-sh/WebKit] either with `curl` or `python3`, depending on what's available.


### AI notice
I do have to admit that I co-created the build script with the help of artificial intelligence as I do not have this area of expertise, meaning the code quality may be concerning.
However, I do want to point out that I don't agree with mass, unprocessed & unaudited AI use, and I did not use AI when creating the Rust bindings; they're hand written.
