use yew::prelude::*;
mod farmer_icon;

#[derive(Properties, PartialEq)]
pub struct SvgProps {
    pub class: String,
    pub src: String,
}

#[function_component(Home)]
pub fn home_layout() -> Html {
    let container_one = classes!(
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "min-h-screen"
    );
    let heading_text = classes!("text-xl", "float-left", "header-text", "text-white");
    let headline_text_titles = classes!(
        "bg-accent",
        "p-4",
        "rounded-lg",
        "shadow-md",
        "w-most",
        "default-text"
    );
    let intro_headline = classes!(
        "block",
        "font-sans",
        "text-base",
        "font-light",
        "uppercase",
        "leading-relaxed",
        "text-inherit",
        "antialiased"
    );
    let intro_padding = classes!("pt-5");
    let para_text = classes!(
        String::from(
            "p-8 block font-sans text-sm font-light leading-normal text-inherit antialiased"
        )
    );

    let _rust_svg_props = SvgProps {
        class: String::from("h-6 w-6"),
        src: String::from(
            "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1200px-Rust_programming_language_black_logo.svg.png"
        ),
    };

    let _wasm_svg_props = SvgProps {
        class: String::from("h-6 w-6"),
        src: String::from(
            "https://upload.wikimedia.org/wikipedia/commons/thumb/9/96/WebAssembly_Logo.svg/1200px-WebAssembly_Logo.svg.png"
        ),
    };

    html! {
        <>
        <div class={container_one}>
        //
        <div class={classes!(String::from("m-min w-most"))}>
        <strong class={heading_text}>{"HELLO ✌"} </strong>
        </div>
        //
        <div class={headline_text_titles}>
        {"=> return yum::Intro::new();"}
        </div>
        //
        <p class={classes!(intro_headline.clone(), intro_padding)}>
        {"[full stack developer] .. 25% .. 50% .. 100% -> ✅"}
        </p>
        <p class={intro_headline.clone()}>
        {"systems engineer .. 24% .. 69% .. 100% -> ✅"}
        </p>
        <p class={intro_headline.clone()}>
        {"(fake it till i make it) .. 24% .. 69% .. 420% -> ✅"}
        </p>
        //
        <p class={classes!(String::from("text-center m-8 underline"))}>
        {"-> with a newfound passion for <-"}
        </p>
        //
        <div class={classes!(String::from("uppercase header-text m-2 text-white"))}>
        {"{"}
        <strong class={classes!(String::from("uppercase p-5 rounded-lg shadow-md header-text m-5 bg-accent"))}>
        {"Rust"}</strong>
        {"&&"}
        <strong class={classes!(String::from("uppercase p-5 rounded-lg shadow-md header-text m-5 bg-accent"))}>
        {"WebAssembly"}</strong>
        {"}"}
           </div>
           //
           <article class={classes!(String::from("prose mt-8"))}>
        <p class={para_text}>
        {farmer_icon::farmer_meme()} <br/>
        <p class="text-center">{"It aint much... but its honest work."}</p>
         </p>
        </article>
        </div>
        </>
    }
}
