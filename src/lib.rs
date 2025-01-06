#![doc = include_str!("../README.md")]

use leptos::{ev, html::Div, prelude::*, text_prop::TextProp};

/// Enum representing the direction of the split.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
  /// Split the container vertically.
  #[default]
  Row,
  /// Split the container horizontally.
  Column,
}

/// A resizable split component. Children will be put into a grid with
/// columns or rows, depending on the direction. The number of children
/// determines the number of splits.
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
///       <div>"Third"</div>
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
  class: TextProp,
  /// The class property for the resize handle.
  #[prop(into, optional)]
  handle_class: TextProp,
  /// The percentages of the splits. Will update on resize.
  #[prop(optional)]
  percentages: Option<RwSignal<Vec<f64>>>,
  children: ChildrenFragment,
) -> impl IntoView {
  let container = NodeRef::<Div>::new();
  let (is_resizing, set_resizing) = signal(false);
  let (resizing_index, set_resizing_index) = signal(None::<usize>);

  // Get the number of children for later calculations.
  let children = children().nodes.into_iter().collect::<Vec<_>>();
  let num_children = children.len();

  // If no `col` signal is provided, create a default one.
  let percentages = percentages.unwrap_or_else(|| {
    let initial_size = 100.0 / num_children as f64;
    RwSignal::new(vec![initial_size; num_children - 1])
  });

  let on_mouse_down = move |index: usize| {
    move |ev: ev::MouseEvent| {
      ev.prevent_default();
      set_resizing.set(true);
      set_resizing_index.set(Some(index));
    }
  };

  let on_mouse_move = move |ev: ev::MouseEvent| {
    ev.prevent_default();

    if !is_resizing.get() {
      return;
    }

    if let (Some(index), Some(container)) =
      (resizing_index.get(), container.get())
    {
      let bounds = container.get_bounding_client_rect();

      let new_percentages = match direction {
        SplitDirection::Row => {
          let x = ev.client_x() as f64 - bounds.left();
          let x_percentage = (x / bounds.width()) * 100.0;
          calculate_new_percentages(percentages.get(), index, x_percentage)
        }
        SplitDirection::Column => {
          let y = ev.client_y() as f64 - bounds.top();
          let y_percentage = (y / bounds.height()) * 100.0;
          calculate_new_percentages(percentages.get(), index, y_percentage)
        }
      };
      percentages.set(new_percentages);
    }
  };

  let stop_resize = move |ev: ev::MouseEvent| {
    ev.prevent_default();
    set_resizing.set(false);
    set_resizing_index.set(None);
  };

  let grid_template_columns = move || match direction {
    SplitDirection::Row => {
      let mut grid_template = String::new();
      for (i, percentage) in percentages.get().iter().enumerate() {
        grid_template.push_str(&format!("{:.2}% ", percentage));
        if i < percentages.get().len() - 1 {
          grid_template.push(' ');
        }
      }
      grid_template.push_str(&format!(
        "{:.2}%",
        100.0 - percentages.get().iter().sum::<f64>()
      ));
      grid_template
    }
    SplitDirection::Column => "100%".into(),
  };

  let grid_template_rows = move || match direction {
    SplitDirection::Row => "100%".into(),
    SplitDirection::Column => {
      let mut grid_template = String::new();
      for (i, percentage) in percentages.get().iter().enumerate() {
        grid_template.push_str(&format!("{:.2}% ", percentage));
        if i < percentages.get().len() - 1 {
          grid_template.push(' ');
        }
      }
      grid_template.push_str(&format!(
        "{:.2}%",
        100.0 - percentages.get().iter().sum::<f64>()
      ));
      grid_template
    }
  };

  let handle_style = move |index: usize| {
    let position = percentages
      .get()
      .iter()
      .take(index + 1)
      .fold(0.0, |acc, &x| acc + x);

    let styles = match direction {
      SplitDirection::Row => {
        format!(
          "cursor: col-resize; width: 20px; transform: translateX(-50%, 0%); \
           top: 0; bottom: 0; left: calc({:.2}% - 10px)",
          position
        )
      }
      SplitDirection::Column => {
        format!(
          "cursor: row-resize; height: 20px; transform: translateY(-50%, 0%); \
           top: calc({:.2}% - 10px); left: 0; right: 0",
          position
        )
      }
    };
    format!("position: absolute; z-index: 1; {}", styles)
  };

  view! {
    <div
      node_ref=container
      class=move || class.get()
      style:position="relative"
      on:mousemove=on_mouse_move
      on:mouseleave=stop_resize
    >
      {(0..num_children - 1)
        .map({
          let handle_class = handle_class.clone();
          move |index| {
            view! {
              <div
                class={
                  let handle_class = handle_class.clone();
                  move || handle_class.get()
                }
                style=move || handle_style(index)
                on:mousedown=on_mouse_down(index)
                on:mouseup=stop_resize
              />
            }
          }
        })
        .collect_view()}
      <div
        style:display="grid"
        style:grid-template-columns=grid_template_columns
        style:grid-template-rows=grid_template_rows
        style:width="100%"
        style:height="100%"
      >
        {children}
      </div>
    </div>
  }
}

/// Calculates the new percentages for the splits based on the current mouse
/// position.
fn calculate_new_percentages(
  current_percentages: Vec<f64>,
  index: usize,
  new_percentage: f64,
) -> Vec<f64> {
  let mut new_percentages = current_percentages.clone();
  if index < new_percentages.len() {
    let prev_sum: f64 = current_percentages.iter().take(index).sum();
    let next_sum: f64 = current_percentages.iter().skip(index + 1).sum();
    let current_percentage = new_percentages[index];

    // Check if the new percentage is within the allowed bounds
    if new_percentage >= prev_sum && new_percentage <= 100.0 - next_sum {
      new_percentages[index] = new_percentage - prev_sum;
      if index < new_percentages.len() - 1 {
        new_percentages[index + 1] +=
          current_percentage - new_percentages[index];
      }
    }
  }
  new_percentages
}
