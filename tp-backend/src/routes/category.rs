use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use sea_orm::{ActiveValue, prelude::*};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    AppState,
    entity::{self, category, task},
    utils::errors::APIError,
};

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateCategory {
    pub name: String,
}

impl Into<category::ActiveModel> for CreateCategory {
    fn into(self) -> category::ActiveModel {
        category::ActiveModel {
            name: ActiveValue::Set(self.name),
            ..Default::default()
        }
    }
}

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(category_info): Json<CreateCategory>,
) -> Result<Json<category::Model>, APIError> {
    let db = &state.db;

    let category: category::ActiveModel = category_info.into();
    let inserted = category.insert(db).await?;

    Ok(Json(inserted))
}

pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<i32>,
) -> Result<(), APIError> {
    let db = &state.db;
    task::Entity::update_many()
        .filter(task::Column::Category.eq(category_id))
        .col_expr(task::Column::Category, Expr::value(Value::Int(None)))
        .exec(db)
        .await?;
    category::Entity::delete_by_id(category_id).exec(db).await?;
    Ok(())
}

pub async fn nuke_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<i32>,
) -> Result<(), APIError> {
    let db = &state.db;

    task::Entity::delete_many()
        .filter(task::Column::Category.eq(category_id))
        .exec(db)
        .await?;

    category::Entity::delete_by_id(category_id).exec(db).await?;

    Ok(())
}

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<category::Model>>, APIError> {
    let db = &state.db;
    let categories = category::Entity::find().all(db).await?;
    Ok(Json(categories))
}

pub async fn list_category_tasks(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<i32>,
) -> Result<Json<Vec<task::Model>>, APIError> {
    let db = &state.db;
    let all: Vec<(category::Model, Vec<task::Model>)> = category::Entity::find_by_id(category_id)
        .find_with_related(task::Entity)
        .all(db)
        .await?;
    let (_category, tasks): &(category::Model, Vec<task::Model>) = all.first().ok_or(APIError::NotFound)?;
    Ok(Json(tasks.clone()))
}
