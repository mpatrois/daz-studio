pub struct MenuItem {
    pub name: String,
    pub value: String,
}

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub current: usize,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            current: 0,
            items: vec![
                MenuItem {
                    name: "Open Project".to_string(),
                    value: ">".to_string(),
                },
                MenuItem {
                    name: "Save".to_string(),
                    value: ">".to_string(),
                },
                MenuItem {
                    name: "Save as".to_string(),
                    value: ">".to_string(),
                },
            ]
        }
    }

    pub fn up(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        } else {
            self.current = self.items.len() - 1;
        }
    }

    pub fn down(&mut self) {
        self.current += 1;
        if self.current > self.items.len() - 1 {
            self.current = 0;
        }
    }

}