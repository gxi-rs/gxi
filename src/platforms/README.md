# Platforms

To achieve a stable standard api for all platforms. Each platform needs to export
certain modules, which include.

//TODO:

- `run()` function

- `NativeWidget`
  Smallest element of the native widget system which can be added to other
  widgets but it itself may or may not have the ability to hold a child

- `NativeContainer`
  An element of the native widget system which can hold a child

- `Element`
  default method of constructing a node, needs to have `&self` fields.
