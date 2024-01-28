pub struct Action<'a> {
    name: &'static str,
    f: Box<dyn Fn() + 'a>,
}
pub struct SubMenu<'a> {
    name: &'static str,
    position: u8,
    items: Vec<Item<'a>>,
}

impl SubMenu<'_> {
    fn get_text(&self) -> &'static str {
        if self.position >= self.items.len() as u8 {
            &Item::Back.get_name()
        } else {
            &self.items[self.position as usize].get_name()
        }
    }

    fn get_item(&self) -> &Item {
        if self.position >= self.items.len() as u8 {
            &Item::Back
        } else {
            &self.items[self.position as usize]
        }
    }

    fn go_next(&mut self, no_back: bool) {
        if no_back {
            self.position = (self.position + 1) % self.items.len() as u8
        } else {
            self.position = (self.position + 1) % (self.items.len() as u8 + 1)
        }
    }
}

pub enum Item<'a> {
    Action(Action<'a>),
    SubMenu(SubMenu<'a>),
    Back,
}

impl<'a> Item<'a> {
    pub fn new_action(name: &'static str, f: impl Fn() + 'a) -> Self {
        Self::Action(Action {
            name,
            f: Box::new(f),
        })
    }
    pub fn new_submenu(name: &'static str, items: Vec<Item<'a>>) -> Self {
        Self::SubMenu(SubMenu {
            position: 0,
            name,
            items,
        })
    }
    fn get_name(&self) -> &'static str {
        match self {
            Item::Action(action) => action.name,
            Item::SubMenu(sub) => sub.name,
            Item::Back => "Back",
        }
    }
}

pub struct Menu<'a> {
    root: SubMenu<'a>,
    depth: u8,
    display_text: Box<dyn Fn(&'static str) + 'a>,
}

impl<'a> Menu<'a> {
    pub fn new(items: Vec<Item<'a>>, disp: impl Fn(&'static str) + 'a) -> Self {
        let menu = Self {
            depth: 0,
            display_text: Box::new(disp),
            root: SubMenu {
                name: "root",
                position: 0,
                items,
            },
        };
        menu.display();
        menu
    }

    /// go forward in the menu
    pub fn skip(&mut self) {
        let skip_back = self.depth == 0;
        self.get_submenu_mut().go_next(skip_back);
        self.display();
    }

    /// accept current selection
    pub fn ok(&mut self) {
        let menu = self.get_submenu_mut();
        let item = menu.get_item();
        match item {
            Item::Action(action) => (action.f)(),
            Item::SubMenu(_) => self.depth += 1,
            Item::Back => {
                menu.position = 0;
                self.depth -= 1;
            }
        }

        self.display();
    }

    fn display(&self) {
        let text = self.get_submenu().get_text();
        (self.display_text)(text);
    }

    /// get currently active submenu
    fn get_submenu(&self) -> &SubMenu {
        let mut menu = &self.root;
        for _ in 0..self.depth {
            if let Item::SubMenu(sub) = &menu.items[menu.position as usize] {
                menu = sub;
            } else {
                panic!("attemped to select sub_menu on wrong item");
            }
        }
        menu
    }

    /// get currently active submenu mutably
    fn get_submenu_mut(&mut self) -> &mut SubMenu<'a> {
        let mut menu = &mut self.root;
        for _ in 0..self.depth {
            if let Item::SubMenu(sub) = &mut menu.items[menu.position as usize] {
                menu = sub;
            } else {
                panic!("attemped to select sub_menu on wrong item");
            }
        }
        menu
    }
}
