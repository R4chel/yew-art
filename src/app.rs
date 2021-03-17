use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};

use crate::circle::{Circle, ColorConfig, ViewWindow};

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
    history: Vec<Circle>,
}

pub enum Msg {
    AddCircle,
    Tick,
    ToggleStatus,
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

    fn view_app(&self) -> Html {
        html! {
                    <div>
                        <p>{ "Hello world!" }</p>
                        <button onclick=self.link.callback(|_| Msg::AddCircle)>{"+"}</button>
                        <button onclick=self.link.callback(|_| Msg::Tick)>{"🦶"}</button>
        { self.view_status_button() }
                        <svg width={self.view_window.x_max} height={self.view_window.y_max} viewBox={format!("{} {} {} {}", self.view_window.x_min, self.view_window.y_min, self.view_window.x_max, self.view_window.y_max)} fill="none" xmlns="http://www.w3.org/2000/svg" style="display:block">

                    { self.history.iter().map(App::view_circle).collect::<Html>() }
                    { self.circles.iter().map(App::view_circle).collect::<Html>() }

                        </svg>
                        </div>
                }
    }

    pub fn tick(&mut self) -> () {
        for circle in self.circles.iter_mut() {
            let clone = circle.clone();
            self.history.push(clone);
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
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let view_window = ViewWindow {
            x_min: 0.0,
            x_max: 1000.0,
            y_min: 0.0,
            y_max: 1000.0,
        };
        let mut app = App {
            link,
            view_window,
            status: Status::Paused,
            color_config: ColorConfig::default(),
            max_position_delta: 20.0,
            circles: vec![],
            history: vec![],
        };
        app.add_circle();
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
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.view_app()
    }
}
