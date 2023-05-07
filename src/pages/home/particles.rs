use crate::content::{ParticleData, ParticleDatas};
use yew::prelude::*;

pub struct Particles {
    particles: ParticleDatas,
    particle_info: ParticleData,
}

pub enum Msg {
    Clicked(i32),
}

impl Component for Particles {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            particles: ParticleDatas::default(),
            particle_info: ParticleData::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(id) => {
                let part_id: usize = id.try_into().unwrap();
                self.particle_info = self.particles.particles[part_id - 1].clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self { particle_info, .. } = self;

        html! {
            <div class="tile container">
                <div class="tile is-vertical">
                    <div class="tile is-parent pb-3">
                        <div class="tile is-child">
                            <p class="title">{"Standard Model Particles"}</p>
                        </div>
                    </div>

                    <div class="tile is-parent">
                        <div class="tile is-child notification is-primary is-3">
                            <div class="tile is-vertical">
                                {self.view_part_buttons(ctx)}
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

impl Particles {
    // view_part_buttons(ctx) iterate over particles then create button
    // with particle name and callback to update particle info
    fn view_part_buttons(&self, ctx: &Context<Self>) -> Html {
        self.particles
            .particles
            .iter()
            .map(|part| {
                let on_part_select = {
                    let part = part.clone();
                    ctx.link()
                        .callback(move |_| Msg::Clicked(part.part_id.clone()))
                };

                html! {
                    <button class="button is-primary"
                    onclick={on_part_select}>
                        {part.part_name.clone()}
                    </button>
                }
            })
            .collect()
    }
}
