use bevy::app::App;
use termshot::TermshotPlugin;

fn main() -> anyhow::Result<()> {
    App::new().add_plugins(TermshotPlugin).run();
    Ok(())
}
