# jscore (in development)
`jscore` is a relatively safe bindings to [JavaScriptCore](https://developer.apple.com/documentation/javascriptcore) written by Apple.

It is currently in development and is not meant for production use.

## Building
The build script (`build.rs`) is a Rust port to [Bun's CMake script](https://github.com/oven-sh/bun/blob/main/cmake/tools/SetupWebKit.cmake).
While build tests are successful, it might not build on your machine due to the lack of libraries. If you encounter any problem, feel free to [create/view issues](https://github.com/AWeirdDev/jscore/issues) because other people might experience the same!

I do have to admit that I co-created the build script with the help of artificial intelligence, meaning the code quality may be concerning.
