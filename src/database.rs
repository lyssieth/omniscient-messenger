use crate::util::Config;
use rustbreak::{deser::Bincode, PathDatabase};
use serde::{Deserialize, Serialize};
use serenity::model::prelude::GuildId;
use std::collections::BTreeMap;
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {}

pub type DB = PathDatabase<BTreeMap<GuildId, Data>, Bincode>;

pub struct DatabaseContainer;

pub struct Database {
    db: DB,
}

impl Database {
    pub async fn load(populate_from_file: bool) -> crate::Result<Self> {
        let cfg = Config::load().await?;
        let data = BTreeMap::new();
        let db = PathDatabase::create_at_path(cfg.database_path, data)?;

        if populate_from_file {
            db.load()?;
        }
        Ok(Self { db })
    }

    pub fn save(&self) -> crate::Result<()> {
        Ok(self.db.save()?)
    }
}

impl Deref for Database {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.db.save().unwrap()
    }
}
