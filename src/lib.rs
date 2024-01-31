#![no_std]

use core::cell::RefCell;

#[derive(Clone)]
pub enum ItemType<'a> {
    Action(&'a (dyn Fn() + 'a)),
    SubItem(&'a RefCell<Item<'a>>),
    Back(&'a RefCell<Item<'a>>),
}

#[derive(Clone)]
pub struct Item<'a> {
    name: &'static str,
    inner: ItemType<'a>,
    next: Option<&'a RefCell<Item<'a>>>,
}

impl<'a> Item<'a> {
    pub fn new_action(name: &'static str, f: &'a (impl Fn() + 'a)) -> RefCell<Self> {
        RefCell::new(Self {
            name,
            inner: ItemType::Action(f),
            next: None,
        })
    }

    /// create a new submenu with some items
    /// panics if the items list is empty
    pub fn new_submenu(name: &'static str, items: &'a [RefCell<Item<'a>>]) -> RefCell<Self> {
        if items.is_empty() {
            panic!("no items given");
        }
        let root = Self {
            name,
            inner: ItemType::SubItem(&items[0]),
            next: None,
        };
        for i in 0..items.len() - 1 {
            items[i].borrow_mut().next = Some(&items[i + 1]);
        }
        RefCell::new(root)
    }
}

pub struct Menu<'a> {
    current: RefCell<Item<'a>>,
    display_text: &'a (dyn Fn(&'static str) + 'a),
}

impl<'a> Menu<'a> {
    pub fn new(items: &'a mut [RefCell<Item<'a>>], disp: &'a (impl Fn(&'static str) + 'a)) -> Self {
        let _ = Item::new_submenu("root", items);
        let menu = Self {
            current: items[0].clone(),
            display_text: disp,
        };
        menu.display();
        menu
    }

    /// go forward in the menu
    pub fn skip(&mut self) {
        let current = self.current.borrow().clone();
        if let Some(next) = current.next {
            self.current = next.clone();
        }
        self.display();
    }

    /// accept current selection
    pub fn ok(&mut self) {
        match self.current.clone().borrow().inner {
            ItemType::Action(action) => action(),
            ItemType::SubItem(sub) => self.current = sub.clone(),
            ItemType::Back(back) => self.current = back.clone(),
        }

        self.display();
    }

    fn display(&self) {
        (self.display_text)(self.current.borrow().name);
    }
}
