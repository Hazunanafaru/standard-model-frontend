use yew_router::prelude::*;

pub mod content;
pub mod pages;
pub mod utils;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
}
