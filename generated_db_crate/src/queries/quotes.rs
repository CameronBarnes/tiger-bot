// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct AddQuoteParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub msg_id: i32,
    pub user_from: rust_decimal::Decimal,
    pub chat_id: i64,
    pub quoted_by: rust_decimal::Decimal,
    pub msg_type: crate::types::QuoteType,
    pub msg_date: crate::types::time::Date,
    pub has_spoiler: bool,
    pub text_content: Option<T1>,
    pub file_id: Option<T2>,
}
#[derive(Clone, Copy, Debug)]
pub struct QuoteFromUserParams {
    pub chat_id: i64,
    pub user_from: rust_decimal::Decimal,
}
#[derive(Debug)]
pub struct SearchQuoteParams<T1: crate::StringSql> {
    pub chat_id: i64,
    pub query: T1,
}
#[derive(Debug)]
pub struct SearchQuoteFromUserParams<T1: crate::StringSql> {
    pub chat_id: i64,
    pub user_from: rust_decimal::Decimal,
    pub query: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Quote {
    pub user_from: rust_decimal::Decimal,
    pub chat_id: i64,
    pub quoted_by: rust_decimal::Decimal,
    pub msg_type: crate::types::QuoteType,
    pub msg_date: crate::types::time::Date,
    pub has_spoiler: bool,
    pub text: Option<String>,
    pub file_id: Option<String>,
}
pub struct QuoteBorrowed<'a> {
    pub user_from: rust_decimal::Decimal,
    pub chat_id: i64,
    pub quoted_by: rust_decimal::Decimal,
    pub msg_type: crate::types::QuoteType,
    pub msg_date: crate::types::time::Date,
    pub has_spoiler: bool,
    pub text: Option<&'a str>,
    pub file_id: Option<&'a str>,
}
impl<'a> From<QuoteBorrowed<'a>> for Quote {
    fn from(
        QuoteBorrowed {
            user_from,
            chat_id,
            quoted_by,
            msg_type,
            msg_date,
            has_spoiler,
            text,
            file_id,
        }: QuoteBorrowed<'a>,
    ) -> Self {
        Self {
            user_from,
            chat_id,
            quoted_by,
            msg_type,
            msg_date,
            has_spoiler,
            text: text.map(|v| v.into()),
            file_id: file_id.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MostQuoted {
    pub user_from: rust_decimal::Decimal,
    pub count: i64,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MostQuotedBy {
    pub quoted_by: rust_decimal::Decimal,
    pub count: i64,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct QuoteQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<QuoteBorrowed, tokio_postgres::Error>,
    mapper: fn(QuoteBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> QuoteQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(QuoteBorrowed) -> R) -> QuoteQuery<'c, 'a, 's, C, R, N> {
        QuoteQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub struct I64Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<i64, tokio_postgres::Error>,
    mapper: fn(i64) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I64Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'c, 'a, 's, C, R, N> {
        I64Query {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub struct MostQuotedQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<MostQuoted, tokio_postgres::Error>,
    mapper: fn(MostQuoted) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> MostQuotedQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(MostQuoted) -> R) -> MostQuotedQuery<'c, 'a, 's, C, R, N> {
        MostQuotedQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub struct MostQuotedByQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<MostQuotedBy, tokio_postgres::Error>,
    mapper: fn(MostQuotedBy) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> MostQuotedByQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(MostQuotedBy) -> R) -> MostQuotedByQuery<'c, 'a, 's, C, R, N> {
        MostQuotedByQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub fn add_quote() -> AddQuoteStmt {
    AddQuoteStmt(crate::client::async_::Stmt::new(
        "INSERT INTO quotes (msg_id, user_from, chat_id, quoted_by, msg_type, msg_date, has_spoiler, text, file_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    ))
}
pub struct AddQuoteStmt(crate::client::async_::Stmt);
impl AddQuoteStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        msg_id: &'a i32,
        user_from: &'a rust_decimal::Decimal,
        chat_id: &'a i64,
        quoted_by: &'a rust_decimal::Decimal,
        msg_type: &'a crate::types::QuoteType,
        msg_date: &'a crate::types::time::Date,
        has_spoiler: &'a bool,
        text_content: &'a Option<T1>,
        file_id: &'a Option<T2>,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(
                stmt,
                &[
                    msg_id,
                    user_from,
                    chat_id,
                    quoted_by,
                    msg_type,
                    msg_date,
                    has_spoiler,
                    text_content,
                    file_id,
                ],
            )
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        AddQuoteParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for AddQuoteStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a AddQuoteParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.msg_id,
            &params.user_from,
            &params.chat_id,
            &params.quoted_by,
            &params.msg_type,
            &params.msg_date,
            &params.has_spoiler,
            &params.text_content,
            &params.file_id,
        ))
    }
}
pub fn random_quote() -> RandomQuoteStmt {
    RandomQuoteStmt(crate::client::async_::Stmt::new(
        "SELECT user_from, chat_id, quoted_by, msg_type, msg_date, has_spoiler, text, file_id FROM quotes WHERE chat_id = ($1) ORDER BY RANDOM() LIMIT 1",
    ))
}
pub struct RandomQuoteStmt(crate::client::async_::Stmt);
impl RandomQuoteStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        chat_id: &'a i64,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 1> {
        QuoteQuery {
            client,
            params: [chat_id],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<QuoteBorrowed, tokio_postgres::Error> {
                Ok(QuoteBorrowed {
                    user_from: row.try_get(0)?,
                    chat_id: row.try_get(1)?,
                    quoted_by: row.try_get(2)?,
                    msg_type: row.try_get(3)?,
                    msg_date: row.try_get(4)?,
                    has_spoiler: row.try_get(5)?,
                    text: row.try_get(6)?,
                    file_id: row.try_get(7)?,
                })
            },
            mapper: |it| Quote::from(it),
        }
    }
}
pub fn get_quote() -> GetQuoteStmt {
    GetQuoteStmt(crate::client::async_::Stmt::new(
        "SELECT user_from, chat_id, quoted_by, msg_type, msg_date, has_spoiler, text, file_id FROM quotes WHERE msg_id = ($1)",
    ))
}
pub struct GetQuoteStmt(crate::client::async_::Stmt);
impl GetQuoteStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        msg_id: &'a i32,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 1> {
        QuoteQuery {
            client,
            params: [msg_id],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<QuoteBorrowed, tokio_postgres::Error> {
                Ok(QuoteBorrowed {
                    user_from: row.try_get(0)?,
                    chat_id: row.try_get(1)?,
                    quoted_by: row.try_get(2)?,
                    msg_type: row.try_get(3)?,
                    msg_date: row.try_get(4)?,
                    has_spoiler: row.try_get(5)?,
                    text: row.try_get(6)?,
                    file_id: row.try_get(7)?,
                })
            },
            mapper: |it| Quote::from(it),
        }
    }
}
pub fn number_of_quotes() -> NumberOfQuotesStmt {
    NumberOfQuotesStmt(crate::client::async_::Stmt::new(
        "SELECT COUNT(*) FROM quotes WHERE chat_id = ($1)",
    ))
}
pub struct NumberOfQuotesStmt(crate::client::async_::Stmt);
impl NumberOfQuotesStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        chat_id: &'a i64,
    ) -> I64Query<'c, 'a, 's, C, i64, 1> {
        I64Query {
            client,
            params: [chat_id],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub fn most_quoted() -> MostQuotedStmt {
    MostQuotedStmt(crate::client::async_::Stmt::new(
        "SELECT user_from, COUNT(*) AS count FROM quotes WHERE chat_id = ($1) GROUP BY user_from ORDER BY count DESC LIMIT 5",
    ))
}
pub struct MostQuotedStmt(crate::client::async_::Stmt);
impl MostQuotedStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        chat_id: &'a i64,
    ) -> MostQuotedQuery<'c, 'a, 's, C, MostQuoted, 1> {
        MostQuotedQuery {
            client,
            params: [chat_id],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<MostQuoted, tokio_postgres::Error> {
                Ok(MostQuoted {
                    user_from: row.try_get(0)?,
                    count: row.try_get(1)?,
                })
            },
            mapper: |it| MostQuoted::from(it),
        }
    }
}
pub fn most_quoted_by() -> MostQuotedByStmt {
    MostQuotedByStmt(crate::client::async_::Stmt::new(
        "SELECT quoted_by, COUNT(*) AS count FROM quotes WHERE chat_id = ($1) GROUP BY quoted_by ORDER BY count DESC LIMIT 5",
    ))
}
pub struct MostQuotedByStmt(crate::client::async_::Stmt);
impl MostQuotedByStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        chat_id: &'a i64,
    ) -> MostQuotedByQuery<'c, 'a, 's, C, MostQuotedBy, 1> {
        MostQuotedByQuery {
            client,
            params: [chat_id],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<MostQuotedBy, tokio_postgres::Error> {
                Ok(MostQuotedBy {
                    quoted_by: row.try_get(0)?,
                    count: row.try_get(1)?,
                })
            },
            mapper: |it| MostQuotedBy::from(it),
        }
    }
}
pub fn quote_from_user() -> QuoteFromUserStmt {
    QuoteFromUserStmt(crate::client::async_::Stmt::new(
        "SELECT user_from, chat_id, quoted_by, msg_type, msg_date, has_spoiler, text, file_id FROM quotes WHERE chat_id = ($1) AND user_from = ($2) ORDER BY RANDOM() LIMIT 1",
    ))
}
pub struct QuoteFromUserStmt(crate::client::async_::Stmt);
impl QuoteFromUserStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        chat_id: &'a i64,
        user_from: &'a rust_decimal::Decimal,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 2> {
        QuoteQuery {
            client,
            params: [chat_id, user_from],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<QuoteBorrowed, tokio_postgres::Error> {
                Ok(QuoteBorrowed {
                    user_from: row.try_get(0)?,
                    chat_id: row.try_get(1)?,
                    quoted_by: row.try_get(2)?,
                    msg_type: row.try_get(3)?,
                    msg_date: row.try_get(4)?,
                    has_spoiler: row.try_get(5)?,
                    text: row.try_get(6)?,
                    file_id: row.try_get(7)?,
                })
            },
            mapper: |it| Quote::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        QuoteFromUserParams,
        QuoteQuery<'c, 'a, 's, C, Quote, 2>,
        C,
    > for QuoteFromUserStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a QuoteFromUserParams,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 2> {
        self.bind(client, &params.chat_id, &params.user_from)
    }
}
pub fn search_quote() -> SearchQuoteStmt {
    SearchQuoteStmt(crate::client::async_::Stmt::new(
        "SELECT user_from, chat_id, quoted_by, msg_type, msg_date, has_spoiler, text, file_id FROM quotes WHERE chat_id = ($1) AND textsearchable_index_col @@ to_tsquery($2) ORDER BY RANDOM() LIMIT 1",
    ))
}
pub struct SearchQuoteStmt(crate::client::async_::Stmt);
impl SearchQuoteStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        chat_id: &'a i64,
        query: &'a T1,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 2> {
        QuoteQuery {
            client,
            params: [chat_id, query],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<QuoteBorrowed, tokio_postgres::Error> {
                Ok(QuoteBorrowed {
                    user_from: row.try_get(0)?,
                    chat_id: row.try_get(1)?,
                    quoted_by: row.try_get(2)?,
                    msg_type: row.try_get(3)?,
                    msg_date: row.try_get(4)?,
                    has_spoiler: row.try_get(5)?,
                    text: row.try_get(6)?,
                    file_id: row.try_get(7)?,
                })
            },
            mapper: |it| Quote::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        SearchQuoteParams<T1>,
        QuoteQuery<'c, 'a, 's, C, Quote, 2>,
        C,
    > for SearchQuoteStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a SearchQuoteParams<T1>,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 2> {
        self.bind(client, &params.chat_id, &params.query)
    }
}
pub fn search_quote_from_user() -> SearchQuoteFromUserStmt {
    SearchQuoteFromUserStmt(crate::client::async_::Stmt::new(
        "SELECT user_from, chat_id, quoted_by, msg_type, msg_date, has_spoiler, text, file_id FROM quotes WHERE chat_id = ($1) AND user_from = ($2) AND textsearchable_index_col @@ to_tsquery($3) ORDER BY RANDOM() LIMIT 1",
    ))
}
pub struct SearchQuoteFromUserStmt(crate::client::async_::Stmt);
impl SearchQuoteFromUserStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        chat_id: &'a i64,
        user_from: &'a rust_decimal::Decimal,
        query: &'a T1,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 3> {
        QuoteQuery {
            client,
            params: [chat_id, user_from, query],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<QuoteBorrowed, tokio_postgres::Error> {
                Ok(QuoteBorrowed {
                    user_from: row.try_get(0)?,
                    chat_id: row.try_get(1)?,
                    quoted_by: row.try_get(2)?,
                    msg_type: row.try_get(3)?,
                    msg_date: row.try_get(4)?,
                    has_spoiler: row.try_get(5)?,
                    text: row.try_get(6)?,
                    file_id: row.try_get(7)?,
                })
            },
            mapper: |it| Quote::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        SearchQuoteFromUserParams<T1>,
        QuoteQuery<'c, 'a, 's, C, Quote, 3>,
        C,
    > for SearchQuoteFromUserStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a SearchQuoteFromUserParams<T1>,
    ) -> QuoteQuery<'c, 'a, 's, C, Quote, 3> {
        self.bind(client, &params.chat_id, &params.user_from, &params.query)
    }
}
pub fn purge_quotes_for_privacy() -> PurgeQuotesForPrivacyStmt {
    PurgeQuotesForPrivacyStmt(crate::client::async_::Stmt::new(
        "DELETE FROM quotes WHERE user_from = ($1) OR quoted_by = ($1)",
    ))
}
pub struct PurgeQuotesForPrivacyStmt(crate::client::async_::Stmt);
impl PurgeQuotesForPrivacyStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        user_id: &'a rust_decimal::Decimal,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[user_id]).await
    }
}
