use vscode::{commands, events, Result};

pub fn activate() -> Result<()> {
    commands::register("extension.sayHello", || {
        vscode::window::show_information_message("Hello, world!");
    })?;

    events::on_activate(|| {
        vscode::window::show_information_message("Extension activated!");
    });

    events::on_deactivate(|| {
        vscode::window::show_information_message("Extension deactivated!");
    });

    Ok(())
}

pub fn deactivate() -> Result<()> {
    vscode::window::show_information_message("Extension deactivated!");
    Ok(())
}
