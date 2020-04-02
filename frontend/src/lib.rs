#[macro_use]
extern crate seed;

use futures::Future;
use rust_web_app::{JsonApiResponse, Task};
use seed::browser::service::fetch::{Request, ResponseDataResult};
use seed::prelude::*;

/**
 * The frontend doesn't do any kind of user input.
 * Doesn't do any kind of routing -- no browsing to multiple pages within the app, no pagination, etc.
 * There are ZERO tests.
 * ZERO attempts at error handling.
 * only scratched the surface of cargo make.
 * There's nary a mention of continuous integration.
 * Nothing about app deployment, upgrades, troubleshooting/debugging, or maintenance.
 * The data model in the database and REST API is trivial.
 * It's out of compliance with the JSON API spec.
 * No users or authentication, or security of any sort.
 * No web-side external plugins (e.g. npm packages).
 * Nothing about interfacing directly with JavaScript.
 *
 * exercise: due dates
 * store a proper date type in the database,
 * carry it up through the models as a date type,
 * and convert it to a string at the last minute before presenting it to the user.
*/

struct Model {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(ResponseDataResult<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| li![{ t.title.clone() }])
        .collect();

    h1![{ "Tasks" }, ul![tasks,],]
}

fn fetch_drills() -> impl Future<Output = Result<Msg, Msg>> {
    let url: &str = "http://localhost:8000/tasks/";
    Request::new(url).fetch_json_data(Msg::FetchedTasks)
}

fn after_mount(_url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(fetch_drills());
    let model = Model { tasks: vec![] };
    AfterMount::new(model).url_handling(UrlHandling::None)
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
