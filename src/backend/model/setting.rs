use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, Set};

use crate::backend::logging;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub key: String,
    pub value_json: String,
    pub value_type: String,
    pub group_name: String,
    pub is_secret: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone)]
pub struct SettingUpsert {
    pub key: String,
    pub value_json: String,
    pub value_type: String,
    pub group_name: String,
    pub is_secret: bool,
}

impl SettingUpsert {
    pub fn new(
        key: impl Into<String>,
        value_json: impl Into<String>,
        value_type: impl Into<String>,
        group_name: impl Into<String>,
        is_secret: bool,
    ) -> Self {
        let key = key.into();
        logging::debug("model.setting", "upsert.new", &key);
        Self {
            key,
            value_json: value_json.into(),
            value_type: value_type.into(),
            group_name: group_name.into(),
            is_secret,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        logging::debug("model.setting", "validate", &self.key);
        if self.key.trim().is_empty() {
            return Err("setting key is required".to_string());
        }
        if self.value_type.trim().is_empty() {
            return Err("setting value_type is required".to_string());
        }
        if self.group_name.trim().is_empty() {
            return Err("setting group_name is required".to_string());
        }
        Ok(())
    }

    pub fn into_active_model(self) -> Result<ActiveModel, String> {
        self.validate()?;
        Ok(ActiveModel {
            id: ActiveValue::NotSet,
            key: Set(self.key),
            value_json: Set(self.value_json),
            value_type: Set(self.value_type),
            group_name: Set(self.group_name),
            is_secret: Set(self.is_secret),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        })
    }
}

impl Model {
    pub fn to_pair(&self) -> (String, String) {
        logging::debug("model.setting", "to_pair", &self.key);
        (self.key.clone(), self.value_json.clone())
    }
}
