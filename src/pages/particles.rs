use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

use crate::content::ParticleData;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Particles {
    particles: Vec<ParticleData>,
}

pub enum Msg {
    FetchParticles(JsValue),
    FetchFailed(JsValue),
}

// DISABLE TILL I FOUND OUT HOW TO FETCH API VIA YEW OR WEB_SYS OR WASM_BINDGEN
#[wasm_bindgen]
pub async fn fetch_particles(link_url: &str) -> Result<JsValue, JsValue> {
    // Start request
    log!(JsValue::from("Start request"));
    let mut opts = RequestInit::new();

    // Setup options
    log!(JsValue::from("Setup options"));
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // Send a request
    log!(JsValue::from("Send a request"));
    let request = Request::new_with_str_and_init(link_url, &opts)?;

    // Configure our result
    log!(JsValue::from("Configure our result"));
    let window = window().unwrap();
    let resp_js_value = JsFuture::from(window.fetch_with_request(&request)).await;

    log!(JsValue::from("Return the Result"));
    create_particles();
    resp_js_value

    // assert!(resp_js_value.is_instance_of::<Response>());
    // let resp: Response = resp_js_value.dyn_into().unwrap();

    // // Convert this other `Promise` into a rust `Future`.
    // let json = JsFuture::from(resp.json()?).await?;

    // // Send the JSON response back to JS.
    // Ok(json)
}

impl Component for Particles {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link_url = "http://localhost:3000/particles";
        ctx.link().send_future(async {
            match fetch_particles(link_url).await {
                Ok(image) => {
                    log!(JsValue::from("image"));
                    log!(image.clone());
                    Msg::FetchParticles(image)
                }
                Err(err) => Msg::FetchFailed(err),
            }
        });
        Self {
            particles: vec![ParticleData {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchParticles(json) => {
                log!(JsValue::from("Fetch Particles"));
                match from_value::<Particles>(json) {
                    Ok(data) => {
                        for particle in data.particles.iter() {
                            log!(JsValue::from(particle.part_name.clone()))
                        }
                        // let sample_name = data.particles[0].part_name.clone();
                        log!(JsValue::from("sample_name"));
                        // log!(&JsValue::from(sample_name));
                        // self.particles = data.particles;
                        true
                    }
                    Err(e) => {
                        log!(JsValue::from("Error"));
                        log!(e);
                        true
                    }
                }
                // let data: Vec<ParticleData> = from_value(json).unwrap();
                // let sample_name = data[1].part_name.clone();
                // log!(JsValue::from("sample_name"));
                // log!(&JsValue::from(sample_name));
                // self.particles = data;
                // true
            }
            Msg::FetchFailed(err) => {
                log!(JsValue::from("fetched_failed"));
                log!(err);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let particles = self.particles.clone();
        for particle in particles.iter() {
            println!("{:?}", &particle.part_id);
            log!(JsValue::from(&particle.charge));
        }

        html! {
            <ul class="item-list">
                { &particles[0].part_name.to_string() }
            </ul>
        }
    }
}

pub fn create_particles() -> Vec<ParticleData> {
    let file = std::fs::File::open("../data/particles.csv").unwrap();
    // let reader = csv::Reader::from_reader(file);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    for result in reader.records() {
        let rcd = result.unwrap();
        let record = rcd.as_slice();
        log!(JsValue::from(record));
        // println!("{:?}", record);
    }
    vec![]
}
