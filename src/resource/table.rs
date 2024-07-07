use getset::{Getters, MutGetters, Setters};


#[derive(Debug, Clone, Default)]
pub enum TableCellAlignment {
    Left,
    #[default] Center,
    Right
}



#[derive(Debug, Clone, Default)]
pub enum TableCell {
    #[default] None,
    ContentCell{content: String, alignment: TableCellAlignment}
}



#[derive(Debug, Clone, Getters, MutGetters, Setters)]
pub struct Table {

    #[getset(get = "pub", set = "pub")]
    header: Option<Vec<TableCell>>,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    body: Vec<Vec<TableCell>>,

    #[getset(get = "pub", set = "pub")]
    footer: Option<Vec<TableCell>>
}

impl Table {
    pub fn new() -> Self {
        Self {
            header: None,
            body: Vec::new(),
            footer: None
        }
    }

    pub fn append_to_body(&mut self, row: Vec<TableCell>) {
        
        self.body.push(row);
    }

    pub fn shift_first_body_row_to_header(&mut self) {

        let first_row = self.body.remove(0);

        self.header = Some(first_row.clone());

    }

    pub fn shift_last_body_row_to_footer(&mut self) {

        let last_row = self.body.remove(self.body.len() - 1);

        self.footer = Some(last_row.clone());

    }
}