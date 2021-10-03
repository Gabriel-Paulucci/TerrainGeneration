use amethyst::{
    core::TransformBundle,
    renderer::{
        types::DefaultBackend, RenderPbr3D, RenderShaded3D, RenderToWindow, RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};
use states::InitialState;

mod states;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(amethyst::LoggerConfig {
        level_filter: amethyst::LogLevelFilter::Debug,
        ..Default::default()
    })
    .start();

    let app_root = application_root_dir()?;

    let display_config = app_root.join("config/display.ron");
    let assets_path = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?;

    let mut game = Application::new(assets_path, InitialState, game_data)?;
    game.run();

    Ok(())
}
