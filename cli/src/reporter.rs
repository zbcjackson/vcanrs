use std::collections::HashMap;
use std::path::PathBuf;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{TableBuilder, TableStyle};
#[cfg(test)]
use mockall::{automock};

#[cfg_attr(test, automock)]
pub trait Reporter {
    fn report(&self, stat: &HashMap<PathBuf, i32>);
}

pub struct TableReporter {}

impl TableReporter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Reporter for TableReporter {
    fn report(&self, stat: &HashMap<PathBuf, i32>) {
        let mut vec: Vec<(&PathBuf, &i32)> = stat.iter().collect();
        vec.sort_by(|a, b| b.1.cmp(a.1));
        let rows = vec.iter().map(|(file, count)| Row::new(vec![TableCell::new(file.display()), TableCell::new_with_alignment(count, 1, Alignment::Right)])).collect();
        let table = TableBuilder::new().style(TableStyle::simple()).rows(rows).build();
        println!("{}", table.render());
    }
}