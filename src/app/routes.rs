use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
//RELEASE ENUM
pub enum Route {
    #[at("/yum-yr/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}
// DEBUG / LOCAL ENUM
// pub enum Route {
//     #[at("/")]
//     Home,
//     #[at("/projects")]
//     Projects,
//     #[at("/secure")]
//     Secure,
//     #[not_found]
//     #[at("/404")]
//     NotFound,
// }