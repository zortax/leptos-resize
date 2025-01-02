use leptos::prelude::*;
use leptos_meta::*;
use leptos_resize::{LayoutDirection, ResizableGrid};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  let col = RwSignal::new(60.);

  view! {
      <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
      <Title text="leptos-resize demo" />
      <Meta charset="UTF-8" />
      <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

      <ResizableGrid col direction=LayoutDirection::Row grid_class="container" handle_class="handle">
        <div class="first" />
        <div class="second" />
      </ResizableGrid>
  }
}
