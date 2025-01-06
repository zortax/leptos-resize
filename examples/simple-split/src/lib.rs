use leptos::prelude::*;
use leptos_meta::*;
use leptos_resize::{ResizableSplit, SplitDirection};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  let percentages = RwSignal::new(vec![20., 40.]);

  view! {
      <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
      <Title text="leptos-resize demo" />
      <Meta charset="UTF-8" />
      <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

      <ResizableSplit class="container" handle_class="handle">
        <div class="first">
          <h2>"Lorem ipsum dolor sit amet"</h2>
          <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."</p>
        </div>
        <div class="second" />
      </ResizableSplit>

      <ResizableSplit class="container" handle_class="handle" percentages>
        <div class="first">
          <h2>"Lorem ipsum dolor sit amet"</h2>
          <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."</p>
        </div>
        <div class="second" />
        <div class="third" />
      </ResizableSplit>

      <ResizableSplit class="container" handle_class="handle" direction=SplitDirection::Column>
        <div class="first">
          <h2>"Lorem ipsum dolor sit amet"</h2>
          <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."</p>
        </div>
        <div class="second" />
      </ResizableSplit>

      <ResizableSplit class="container" handle_class="handle">
        <div class="first">
          <h2>"Lorem ipsum dolor sit amet"</h2>
          <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."</p>
        </div>
        <ResizableSplit handle_class="handle" direction=SplitDirection::Column>
          <div class="second" />
          <div class="third" />
        </ResizableSplit>
      </ResizableSplit>
  }
}
