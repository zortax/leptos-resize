# Leptos Resize

![Preview](https://github.com/zortax/leptos-resize/blob/main/preview.gif?raw=true)

This crate provides a simple user-resizable split container for the Leptos web
framework. Horizontal and vertical splits are both supported.

```rust
fn MyComponent() -> impl IntoView {
  view! {
    <ResizableSplit>
      <div>"First"</div>
      <div>"Second"</div>
    </ResizableSplit>
  }
}
```
(See `examples` directory for a full CSR example)

## Leptos compatibility

| Crate version | Compatible Leptos version |
|---------------|---------------------------|
| 0.1           | 0.7                       |

## Features

- Horizontal and vertical split
- Split ration can be bound to `RwSignal`
- works with server-side rendering
