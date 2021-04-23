use log::error;
use std::error::Error;
use std::process::exit;

mod counter;
mod handler;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    match do_main().await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to run bot. {:?}", e);
            exit(1);
        }
    }
}

async fn do_main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
