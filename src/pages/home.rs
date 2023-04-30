use crate::{
    content::{Author, ParticleDatas},
    utils::csv::read_author_csv,
};
use yew::prelude::*;

pub struct Home {
    author: Author,
    particles: ParticleDatas,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            author: read_author_csv().unwrap_or_else(|_| Author::default()),
            particles: ParticleDatas::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="hero is-parent">
                    <div class="hero-body container pb-3">
                        <h1 class="title">
                            { "Welcome to the Standard Model!" }
                        </h1>
                    </div>
                </div>

                <div class="tile is-parent pu-3">
                    <div class="tile is-child">
                        <figure class="image is-3by1">
                            <img
                                alt="Image of Standard Model"
                                src="https://source.unsplash.com/random/1200x400/?abstract" />
                        </figure>
                    </div>
                </div>

                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>
                <div class="tile is-parent container">
                    { self.view_particles() }
                </div>
                <div class="tile is-parent container">
                    { self.view_author() }
                </div>
            </div>
        }
    }
}

impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <div class="tile is-child box">
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
        }
    }

    fn view_particles(&self) -> Html {
        let Self { particles, .. } = self;

        html! {
            // <div class="section container is-vertical">
            //     <div class="tile pb-3">
            //         <article class="tile is-child box has-text-centered">
            //             <p class="title">{"Standard Model Particles"}</p>
            //         </article>
            //     </div>
            //     <div class="tile pu-3">
            //         <div class="tile">
            //             <article class="tile box is-child notification has-text-centered">
            //                 <ul class="item-list">
            //                     {
            //                         for particles.particles.iter().map(
            //                             |(_, particle)| html! {
            //                                 <li class="pu-3 pb-3">{particle.part_name.clone() }</li>
            //                             })
            //                     }
            //                 </ul>
            //             </article>
            //         </div>
            //         <div class="tile">
            //             <article class="tile box is-child notification has-text-centered">
            //                 {"Particle Info Placeholder"}
            //             </article>
            //         </div>
            //     </div>
            // </div>
            <div class="section container">
                <div class="tile is-vertical is-child">
                    <div class="tile pb-3">
                        <article class="tile box has-text-centered">
                            <p class="title">{"Standard Model Particles"}</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile box notification is-4">
                            <article class="tile">
                                <ul class="item-list">
                                    {
                                        for particles.particles.iter().map(
                                            |(_, particle)| html! {
                                                <li class="pu-3 has-text-centered">
                                                {particle.part_name.clone()}
                                                </li>
                                            })
                                    }
                                </ul>
                            </article>
                        </div>
                        <div class="tile box notification has-text-centered is-8">
                            <article class="tile">
                                {"Particle Info Placeholder"}
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn view_author(&self) -> Html {
        let Self { author, .. } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child is-light box has-text-centered">
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
