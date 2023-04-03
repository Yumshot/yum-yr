use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/yum-yr/projects")]
    Projects,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/yum-yr/404")]
    NotFound,
}