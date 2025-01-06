# Leptos Resize

![Preview](https://github.com/zortax/leptos-resize/blob/main/preview.gif?raw=true)

This crate provides a simple user-resizable split container for the Leptos web
framework. Horizontal and vertical splits are both supported.

```rust
#[component]
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
| 0.1 - 0.2     | 0.7                       |

## Features

- Horizontal and vertical split
- Split ratio can be bound to `RwSignal`
- Works with server-side rendering
- Arbitrary number of splits
- Nested splits

### Split direction

The split direction can be changed by setting the `direction` property.

```rust
#[component]
fn MyComponent() -> impl IntoView {
  view! {
    // split the container horizontally
    <ResizableSplit direction=SplitDirection::Column>
      <div>"First"</div>
      <div>"Second"</div>
    </ResizableSplit>
  }
}
```

### Bind size percentages to signal

The size percentages can be bound to a `RwSignal` by setting the `percentages`
property. The `RwSignal` should contain a `Vec` with one element less than the
amount of children you pass. The last percentage will always be calculated.
The values sum should be less than `100`.

```rust
#[component]
fn MyComponent() -> impl IntoView {
  // the last childs size will be calculated (in this case 40.)
  let percentages = RwSignal::new(vec![20., 40.]);

  view! {
    <ResizableSplit percentages>
      <div>"First"</div>
      <div>"Second"</div>
      <div>"Third"</div>
    </ResizableSplit>
  }
}
```

### Nest multiple split containers

The `<ResizableSplit>` container can also be nested to create more complex
resizable layouts.

```rust
#[component]
fn MyComponent() -> impl IntoView {
  view! {
    <ResizableSplit>
      <div>"First"</div>
      <ResizableSplit direction=SplitDirection::Column>
        <div>"Second"</div>
        <div>"Third"</div>
      </ResizableSplit>
    </ResizableSplit>
  }
}
```
