use yew_router::prelude::*;

pub mod content;
pub mod pages;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
}
