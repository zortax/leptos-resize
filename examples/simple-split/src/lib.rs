use leptos::prelude::*;
use leptos_meta::*;
use leptos_resize::ResizableSplit;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
      <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
      <Title text="leptos-resize demo" />
      <Meta charset="UTF-8" />
      <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

      <ResizableSplit grid_class="container" handle_class="handle">
        <div class="first">
          <h2>"Lorem ipsum dolor sit amet"</h2>
          <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."</p>
        </div>
        <div class="second" />
        <div class="third" />
      </ResizableSplit>
  }
}
