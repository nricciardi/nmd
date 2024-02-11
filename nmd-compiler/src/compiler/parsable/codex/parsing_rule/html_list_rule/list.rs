
#[derive(Debug)]
pub enum ListType {
    Unordered,
    Ordered(usize)
}


#[derive(Debug)]
pub struct List {
    list_type: ListType,
    items: Vec<ListItem>
}

#[derive(Debug)]
pub enum ListItem {
    String,
    List
}

impl List {
    pub fn new(list_type: ListType) -> Self {
        Self {
            list_type,
            items: Vec::new()
        }
    }

    pub fn add_item(&mut self, item: ListItem) {
        self.items.push(item)
    }

    pub fn list_type(&self) -> &ListType {
        &self.list_type
    }

    pub fn set_list_type(&mut self, value: ListType) {
        self.list_type = value
    }

    pub fn items(&self) -> &Vec<ListItem> {
        &self.items
    }

    pub fn set_items(&mut self, value: Vec<ListItem>) {
        self.items = value
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            list_type: ListType::Unordered,
            items: Vec::new()
        }
    }
}