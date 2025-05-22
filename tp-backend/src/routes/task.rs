use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_with::rust::double_option;
use ts_rs::TS;

use crate::{
    entity::{self, task}, utils::{errors::APIError, option_to_active}, AppState
};

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<entity::task::Model>, APIError> {
    let db = &state.db;
    let task = entity::prelude::Task::find_by_id(id)
        .one(db)
        .await?
        .ok_or(APIError::NotFound)?;
    Ok(Json(task))
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateTask {
    description: String,
    importance: Option<f32>,
    urgency: Option<f32>,
    due_date: Option<chrono::DateTime<Utc>>,
    do_date: Option<chrono::DateTime<Utc>>,
    category: Option<i32>,
}

impl Into<entity::task::ActiveModel> for CreateTask {
    fn into(self) -> entity::task::ActiveModel {
        entity::task::ActiveModel {
            id: ActiveValue::NotSet,
            done: ActiveValue::Set(false),
            description: ActiveValue::Set(self.description),
            importance: option_to_active(Some(self.importance)),
            urgency: option_to_active(Some(self.urgency)),
            due_date: option_to_active(Some(self.due_date)),
            do_date: option_to_active(Some(self.do_date)),
            category: option_to_active(Some(self.category)),
        }
    }
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(task_info): Json<CreateTask>,
) -> Result<Json<entity::task::Model>, APIError> {
    let db = &state.db;

    let first_task_entity: entity::task::ActiveModel = task_info.into();

    let inserted_id = entity::prelude::Task::insert(first_task_entity)
        .exec(db)
        .await?
        .last_insert_id;
    // We can unwrap here because we know it exists
    let inserted = entity::prelude::Task::find_by_id(inserted_id)
        .one(db)
        .await?
        .unwrap();

    Ok(Json(inserted))
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PatchTask {
    #[ts(optional)]
    description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    #[ts(optional)]
    importance: Option<Option<f32>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    #[ts(optional)]
    urgency: Option<Option<f32>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    #[ts(optional)]
    due_date: Option<Option<chrono::DateTime<Utc>>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    #[ts(optional)]
    do_date: Option<Option<chrono::DateTime<Utc>>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    #[ts(optional)]
    category: Option<Option<i32>>,
}

pub async fn patch_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(patch_info): Json<PatchTask>,
) -> Result<Json<entity::task::Model>, APIError> {
    let db = &state.db;
    let task = entity::prelude::Task::find_by_id(id)
        .one(db)
        .await?
        .ok_or(APIError::NotFound)?;
    let mut task: entity::task::ActiveModel = task.into();

    macro_rules! update_task_field {
        ($field:ident) => {
            if let Some(value) = patch_info.$field {
                task.$field = ActiveValue::Set(value);
            }
        };
    }
    update_task_field!(description);
    update_task_field!(importance);
    update_task_field!(urgency);
    update_task_field!(due_date);
    update_task_field!(do_date);
    update_task_field!(category);
    
    task.update(db).await?;

    let task = entity::prelude::Task::find_by_id(id)
        .one(db)
        .await?
        .ok_or(APIError::NotFound)?;

    Ok(Json(task))
}
