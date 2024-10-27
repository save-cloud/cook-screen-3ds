use cook_screen::{app, constant::VERSION_STR, render, resource::Resource};
use log::{error, info};

fn main() {
    if let Err(err) = match Resource::new(false) {
        Ok(resource) => {
            info!("Cook Screen {}", VERSION_STR);
            render::launch(app::Main, resource)
        }
        Err(err) => Err(err),
    } {
        error!("Failed to initialize app: {:?}", err);
    }
}
