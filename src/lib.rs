use leptos::{ev, html::Div, prelude::*, text_prop::TextProp};

/// Enum representing the direction of the split.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
  /// Split the container horizontally.
  #[default]
  Row,
  /// Split the container vertically.
  Column,
}

/// A resizable split component. Children will be put into a grid with two
/// columns or rows, depending on the direction.
///
/// # Example:
/// ```
/// use leptos_resize::ResizableSplit;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///   view! {
///     <ResizableSplit>
///       <div>"First"</div>
///       <div>"Second"</div>
///     </ResizableSplit>
///   }
/// }
/// ```
#[component]
pub fn ResizableSplit(
  /// The direction of the split.
  #[prop(into, optional)]
  direction: SplitDirection,
  /// The class property for the underlying grid.
  #[prop(into, optional)]
  grid_class: TextProp,
  /// The class property for the resize handle.
  #[prop(into, optional)]
  handle_class: TextProp,
  /// The percentage of the first split half. Will update on resize.
  #[prop(optional, default = RwSignal::new(50f64))]
  col: RwSignal<f64>,
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

      col.set(match direction {
        SplitDirection::Row => {
          let x = ev.client_x() as f64 - bounds.left();
          (x / bounds.width()) * 100.0
        }
        SplitDirection::Column => {
          let y = ev.client_y() as f64 - bounds.top();
          (y / bounds.height()) * 100.0
        }
      });
    }
  };

  let stop_resize = move |ev: ev::MouseEvent| {
    ev.prevent_default();
    set_resizing.set(false);
  };

  let grid_template_columns = move || match direction {
    SplitDirection::Row => {
      format!("{:.2}% {:.2}%", col.get(), 100.0 - col.get())
    }
    SplitDirection::Column => "100%".into(),
  };

  let grid_template_rows = move || match direction {
    SplitDirection::Row => "100%".into(),
    SplitDirection::Column => {
      format!("{:.2}% {:.2}%", col.get(), 100.0 - col.get())
    }
  };

  let handle_style = move || {
    let styles = match direction {
      SplitDirection::Row => {
        format!(
          "cursor: col-resize; width: 20px; transform: translateX(-50%, 0%); \
           top: 0; bottom: 0; left: calc({:.2}% - 10px)",
          col.get()
        )
      }
      SplitDirection::Column => {
        format!(
          "cursor: row-resize; height: 20px; transform: translateY(-50%, 0%); \
           top: calc({:.2}% - 10px); left: 0; right: 0",
          col.get()
        )
      }
    };
    format!("position: absolute; z-index: 1; {}", styles)
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
        style=handle_style
        on:mousedown=on_mouse_down
        on:mouseup=stop_resize
      />
      <div
        style:display="grid"
        style:grid-template-columns=grid_template_columns
        style:grid-template-rows=grid_template_rows
        style:width="100%"
        style:height="100%"
      >
        {children()}
      </div>
    </div>
  }
}
