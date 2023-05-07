use crate::content::{ParticleData, ParticleDatas};
use yew::prelude::*;

pub struct Particles {
    particles: ParticleDatas,
    particle_info: ParticleData,
    test_id: i32,
}

pub enum Msg {
    Clicked(String),
}

impl Component for Particles {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            particles: ParticleDatas::default(),
            particle_info: ParticleData::default(),
            test_id: 0_i32,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(id) => {
                let part_id: usize = id.parse().unwrap_or_else(|_| 0_usize);
                self.particle_info = self.particles.particles[part_id].clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            particles,
            particle_info,
            ..
        } = self;

        // let onclick = ctx
        //     .link()
        //     .callback(|part_id: AttrValue| Msg::Clicked(part_id));

        html! {
            <div class="tile container">
                <div class="tile is-vertical">
                    <div class="tile is-parent pb-3">
                        <div class="tile is-child">
                            <p class="title">{"Standard Model Particles"}</p>
                        </div>
                    </div>

                    <div class="tile is-parent">
                        <div class="tile is-child notification is-primary is-2">
                            <div class="tile is-vertical">
                                <button class="button is-primary"
                                    onclick={ctx.link().callback(|_| Msg::Clicked("0".to_string()))}
                                >
                                    {&particles.particles[0].part_name}
                                </button>
                                <button class="button is-primary"
                                    onclick={ctx.link().callback(|_| Msg::Clicked("1".to_string()))}
                                >
                                    {&particles.particles[1].part_name}
                                </button>
                                <button class="button is-primary"
                                    onclick={ctx.link().callback(|_| Msg::Clicked("2".to_string()))}
                                >
                                    {&particles.particles[2].part_name}
                                </button>
                                <button class="button is-primary"
                                    onclick={ctx.link().callback(|_| Msg::Clicked("3".to_string()))}
                                >
                                    {&particles.particles[3].part_name}
                                </button>
                            </div>
                        </div>
                        <div class="tile is-child notification is-link has-text-centered">
                            <figure class="image container is-128x128 mb-6">
                                <img class="is-rounded" src="https://upload.wikimedia.org/wikipedia/commons/thumb/6/6f/Stylised_atom_with_three_Bohr_model_orbits_and_stylised_nucleus.svg/530px-Stylised_atom_with_three_Bohr_model_orbits_and_stylised_nucleus.svg.png" />
                            </figure>
                            <p class="title">
                                {&particle_info.part_name}
                                {" "}
                                {&particle_info.part_type}
                            </p>

                            <div class="columns">
                                <div class="column">
                                    <p class="subtitle">{"Particle Mass"}</p>
                                    <p class="subtitle">{ &particle_info.mass }{" eV"}</p>
                                </div>
                                <div class="column">
                                    <p class="subtitle">{"Particle Charge"}</p>
                                    <p class="subtitle">{ &particle_info.charge }</p>
                                </div>
                                <div class="column">
                                    <p class="subtitle">{"Particle Spin"}</p>
                                    <p class="subtitle">{ &particle_info.spin }</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
