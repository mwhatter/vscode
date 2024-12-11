use vscode::{commands, events, Result};

fn main() -> Result<()> {
    let context = vscode::initialize()?;

    commands::register("extension.sayHello", || {
        vscode::window::show_information_message("Hello, world!");
    })?;

    events::on_activate(|| {
        vscode::window::show_information_message("Extension activated!");
    });

    events::on_deactivate(|| {
        vscode::window::show_information_message("Extension deactivated!");
    });

    context.run()
}
