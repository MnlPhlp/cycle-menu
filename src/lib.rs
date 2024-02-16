#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

use core::cell::RefCell;

#[cfg(not(feature = "std"))]
mod types {
    pub type ActionFunc<'a> = &'a (dyn Fn() + 'a + Send);
    pub type ItemList<'a> = &'a [super::Item<'a>];
    pub type DispFunc<'a> = &'a (dyn Fn(&'static str) + 'a + Send);
}
#[cfg(feature = "std")]
mod types {
    pub type ActionFunc<'a> = Box<dyn Fn() + 'a + Send>;
    pub type ItemList<'a> = Vec<super::Item<'a>>;
    pub type DispFunc<'a> = Box<dyn Fn(&'static str) + 'a + Send>;
}

pub struct Action<'a> {
    name: &'static str,
    f: types::ActionFunc<'a>,
}
pub struct SubMenu<'a> {
    name: &'static str,
    position: RefCell<u8>,
    items: types::ItemList<'a>,
}

impl SubMenu<'_> {
    fn get_text(&self) -> &'static str {
        self.get_item().get_name()
    }

    fn get_item(&self) -> &Item {
        if *self.position.borrow() >= self.items.len() as u8 {
            &Item::Back
        } else {
            &self.items[*self.position.borrow() as usize]
        }
    }

    fn go_next(&self, no_back: bool) {
        let next_pos = if no_back {
            (*self.position.borrow() + 1) % self.items.len() as u8
        } else {
            (*self.position.borrow() + 1) % (self.items.len() as u8 + 1)
        };
        *self.position.borrow_mut() = next_pos;
    }
}

pub enum Item<'a> {
    Action(Action<'a>),
    SubMenu(SubMenu<'a>),
    Back,
}

impl<'a> Item<'a> {
    #[cfg(not(feature = "std"))]
    pub fn new_action(name: &'static str, f: types::ActionFunc<'a>) -> Self {
        Self::Action(Action { name, f })
    }
    #[cfg(feature = "std")]
    pub fn new_action(name: &'static str, f: impl Fn() + 'a + Send) -> Self {
        Self::Action(Action {
            name,
            f: Box::new(f),
        })
    }
    pub fn new_submenu(name: &'static str, items: types::ItemList<'a>) -> Self {
        Self::SubMenu(SubMenu {
            position: RefCell::new(0),
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
    disp: types::DispFunc<'a>,
}

impl<'a> Menu<'a> {
    #[cfg(not(feature = "std"))]
    pub fn new(items: types::ItemList<'a>, disp: types::DispFunc<'a>) -> Self {
        Self::inner_new(items, disp)
    }
    #[cfg(feature = "std")]
    pub fn new(items: types::ItemList<'a>, disp: impl Fn(&'static str) + 'a + Send) -> Self {
        Self::inner_new(items, Box::new(disp))
    }

    fn inner_new(items: types::ItemList<'a>, disp: types::DispFunc<'a>) -> Self {
        let menu = Self {
            disp,
            depth: 0,
            root: SubMenu {
                name: "root",
                position: RefCell::new(0),
                items,
            },
        };
        menu.display();
        menu
    }

    /// go forward in the menu
    pub fn skip(&mut self) {
        let skip_back = self.depth == 0;
        self.get_submenu().go_next(skip_back);
        self.display();
    }

    /// accept current selection
    pub fn ok(&mut self) {
        let menu = self.get_submenu();
        let item = menu.get_item();
        match item {
            Item::Action(action) => (action.f)(),
            Item::SubMenu(_) => self.depth += 1,
            Item::Back => {
                *menu.position.borrow_mut() = 0;
                self.depth -= 1;
            }
        }

        self.display();
    }

    fn display(&self) {
        let text = self.get_submenu().get_text();
        (self.disp)(text);
    }

    /// get currently active submenu
    fn get_submenu(&self) -> &SubMenu {
        let mut menu = &self.root;
        for _ in 0..self.depth {
            if let Item::SubMenu(sub) = &menu.items[*menu.position.borrow() as usize] {
                menu = sub;
            } else {
                panic!("attemped to select sub_menu on wrong item");
            }
        }
        menu
    }
}
