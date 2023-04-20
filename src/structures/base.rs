use crate::{database::pool, utils::Snowflake};
use sqlx::{
    postgres::{PgArguments, PgRow},
    Arguments, FromRow, Transaction,
};
use sqlx::{Encode, Postgres, Type};

pub struct SqlQuery {
    args: PgArguments,
    query: String,
}

impl SqlQuery {
    pub fn new<Q: Into<String>>(query: Q) -> Self {
        Self {
            args: PgArguments::default(),
            query: query.into(),
        }
    }

    pub fn push<'q, Q>(mut self, arg: Q) -> Self
    where
        Q: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
    {
        self.args.add(arg);
        self
    }

    pub async fn find<'a, T: Base<'a, Snowflake> + Send>(self) -> Result<Vec<T>, sqlx::Error> {
        T::find_with_args(self.query, self.args).await
    }

    pub async fn find_one<'a, T: Base<'a, Snowflake> + Send>(self) -> Result<T, sqlx::Error> {
        T::find_one_with_args(self.query, self.args).await
    }
}

#[async_trait]
pub trait Base<'q, T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send + ToString + 'q>:
    for<'a> FromRow<'a, PgRow> + Sized + Unpin
{
    fn id(&self) -> T;

    fn table_name() -> &'static str;

    fn fields(&self) -> (Vec<&str>, PgArguments);

    fn primary_key() -> &'static str {
        "id"
    }

    async fn update(&self) -> Result<(), sqlx::Error> {
        let (columns, args) = self.fields();
        let mut args_placeholders = vec![];

        let mut i = 1;

        for col in columns {
            args_placeholders.push(format!("{col} = ${i}"));
            i += 1;
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {} = ${i}",
            Self::table_name(),
            args_placeholders.join(","),
            Self::primary_key()
        );

        log::debug!("{query}");

        sqlx::query_with(&query, args).execute(pool()).await?;

        Ok(())
    }

    async fn update_tx(&self, tx: &mut Transaction<Postgres>) -> Result<(), sqlx::Error> {
        let (columns, mut args) = self.fields();
        let mut args_placeholders = vec![];

        let mut i = 1;

        for col in columns {
            args_placeholders.push(format!("{col} = ${i}"));
            i += 1;
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {} = ${i}",
            Self::table_name(),
            args_placeholders.join(","),
            Self::primary_key()
        );

        args.add(self.id());

        log::debug!("{query}");

        sqlx::query_with(&query, args).execute(tx).await?;

        Ok(())
    }

    async fn insert(&self) -> Result<(), sqlx::Error> {
        let (columns, args) = self.fields();
        let mut args_placeholders = vec![];

        let mut i = 1;

        for _ in 0..columns.len() {
            args_placeholders.push(format!("${i}"));
            i += 1;
        }

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            Self::table_name(),
            columns.join(","),
            args_placeholders.join(",")
        );

        log::debug!("{query}");

        sqlx::query_with(&query, args).execute(pool()).await?;

        Ok(())
    }

    async fn find<TT: Into<String> + Send, Q>(
        filter: TT,
        args: Vec<Q>,
    ) -> Result<Vec<Self>, sqlx::Error>
    where
        Q: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
    {
        let mut arguments = PgArguments::default();

        for arg in args {
            arguments.add(arg);
        }

        Self::find_with_args(filter, arguments).await
    }

    async fn find_with_args<TT: Into<String> + Send>(
        filter: TT,
        arguments: PgArguments,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as_with::<_, Self, _>(
            &format!(
                "SELECT * FROM {} WHERE {}",
                Self::table_name(),
                filter.into()
            ),
            arguments,
        )
        .fetch_all(pool())
        .await
    }

    async fn find_and_limit<TT: Into<String> + Send, Q>(
        filter: TT,
        args: Vec<Q>,
        limit: usize,
    ) -> Result<Vec<Self>, sqlx::Error>
    where
        Q: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
    {
        let mut arguments = PgArguments::default();

        for arg in args {
            arguments.add(arg);
        }

        sqlx::query_as_with::<_, Self, _>(
            &format!(
                "SELECT * FROM {} WHERE {} LIMIT {}",
                Self::table_name(),
                filter.into(),
                limit
            ),
            arguments,
        )
        .fetch_all(pool())
        .await
    }

    async fn find_one<TT: Into<String> + Send, Q>(
        filter: TT,
        args: Vec<Q>,
    ) -> Result<Self, sqlx::Error>
    where
        Q: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
    {
        let mut arguments = PgArguments::default();

        for arg in args {
            arguments.add(arg);
        }

        Self::find_one_with_args(filter, arguments).await
    }

    async fn find_one_with_args<TT: Into<String> + Send>(
        filter: TT,
        arguments: PgArguments,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as_with::<_, Self, _>(
            &format!(
                "SELECT * FROM {} WHERE {} LIMIT 1",
                Self::table_name(),
                filter.into()
            ),
            arguments,
        )
        .fetch_one(pool())
        .await
    }

    async fn find_by_id<Q: ToString + Send>(id: Q) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT * FROM {} WHERE {} = {}",
            Self::table_name(),
            Self::primary_key(),
            id.to_string()
        ))
        .fetch_one(pool())
        .await
    }

    async fn count<TT: Into<String> + Send, Q>(filter: TT, args: Vec<Q>) -> Result<u64, sqlx::Error>
    where
        Q: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
    {
        let mut arguments = PgArguments::default();

        for arg in args {
            arguments.add(arg);
        }

        Ok(sqlx::query_with(
            &format!(
                "SELECT COUNT(*) FROM {} WHERE {}",
                Self::table_name(),
                filter.into()
            ),
            arguments,
        )
        .execute(pool())
        .await?
        .rows_affected())
    }

    async fn delete(self) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "DELETE FROM {} WHERE {} = {}",
            Self::table_name(),
            Self::primary_key(),
            self.id().to_string()
        ))
        .execute(pool())
        .await
        .map(|_| ())
    }

    async fn delete_tx(self, tx: &mut Transaction<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "DELETE FORM {} WHERE {} = {}",
            Self::table_name(),
            Self::primary_key(),
            self.id().to_string()
        ))
        .execute(tx)
        .await
        .map(|_| ())
    }
}
