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
    ui.current_task(move |string|{
        let ui = ui_handle.unwrap();
        let parsed = string.trim().unwrap();
        let task_list = get_collected_tasks();
        println!(task_list);
        
    });
    ui.run()
}
