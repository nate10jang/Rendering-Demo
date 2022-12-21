mod application;
mod window;
mod events;

use application::Application;

fn main() {
    let app: Application = Application::new();

    app.run();
} 