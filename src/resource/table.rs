
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



#[derive(Debug, Clone)]
pub struct Table {
    header: Option<Vec<TableCell>>,
    body: Vec<Vec<TableCell>>,
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

    pub fn header(&self) -> &Option<Vec<TableCell>> {
        &self.header
    }

    pub fn body(&self) -> &Vec<Vec<TableCell>> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Vec<Vec<TableCell>> {
        &mut self.body
    }

    pub fn footer(&self) -> &Option<Vec<TableCell>> {
        &self.footer
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