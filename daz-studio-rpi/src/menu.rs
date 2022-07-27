use std::fs;
use std::path::Path;

pub const ID_MENU_MAIN : usize = 0;
pub const ID_MENU_PROJECT : usize = 1;

#[derive(Clone)]
pub enum Action {
    OpenMenu(usize),
    LoadProject(String),
    SaveProject,
    // Nothing
}

pub struct MenuItem {
    pub name: String,
    pub value: String,
    pub action: Action
}

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub current: usize,
    pub is_opened: bool,
}

impl Menu {
    pub fn main() -> Menu {
        Menu {
            current: 0,
            is_opened: false,
            items: vec![
                MenuItem {
                    name: "Open Project".to_string(),
                    value: ">".to_string(),
                    action: Action::OpenMenu(ID_MENU_PROJECT),
                    
                },
                MenuItem {
                    name: "Save".to_string(),
                    value: ">".to_string(),
                    action: Action::SaveProject,
                },
                // MenuItem {
                //     name: "Save as".to_string(),
                //     value: ">".to_string(),
                //     action: Action::Nothing,
                // },
            ]
        }
    }

    pub fn projects() -> Menu {
        let mut projects : Vec<MenuItem> = Vec::new();

        fs::create_dir_all("./saves").unwrap();
        let paths = fs::read_dir("./saves").unwrap();

        for path in paths {
            let full_path = path.as_ref().unwrap().path().to_str().unwrap().to_string();
            // if path.unwrap().
            // println!("{}",full_path);
            let extension_result = Path::new(&full_path).extension(); 
            if extension_result.is_some() && extension_result.unwrap().to_str().unwrap().to_string().eq("daz") {
                projects.push(MenuItem {
                    name: path.as_ref().unwrap().file_name().to_str().unwrap().to_string(),
                    value: path.as_ref().unwrap().path().to_str().unwrap().to_string(),
                    action: Action::LoadProject(full_path)
                });
            }
        }

        Menu {
            current: 0,
            is_opened: false,
            items: projects
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
    
    pub fn enter(&mut self) -> Action {
        self.items[self.current].action.clone()
    }

}