use frontend::pages::{
    about::Author, home::Home, page_not_found::PageNotFound, particles::Particles,
};
use frontend::Route;
use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

// Message to store ToggleNavbar
pub enum Msg {
    ToggleNavbar,
}

// The base App Struct
pub struct App {
    navbar_active: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={switch} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { "." }
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}

impl App {
    // Method to create Navigation View when navbar_active is true
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main-navigation">
                <div class="navbar-brand">
                    <Link<Route> classes={classes!("navbar-item", "is-size-3")} to={Route::Home}>
                        { "Standard Model" }
                    </Link<Route>>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}>

                        <span aria-hiden="true"></span>
                        <span aria-hiden="true"></span>
                        <span aria-hiden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Creation}>
                            { "Creation" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Particles}>
                            { "Particles" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::About}>
                            { "About" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home />}
        }
        Route::Particles => {
            html! { <Particles />}
        }
        Route::Creation => {
            html! { <PageNotFound />}
        }
        Route::About => {
            html! { <Author />}
        }
        Route::NotFound => {
            html! { <PageNotFound />}
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
