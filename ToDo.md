- ? add generic WinRT interfaces with signatures for selected concrete types
- ? replace embedding .winmd at compile time with reading at runtime
- rustdocs queries: wait until `rustdoc_types::Type` support is added to [trustfall-rustdoc-adapter](https://github.com/obi1kenobi/trustfall-rustdoc-adapter) and replace getting constant type via ID with a direct query of a `type` property

- ? add AHK types here or do it later via lookup? (do it later)
  - then you can have a single dll call that gets both type and value and use it (in a tuple or 2 vars??)

- convert `::core::ptr::invalid_mut(#value as _)` to actual value, though doesn't seem to appear in the output
- 