use super::{PgColumn, PgTypeInfo};
use cdbc::column::ColumnIndex;
use cdbc::error::Error;
use cdbc::utils::ustr::UStr;
use crate::{PgArguments, Postgres};
use cdbc::statement::Statement;
use cdbc::HashMap;
use either::Either;
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PgStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) metadata: Arc<PgStatementMetadata>,
}

#[derive(Debug, Default)]
pub(crate) struct PgStatementMetadata {
    pub(crate) columns: Vec<PgColumn>,
    pub(crate) column_names: HashMap<UStr, usize>,
    pub(crate) parameters: Vec<PgTypeInfo>,
}

impl<'q> Statement<'q> for PgStatement<'q> {
    type Database = Postgres;

    fn to_owned(&self) -> PgStatement<'static> {
        PgStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            metadata: self.metadata.clone(),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[PgTypeInfo], usize>> {
        Some(Either::Left(&self.metadata.parameters))
    }

    fn columns(&self) -> &[PgColumn] {
        &self.metadata.columns
    }

    impl_statement_query!(PgArguments);
}

impl ColumnIndex<PgStatement<'_>> for &'_ str {
    fn index(&self, statement: &PgStatement<'_>) -> Result<usize, Error> {
        statement
            .metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
