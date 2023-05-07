use crate::{content::Author, pages::home::particles::Particles};
use yew::prelude::*;

pub struct Home {
    author: Author,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            author: Author::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-vertical">
                <div class="hero is-ancestor">
                    <div class="hero-body container is-parent pb-2 pu-2">
                        <h1 class="title is-child">
                            { "Welcome to the Standard Model!" }
                        </h1>
                    </div>
                </div>

                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        <figure class="tile image is-child is-3by1">
                            <img
                                alt="Image of Standard Model"
                                src="https://source.unsplash.com/random/1200x400/?abstract" />
                        </figure>
                    </div>
                </div>

                <div class="tile is-ancestor">
                    { self.view_info_tiles() }
                </div>
                <div class="tile is-ancestor">
                    <Particles />
                </div>
                <div class="tile is-ancestor">
                    { self.view_author() }
                </div>
            </div>
        }
    }
}

impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <div class="tile container is-parent">
                <div class="tile is-child">
                    <p class="title">{ "What is Standard Model?" }</p>
                    <p class="subtitle">{ "Standard Model in a nutshell!"}</p>

                    <div class="content">
                        {r#"
                        The Standard Model of particle physics is the theory describing three of the four known fundamental forces (electromagnetic, weak and strong interactions — excluding gravity) in the universe and classifying all known elementary particles.
                        It was developed in stages throughout the latter half of the 20th century, through the work of many scientists worldwide,[1] with the current formulation being finalized in the mid-1970s upon experimental confirmation of the existence of quarks.
                        Since then, proof of the top quark (1995), the tau neutrino (2000), and the Higgs boson (2012) have added further credence to the Standard Model.
                        In addition, the Standard Model has predicted various properties of weak neutral currents and the W and Z bosons with great accuracy.
                        "#}
                    </div>
                </div>
            </div>
        }
    }

    fn view_author(&self) -> Html {
        let Self { author, .. } = self;

        html! {
            <div class="tile container">
                <div class="tile is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child is-light">
                            <p class="title">{"Meet the author"}</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "Interests" }</p>
                                <div class="tags">
                                    { for author.keywords.iter().map(|tag| html! { <span class="tag is-info">{ tag }</span>} )}
                                </div>
                            </article>
                        </div>
                        <div class="tile is-parent">
                            <figure class="tile is-child image">
                                <img class="is-rounded" alt="The author picture." src={author.image_url.clone()} />
                            </figure>
                        </div>
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-info">
                                <div class="content">
                                    <p class="title">{ author.name.clone() }</p>
                                    <div class="content">
                                    {r#"
                                    I'm from Yogyakarta, Indonesia.
                                    I love to learn new things especially related to enginering.
                                    My passion is on technology and science (more to nuclear science).
                                    I hope you enjoy this website ❤️
                                    "#}
                                    </div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
