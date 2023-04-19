
use serde::Deserialize;
use sqlx::Type;
use futures::TryStreamExt;

#[derive(Clone, Debug, Deserialize)]
pub struct LabrinthUser {
    pub id: UserId,
    pub github_id: Option<i64>,
    pub username: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Type, Deserialize)]
#[sqlx(transparent)]
pub struct UserId(pub i64);


impl LabrinthUser {
    pub async fn get_all<'a, E>(
        exec: E,
    ) -> Result<Vec<LabrinthUser>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + Copy,
    {
        let users = sqlx::query!(
            "
            SELECT id, github_id, username, name, email FROM users
            ",
        )
        .fetch_many(exec)
        .try_filter_map(|e| async {
            Ok(e.right().map(|u| LabrinthUser {
                id: UserId(u.id),
                github_id: u.github_id,
                name: u.name,
                email: u.email,
                username: u.username,
            }))
        })
        .try_collect::<Vec<LabrinthUser>>()
        .await?;

        Ok(users)
    }
}