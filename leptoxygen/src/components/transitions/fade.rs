use leptos::*;

#[component]
pub fn Fade(#[prop(into)] inn: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <div class="leptoxygen-fade" data-in=move || inn.get()>
            { children() }
        </div>
    }
}
