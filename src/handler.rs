use log::info;
use serenity::async_trait;
use serenity::model::prelude::{Activity, Ready};
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data: Ready) {
        info!("Logged in shard {} as {:?}", ctx.shard_id, data.user);
        ctx.set_activity(Activity::playing("om>help for help"))
            .await;
    }
}
