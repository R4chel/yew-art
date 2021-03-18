use std::collections::VecDeque;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};

use crate::circle::{Circle, ColorConfig, ViewWindow};

pub struct History {
    size: u32,
    elements: VecDeque<Circle>,
}

impl History {
    fn add_element(&mut self, circle: Circle) {
        self.elements.push_back(circle);
        if self.elements.len() as u32 >= self.size {
            let _old_element = self.elements.pop_front();
        }
    }
}

pub enum Status {
    Running(IntervalTask),
    Paused,
}

pub struct App {
    link: ComponentLink<Self>,
    status: Status,
    color_config: ColorConfig,
    max_position_delta: f64,
    view_window: ViewWindow,

    circles: Vec<Circle>,
    history: History,
}

pub enum Msg {
    AddCircle,
    Tick,
    ToggleStatus,
    Save,
}

impl App {
    fn view_circle(circle: &Circle) -> Html {
        html! {
            <circle cx={circle.position.x} cy={circle.position.y} r={circle.radius} fill={circle.color} fill-opacity="0.75" stroke={circle.color} stroke-width="3"/>
        }
    }

    fn view_status_button(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::ToggleStatus)>{match self.status { Status::Running(_) => "⏸",
                                                                                           Status::Paused => "▶️"
            }
            }</button>

        }
    }

    fn view_image(&self) -> Html {
        html! {

            <svg id="svg" width={self.view_window.x_max} height={self.view_window.y_max} viewBox={format!("{} {} {} {}", self.view_window.x_min, self.view_window.y_min, self.view_window.x_max, self.view_window.y_max)} fill="none" xmlns="http://www.w3.org/2000/svg" style="display:block">

            { self.history.elements.iter().map(App::view_circle).collect::<Html>() }
            { self.circles.iter().map(App::view_circle).collect::<Html>() }

            </svg>
        }
    }
    fn view_app(&self) -> Html {
        html! {
                    <div>
                        <button onclick=self.link.callback(|_| Msg::AddCircle)>{"+"}</button>
                        <button onclick=self.link.callback(|_| Msg::Tick)>{"🦶"}</button>

                <button onclick=self.link.callback(|_| Msg::Save)>{"💾"} </button>

        { self.view_status_button() }
            {self.view_image()}
                        </div>
                }
    }

    pub fn tick(&mut self) -> () {
        for circle in self.circles.iter_mut() {
            let clone = circle.clone();
            self.history.add_element(clone);
            circle.update(
                &self.view_window,
                self.max_position_delta,
                &self.color_config,
            );
        }
    }

    pub fn add_circle(&mut self) -> () {
        self.circles.push(Circle::rand(
            &self.color_config,
            self.view_window.random_position(),
        ))
    }

    pub fn save(&self) -> () {
        // UGGG trying to get something to work

        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let document = yew::utils::document();

        let svg = document.get_element_by_id("svg").unwrap();

        let xml_serializer = web_sys::XmlSerializer::new().unwrap();
        let svg_buf = xml_serializer.serialize_to_string(&svg).unwrap();
        let mut blob_type = web_sys::BlobPropertyBag::new();
        blob_type.type_("image/svg+xml;charset=utf-8");

        let arr = js_sys::Array::new_with_length(1);
        arr.set(0, JsValue::from_str(&svg_buf));

        let blob =
            web_sys::Blob::new_with_str_sequence_and_options(&JsValue::from(arr), &blob_type)
                .unwrap();

        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

        // html! {
        //     <a href={url} download="yew-art.svg"></a>
        // }

        let anchor = document
            .create_element("a")
            .unwrap()
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .unwrap();

        anchor.set_href(&url);
        anchor.set_download("yew-art.svg");
        anchor.click();
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let view_window = ViewWindow {
            x_min: 0.0,
            x_max: 750.0,
            y_min: 0.0,
            y_max: 750.0,
        };
        let mut app = App {
            link,
            view_window,
            status: Status::Paused,
            color_config: ColorConfig::default_2(),
            max_position_delta: 20.0,
            circles: vec![],
            history: History {
                size: 10000,
                elements: VecDeque::new(),
            },
        };
        app.add_circle();
        app
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddCircle => {
                self.add_circle();
                true
            }
            Msg::Tick => {
                self.tick();
                true
            }
            Msg::ToggleStatus => {
                match &self.status {
                    Status::Running(task) => {
                        drop(task);
                        self.status = Status::Paused;
                    }
                    Status::Paused => {
                        let task = IntervalService::spawn(
                            Duration::from_millis(30),
                            self.link.callback(|_| Msg::Tick),
                        );

                        self.status = Status::Running(task);
                    }
                };
                true
            }

            Msg::Save => {
                self.save();
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.view_app()
    }
}
