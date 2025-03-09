use crate::model::Todo;
use sqlx::PgPool;

pub async fn create_todo(pool: &PgPool, todo: Todo) -> anyhow::Result<Todo> {
    let id: i32 = sqlx::query_scalar!(
        "INSERT INTO todos (title, description, completed) VALUES ($1, $2, $3) RETURNING id",
        todo.title,
        todo.description,
        todo.completed,
    )
    .fetch_one(pool)
    .await?;

    Ok(Todo {
        id: Some(id),
        title: todo.title,
        description: todo.description,
        completed: todo.completed,
    })
}
