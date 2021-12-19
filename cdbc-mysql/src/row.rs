use cdbc::column::ColumnIndex;
use cdbc::error::Error;

use crate::{protocol, MySql, MySqlColumn, MySqlValueFormat, MySqlValueRef};
use cdbc::row::Row;
use cdbc::HashMap;
use std::sync::Arc;
use cdbc::utils::ustr::UStr;

/// Implementation of [`Row`] for MySQL.
#[derive(Debug)]
pub struct MySqlRow {
    pub(crate) row: protocol::Row,
    pub(crate) format: MySqlValueFormat,
    pub(crate) columns: Arc<Vec<MySqlColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}


impl Row for MySqlRow {
    type Database = MySql;

    fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<MySqlValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        let column = &self.columns[index];
        let value = self.row.get(index);

        Ok(MySqlValueRef {
            format: self.format,
            row: Some(&self.row.storage),
            type_info: column.type_info.clone(),
            value,
        })
    }
}

impl ColumnIndex<MySqlRow> for &'_ str {
    fn index(&self, row: &MySqlRow) -> Result<usize, Error> {
        row.column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}