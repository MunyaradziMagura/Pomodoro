use std::rc::Rc;

use slint::{ModelRc, SharedVector, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // this vector will store all of the users tasks
    
    let ui: AppWindow = AppWindow::new().unwrap();

    // https://docs.rs/slint/latest/slint/struct.ModelRc.html
    //let collected_tasks_clone = ui.get_collected_tasks().clone();
    let the_model : Rc<VecModel<slint::SharedString>> =
    Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
// Convert it to a ModelRc.
let the_model_rc = ModelRc::from(the_model.clone());
// Pass the model to the ui: The generated set_the_model setter from the
// the_model property takes a ModelRc.

ui.set_collected_tasks(the_model_rc);


    ui.on_current_task(move |string|{

      // Trim the parsed string
    let parsed = string.trim().to_string();
    println!("{}", parsed);
    the_model.push(parsed.into());

    
    });
    ui.run()
}
