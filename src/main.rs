mod doctest_setup;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use diesel::{ExpressionMethods as _, QueryDsl};

    // use crate::doctest_setup::establish_connection;

    use super::*;

    #[tokio::test]
    async fn demo() {
        use diesel::result::Error;
        use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
        use doctest_setup::schema::users::dsl::*;

        use diesel_async::pooled_connection::bb8::Pool;
        use diesel_async::pooled_connection::AsyncDieselConnectionManager;
        // let conn = &mut establish_connection().await;

        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
            "postgres://localhost:5432",
        );
        let pool = Pool::builder().build(config).await.unwrap();
        let mut conn = pool.get().await.unwrap();

        conn.test_transaction::<_, Error, _>(|conn| {
            async move {
                diesel::insert_into(users)
                    .values(name.eq("Ruby"))
                    .execute(conn)
                    .await?;

                let all_names = users.select(name).load::<String>(conn).await?;
                assert_eq!(vec!["Sean", "Tess", "Ruby"], all_names);

                Ok(())
            }
            .scope_boxed()
        })
        .await;

        // Even though we returned `Ok`, the transaction wasn't committed.
        let all_names = users.select(name).load::<String>(&mut conn).await.unwrap();
        assert_eq!(vec!["Sean", "Tess"], all_names);
    }
}
