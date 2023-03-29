use yew::prelude::*;
use yew_router::prelude::*;

mod secure;
mod home;
mod projects;
mod routes;
mod navbar;

fn switch(routes: routes::Route) -> Html {
    match routes {
        routes::Route::Home => html! { <home::Home /> },
        // routes::Route::Projects => html! { <projects::Projects /> },
        routes::Route::Secure => html! {
            <secure::Secure />
        },
        routes::Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class={classes!(String::from("bg-dark h-full text-white"))}>
                <navbar::Navbar />
                       <Switch <routes::Route> render={switch} /> // <- must be child of <BrowserRouter>
                 </div>
        </BrowserRouter>
    }
}
