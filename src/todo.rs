pub struct Item {
    pub descr: String,
    pub is_complete: bool,
}

pub struct ToDo {
    pub items: Vec<Item>,
}

impl ToDo {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add (&mut self, val: &str) {
        self.items.push(Item {
            descr: val.to_string(),
            is_complete: false,
        });
    }

    pub fn toggle(&mut self, idx: usize) -> Result<(), &'static str> {
        if let Some(item) = self.items.get_mut(idx) {
            item.is_complete = !item.is_complete;
            Ok(())
        } else {
            Err("Item index out of range!")
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

} // impl
