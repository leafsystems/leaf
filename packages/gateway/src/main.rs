use anyhow::{Error, Result};
use rppal::uart::{Parity, Uart};

fn main() {
    async_std::task::block_on(async {
        match tester().await {
            Ok(_) => println!("working!"),
            Err(e) => println!("failed!, {:#?}", e),
        }
    })
}

async fn app() -> Result<()> {
    let res = reqwest::get("https://hyper.rs").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);
    Ok(())
}
