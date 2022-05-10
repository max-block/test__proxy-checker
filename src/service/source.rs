use std::sync::Arc;
use crate::{db::{Db, bson_object_id_from_str}, error::AppError};

pub struct SourceService {
    db: Arc<Db>,
}

impl SourceService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: String) ->Result<(), AppError> {
        let _source = self.db.source.find_by_id(bson_object_id_from_str(id)?).await?;
        Ok(())
    }
}