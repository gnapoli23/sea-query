use std::rc::Rc;
use super::*;

impl IndexBuilder for PostgresQueryBuilder {
    fn prepare_table_index_expression(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        self.prepare_index_prefix(create, sql);
        write!(sql, "KEY ").unwrap();

        self.prepare_index_name(&create.index.name, sql);

        // self.prepare_index_type(&create.index_type, sql);

        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

        self.prepare_index_name(&create.index.name, sql);

        write!(sql, " ON ").unwrap();
        if let Some(table) = &create.table {
            table.prepare(sql, '"');
        }

        self.prepare_index_type(&create.index_type, sql);

        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();
        if let Some(name) = &drop.index.name {
            write!(sql, "\"{}\"", name).unwrap();
        }
    }
}

impl PostgresQueryBuilder {
    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY ").unwrap();
        }
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
    }

    fn prepare_index_name(&self, name: &Option<String>, sql: &mut SqlWriter) {
        if let Some(name) = name {
            write!(sql, "\"{}\"", name).unwrap();
        }
    }

    fn prepare_index_type(&self, col_index_type: &Option<IndexType>, sql: &mut SqlWriter) {
        if let Some(index_type) = col_index_type {
            write!(sql, " USING {}", match index_type {
                IndexType::BTree => "BTREE".to_owned(),
                IndexType::FullText => "GIN".to_owned(),
                IndexType::Hash => "HASH".to_owned(),
                IndexType::Custom(custom) => custom.to_string(),
            }).unwrap();
        }
    }

    fn prepare_index_columns(&self, columns: &[Rc<dyn Iden>], sql: &mut SqlWriter) {
        write!(sql, " (").unwrap();
        columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql, '"');
            false
        });
        write!(sql, ")").unwrap();
    }
}