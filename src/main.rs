use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy},
    renderer::{DrawFlat, PosTex},
};
use std::time::Duration;

mod gametwo;
use crate::gametwo::GameTwo;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use amethyst::utils::application_root_dir;
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    //let config = DisplayConfig::load(&path);
/*
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );
*/
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), true)?;
    
    let mut game = Application::build("./", GameTwo)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();

    Ok(())
}
