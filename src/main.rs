use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, get_service, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

type AppState = Arc<SqlitePool>;

#[tokio::main]
async fn main() {
    
    let pool = SqlitePool::connect_with(
        sqlx::sqlite::SqliteConnectOptions::new()
            .filename("todos.db")
            .create_if_missing(true)
    )
    .await
    .expect("Failed to connect to SQLite");

    // Create the todos table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create todos table");

    // Get static directory from environment or default
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "./frontend/dist/frontend".to_string());
    
    let app = Router::new()
        .route("/api/todos", get(get_todos))
        .route("/api/todos", post(create_todo))
        .route("/api/todos/:id", put(update_todo))
        .route("/api/todos/:id", delete(delete_todo))
        .fallback(get_service(ServeDir::new(&static_dir)).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .with_state(Arc::new(pool));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = format!("{}:{}", host, port);
    
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_todos(State(pool): State<AppState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let rows = sqlx::query("SELECT id, title, completed, created_at FROM todos ORDER BY created_at DESC")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos: Vec<Todo> = rows
        .iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            completed: row.get::<i32, _>("completed") == 1,
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    sqlx::query("INSERT INTO todos (id, title, completed, created_at) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&payload.title)
        .bind(0)
        .bind(&created_at)
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
        created_at,
    };

    Ok(Json(todo))
}

async fn update_todo(
    State(pool): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let existing = sqlx::query("SELECT id, title, completed, created_at FROM todos WHERE id = ?")
        .bind(&id)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = existing.ok_or(StatusCode::NOT_FOUND)?;

    let title = payload.title.unwrap_or_else(|| existing.get("title"));
    let completed = payload.completed.unwrap_or_else(|| existing.get::<i32, _>("completed") == 1);

    sqlx::query("UPDATE todos SET title = ?, completed = ? WHERE id = ?")
        .bind(&title)
        .bind(if completed { 1 } else { 0 })
        .bind(&id)
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo = Todo {
        id,
        title,
        completed,
        created_at: existing.get("created_at"),
    };

    Ok(Json(todo))
}

async fn delete_todo(
    State(pool): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(&id)
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}