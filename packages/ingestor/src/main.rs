mod app;
mod logging;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    logging::set_up_logging();
    app::app().await?;
    Ok(())
}
