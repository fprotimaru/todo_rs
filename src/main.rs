use sqlx::PgPool;
use std::env;
use todo_rs::db::create_todo;
use todo_rs::model::Todo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let todo_1 = Todo::new(
        "Buy groceries".to_string(),
        "Milk, eggs, bread".to_string(),
        false,
    );
    let todo_2 = Todo::new(
        "Finish homework".to_string(),
        "Math, Science, English".to_string(),
        false,
    );

    let inserted_todo_1 = create_todo(&pool, todo_1).await?;
    println!("Inserted Todo 1: {:#?}", inserted_todo_1);
    let inserted_todo_2 = create_todo(&pool, todo_2).await?;
    println!("Inserted Todo 2: {:#?}", inserted_todo_2);

    Ok(())
}
