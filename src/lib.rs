use yew_router::prelude::*;

pub mod components;
pub mod content;
pub mod pages;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/creation")]
    Creation,
    #[at("/particles")]
    Particles,
    #[not_found]
    #[at("/404")]
    NotFound,
}
