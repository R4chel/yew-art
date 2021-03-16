use yew::prelude::*;

use crate::circle::{Circle, Position};

pub struct App {
    circles: Vec<Circle>,
}

pub enum Msg {}

impl App {
    fn view_circle(circle: &Circle) -> Html {
        html! {
            <circle cx={circle.position.x} cy={circle.position.y} r={circle.radius} fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
        }
    }

    fn view_app(&self) -> Html {
        html! {
            <div>
                <p>{ "Hello world!" }</p>

                <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
            { self.circles.iter().map(App::view_circle).collect::<Html>() }
                </svg>
                </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            circles: vec![
                Circle {
                    position: Position { x: 1.0, y: 2.0 },
                    radius: 5.,
                },
                Circle {
                    position: Position { x: 5.0, y: 5.0 },
                    radius: 10.,
                },
            ],
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.view_app()
    }
}
