# cycle-menu

Simple no-std compatible library to create a menu that can be cycled through and shown with custom input/output.

The menu object has two functions, next and ok, to controll it.
When crating the menu a callback is given that is later used to display menu entries.
Two different Menu-Items exist:
 - Action: Contains a callback that is executed on selection
 - Submenu: Contains a list of Menu-Items

## Example
This example uses [edge-executor](https://crates.io/crates/edge-executor) to run two tasks that wait for button input and controll the menu.
The Items are printed on the console

```rust
pub fn setup<P1: Pin, P2: Pin>(
    ex: &LocalExecutor,
    btn1: PinDriver<'static, P1, Input>,
    btn2: PinDriver<'static, P2, Input>,
) {
    let menu = Box::leak(Box::new(Mutex::new(create_menu())));
    // left button
    ex.spawn(btn_task(btn1, || {
        if let Ok(mut menu) = menu.try_lock() {
            menu.skip();
        } else {
            info!("Menu is locked");
        };
    }))
    .detach();

    // right button
    ex.spawn(btn_task(btn2, || {
        if let Ok(mut menu) = menu.try_lock() {
            menu.ok();
        } else {
            info!("Menu is locked");
        };
    }))
    .detach();
}

fn create_menu() -> Menu<'static> {
    let items = vec![
        Item::new_action("Test 1", || println!("Test activated")),
        Item::new_submenu(
            "Sub",
            vec![
                Item::new_action("Sub 1", || println!("Sub 1 activated")),
                Item::new_action("Sub 2", || println!("Sub 2 activated")),
            ],
        ),
    ];
    Menu::new(items, display)
}

fn display(text: &str) {
    info!("Menu: {text}")
}
```
