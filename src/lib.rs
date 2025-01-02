use leptos::{ev, html::Div, prelude::*, text_prop::TextProp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutDirection {
  #[default]
  Row,
  Column,
}

#[component]
pub fn ResizableGrid(
  #[prop(into, optional)] direction: LayoutDirection,
  #[prop(into, optional)] grid_class: TextProp,
  #[prop(into, optional)] handle_class: TextProp,
  #[prop(optional, default = RwSignal::new(50f64))] col: RwSignal<f64>,
  children: Children,
) -> impl IntoView {
  let container = NodeRef::<Div>::new();
  let (is_resizing, set_resizing) = signal(false);

  let on_mouse_down = move |ev: ev::MouseEvent| {
    ev.prevent_default();
    set_resizing.set(true);
  };

  let on_mouse_move = move |ev: ev::MouseEvent| {
    ev.prevent_default();

    if !is_resizing.get() {
      return;
    }

    if let Some(container) = container.get() {
      let bounds = container.get_bounding_client_rect();
      let x = ev.client_x() as f64 - bounds.left();
      let x_percentage = (x / bounds.width()) * 100.0;

      col.set(x_percentage);
    }
  };

  let stop_resize = move |ev: ev::MouseEvent| {
    ev.prevent_default();
    set_resizing.set(false);
  };

  view! {
    <div
      node_ref=container
      class=move || grid_class.get()
      style:position="relative"
      on:mousemove=on_mouse_move
      on:mouseleave=stop_resize
    >
      <div
        class=move || handle_class.get()
        style:position="absolute"
        style:z-index="1"
        style:width="20px"
        style:top="0"
        style:bottom="0"
        style:left=move || format!("calc({:.2}% - 10px)", col.get())
        style:transform="translateX(-50%, 0%)"
        style:cursor="ew-resize"
        on:mousedown=on_mouse_down
        // on:mousemove=on_mouse_move
        on:mouseup=stop_resize
      />
      <div
        style:display="grid"
        style:grid-template-columns=move || format!("{:.2}% {:.2}%", col.get(), 100.0 - col.get())
        style:width="100%"
        style:height="100%"
      >
        {children()}
      </div>
    </div>
  }
}
