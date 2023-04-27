use crate::content;
use yew::prelude::*;

pub struct Home {
    author: content::Author,
    particles: Vec<content::ParticleData>,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            author: content::Author {
                name: "Husni Naufal Zuhdi".to_string(),
                keywords: vec![
                    "Kubernetes".to_string(),
                    "Rust".to_string(),
                    "DevOps".to_string(),
                    "WebAssembly".to_string(),
                ],
                image_url: "https://avatars.githubusercontent.com/u/35314346".to_string(),
            },
            particles: vec![content::ParticleData {
                part_id: 2,
                part_type: "quark".to_string(),
                part_name: "down".to_string(),
                mass: 4700000,
                charge: "-1/3".to_string(),
                spin: "1/2".to_string(),
                // image_url: "test".to_string(),
                created_at: chrono::Local::now().naive_local(),
                updated_at: chrono::Local::now().naive_local(),
            }],
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="hero is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Welcome..." }</h1>
                        <h2 class="subtitle">{ "...to the Standard Model!" }</h2>
                    </div>
                </div>

                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img alt="Image of Standard Model" src="https://source.unsplash.com/random/1200x400/?abstract" />
                    </figure>
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
            <>
                <div class="tile is-parent">
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
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Who am I?"}</p>

                        <div class="content">
                            {r#"
                            I'm Husni from Yogyakarta, Indonesia.
                            A software and former system engineer who like to learn a new thing.
                            This website is a result of me learning many new tech included:
                            "#}
                            <ul>
                                <li>{ "🏗 Kubernetes" }</li>
                                <li>{ "⛵️ Istio" }</li>
                                <li>{ "🦀 Rust (Actix Web and Yew)" }</li>
                                <li>{ "🖥 Wasm" }</li>
                            </ul>
                            {r#"
                            I hope you enjoy this website 🫶
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn view_particles(&self) -> Html {
        let Self { particles, .. } = self;

        html! {
            <ul class="item-list">
                { &particles[0].part_name.to_string() }
            </ul>
        }
    }
    fn view_author(&self) -> Html {
        let Self { author, .. } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &author.name }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "interests" }</p>
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
                                    <p class="title">{ "About me" }</p>
                                    <div class="content">
                                    {r#"
                                    I'm Husni from Yogyakarta, Indonesia.
                                    A software and former system engineer who like to learn a new thing.
                                    This website is a result of me learning many new tech included:
                                    "#}
                                    <ul>
                                        <li>{ "🏗 Kubernetes" }</li>
                                        <li>{ "⛵️ Istio" }</li>
                                        <li>{ "🦀 Rust (Actix Web and Yew)" }</li>
                                        <li>{ "🖥 Wasm" }</li>
                                    </ul>
                                    {r#"
                                    I hope you enjoy this website 🫶
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
