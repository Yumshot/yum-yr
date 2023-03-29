use yew::prelude::*;

#[function_component(Secure)]
pub fn secure() -> Html {
    let container_one = classes!(
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "min-h-screen"
    );
    html!(<div class={container_one}>{"Secure"}</div>)
}