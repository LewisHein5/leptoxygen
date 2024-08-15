use leptos::*;

use crate::hooks::{
    anchor_link::{use_anchor_link, Href, UseAnchorLinkReturn},
    prelude::{UseAnchorLinkInput, UsePressInput},
};

#[component]
pub fn AnchorLink(
    /// The anchor link. For example: "#my-anchor".
    #[prop(into)]
    href: Oco<'static, str>,

    #[prop(into, optional)]
    scroll_behavior: Option<web_sys::ScrollBehavior>,

    /// Description of this anchor for accessibility.
    /// If text is provided in children, this could be omitted.
    /// If no children are provided, this component renders a single `#`,
    /// which should be described using this field.
    #[prop(into, optional)]
    description: Option<Oco<'static, str>>,

    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,

    /// If no children are provided, this component renders a single `#` character.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // We make links "use_press", so that optional PressResponder's higher up the component tree can react on link interactions
    // and so that a custom `on_press` handler can immediately work with the underlying link element.
    let UseAnchorLinkReturn { props } = use_anchor_link(UseAnchorLinkInput {
        href: Href::from_str(href).expect("valid href"),
        scroll_behavior: scroll_behavior.or(Some(web_sys::ScrollBehavior::Smooth)),
        disabled: false.into(),
        description,
        use_press_input: UsePressInput {
            // Links cannot be disabled (for now).
            disabled: false.into(),
            // We are using an <a> tag and want the custom scrolling behavior from `use_anchor_link`.
            // If we would not enforce prevention of default behavior, automatic browser scroll-jumps would occur.
            force_prevent_default: true,
            on_press: Callback::new(move |_| {}),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
    });

    view! {
        <a
            {..props.attrs}
            id=id
            class=class
            style=style
            class="leptoxygen-anchor-link"
            target="_self"
            on:keydown=props.on_key_down
            on:click=props.on_click
            on:pointerdown=props.on_pointer_down
        >
            { match children {
                Some(children) => children().into_view(),
                None => "#".into_view(),
            } }
        </a>
    }
}
