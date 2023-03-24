mod app;

use yew::Renderer;
use app::App;

fn main() {
    Renderer::<App>::new().render();
}