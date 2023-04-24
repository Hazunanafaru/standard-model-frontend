use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
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
}