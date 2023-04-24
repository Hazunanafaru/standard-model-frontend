use yew::prelude::*;
use crate::content;

pub struct About {
    about: content::About,
}
impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>, ) -> Self {
        Self {
            about: content::About {
                name: "Husni Naufal Zuhdi".to_string(),
                keywords: vec![
                    "Kubernetes".to_string(),
                    "Rust".to_string(),
                    "DevOps".to_string(),
                    "WebAssembly".to_string(),
                    ],
                image_url: "https://avatars.githubusercontent.com/u/35314346".to_string(),
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self {about} = self;
    
        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &about.name }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "interests" }</p>
                                <div class="tags">
                                    { for about.keywords.iter().map(|tag| html! { <span class="tag is-info">{ tag }</span>} )}
                                </div>
                            </article>
                        </div>
                        <div class="tile is-parent">
                            <figure class="tile is-child image">
                                <img class="is-rounded" alt="The author picture." src={about.image_url.clone()} />
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