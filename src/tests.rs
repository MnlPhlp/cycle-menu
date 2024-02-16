use core::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use crate::{Item, Menu};

#[cfg(feature = "std")]
#[test]
fn test_std() {
    let counter1 = AtomicI32::new(0);
    let counter2 = AtomicI32::new(0);
    let flag = AtomicBool::new(false);
    let menu = Menu::new(
        vec![
            Item::new_submenu(
                "sub",
                vec![
                    Item::new_action("counter1", || {
                        counter1.fetch_add(1, Ordering::Relaxed);
                    }),
                    Item::new_action("counter2", || {
                        counter2.fetch_add(1, Ordering::Relaxed);
                    }),
                ],
            ),
            Item::new_action("flag", || flag.store(true, Ordering::Relaxed)),
        ],
        |_| {},
    );
    test_menu(menu, &flag, &counter1, &counter2);
}

#[cfg(not(feature = "std"))]
#[test]
fn test_no_std() {
    let counter1 = AtomicI32::new(0);
    let counter1_func = || {
        counter1.fetch_add(1, Ordering::Relaxed);
    };
    let counter2 = AtomicI32::new(0);
    let counter2_func = || {
        counter2.fetch_add(1, Ordering::Relaxed);
    };
    let flag = AtomicBool::new(false);
    let flag_func = || flag.store(true, Ordering::Relaxed);

    let sub_items = [
        Item::new_action("counter1", &counter1_func),
        Item::new_action("counter2", &counter2_func),
    ];
    let items = [
        Item::new_submenu("sub", &sub_items),
        Item::new_action("flag", &flag_func),
    ];
    let menu = Menu::new(&items, &|_| {});
    test_menu(menu, &flag, &counter1, &counter2);
}

fn test_menu(mut menu: Menu, flag: &AtomicBool, counter1: &AtomicI32, counter2: &AtomicI32) {
    // go into sub
    menu.ok();
    // skip to counter 2
    menu.skip();
    // trigger counter2
    menu.ok();
    // go back
    menu.skip();
    menu.ok();
    // set flag
    menu.skip();
    menu.ok();
    // go to sub again
    menu.skip();
    menu.ok();
    // trigger counter 1
    menu.ok();
    assert!(flag.load(Ordering::Relaxed), "falg not set");
    assert_eq!(counter1.load(Ordering::Relaxed), 1, "counter 1 wrong");
    assert_eq!(counter2.load(Ordering::Relaxed), 1, "counter 2 wrong");
}
