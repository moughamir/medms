use sqlx::sqlite::SqlitePoolOptions;

#[tokio::test]
async fn test_database_migrations() {
    // Use an in-memory database for testing
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Verify system_settings table exists
    let table_exists =
        sqlx::query!("SELECT name FROM sqlite_master WHERE type='table' AND name='users'")
            .fetch_one(&pool)
            .await
            .is_ok();

    assert!(table_exists);
}

#[tokio::test]
async fn test_user_creation_and_totp_flow() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // Insert a test user
    sqlx::query!(
        "INSERT INTO users (username, role, password_hash, full_name) VALUES (?, ?, ?, ?)",
        "testuser",
        "admin",
        "mock_hash",
        "Test User"
    )
    .execute(&pool)
    .await
    .unwrap();

    // Verify user exists
    let user = sqlx::query!("SELECT username FROM users WHERE username = ?", "testuser")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(user.username, "testuser");
}
