pub use color_eyre::eyre::Result;
use color_eyre::Report;
use log::{error, info};
use serenity::framework::StandardFramework;
use serenity::Client;
use std::collections::HashSet;
use std::process::exit;

mod cmd;
mod database;
mod handler;
mod util;

#[tokio::main]
async fn main() {
    if option_env!("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "omniscient-messenger");
    }

    pretty_env_logger::init();
    better_panic::install();
    color_eyre::install().unwrap();

    info!("Starting up...");
    match do_main().await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to run bot. {:?}", e);
            exit(1);
        }
    }
}

async fn do_main() -> crate::Result<()> {
    let cfg = util::Config::load().await?;

    let client = Client::builder(&cfg.token)
        .framework(StandardFramework::default())
        .await
        .expect("Failed to create initial client.");

    let owners = match client
        .cache_and_http
        .http
        .get_current_application_info()
        .await
    {
        Ok(info) => {
            let mut set = HashSet::with_capacity(1);
            set.insert(info.owner.id);

            set
        }
        Err(why) => {
            error!("Could not retrieve owner information: {:?}", why);
            HashSet::with_capacity(0)
        }
    };

    let framework = StandardFramework::new().configure(|c| {
        c.owners(owners)
            .prefix("om>")
            .with_whitespace(false)
            .case_insensitivity(true)
    });

    let mut client = Client::builder(cfg.token)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Failed to create primary client.");

    // TODO: Insert database into the client data.
    // {
    //     let mut data = client.data.write().await;
    // }

    if let Err(why) = client.start_autosharded().await {
        Err(Report::from(why))
    } else {
        Ok(())
    }
}
