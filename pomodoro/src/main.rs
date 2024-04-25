slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    // this vector will store all of the users tasks
    


    let ui: AppWindow = AppWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });
    ui.on_current_task(move |string|{
        let parsed = string.trim();
        println!("{}", parsed);
        
    });
    ui.run()
}
