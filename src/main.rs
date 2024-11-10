use anyhow::Context;
use obws::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenvy::dotenv();

    let host = std::env::var("OBS_WS_HOST")?;
    let port = std::env::var("OBS_WS_PORT")?.parse::<u16>()?;
    let password = std::env::var("OBS_WS_PASSWORD")?;

    // Connect to the OBS instance through obs-websocket.
    let client = Client::connect(host, port, Some(password)).await?;

    // Get and print out version information of OBS and obs-websocket.
    let version = client.general().version().await?;
    println!("{version:#?}");

    // Get a list of available scenes and print them out.
    let scene_list = client.scenes().list().await?;
    println!("{scene_list:#?}");

    let blank_scene_id: uuid::Uuid = std::env::var("OBS_BLANK_SCENE")?.parse()?;

    let blank_scene = scene_list
        .scenes
        .iter()
        .find(|scene| scene.id.uuid == blank_scene_id)
        .context("failed to find blank screen")?;

    // Set current scene to the blank screen
    client
        .scenes()
        .set_current_program_scene(blank_scene.id.clone())
        .await?;

    Ok(())
}
