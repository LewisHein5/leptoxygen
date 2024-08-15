use leptos::*;

#[component]
pub fn Zoom(inn: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <div class="leptoxygen-zoom" data-in=move || inn.get()>
            { children() }
        </div>
    }
}
