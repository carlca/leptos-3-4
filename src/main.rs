use leptos::*;

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
  #[prop(default = 100)] max: u16,
  #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
  view! {
    <progress max=max value=progress></progress>
    <br/>
  }
}

#[component]
fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let double_count = move || count() * 2;

  view! {
    <button on:click=move |_| { set_count.update(|n| *n += 1); }>
      "Click me"
    </button>
    <br/>
    <ProgressBar max=50 progress=count/>
    <ProgressBar progress=count/>
    <ProgressBar max=50 progress=Signal::derive(double_count)/>
  }
}

fn main() {
  leptos::mount_to_body(App)
}
