use std::{rc::Rc, time::Duration};

use slint::{ModelRc, Timer, TimerMode, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // this vector will store all of the users tasks
    
    let ui: AppWindow = AppWindow::new().unwrap();

    // https://docs.rs/slint/latest/slint/struct.ModelRc.html
    let the_model : Rc<VecModel<slint::SharedString>> =
    Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
// Convert it to a ModelRc.
let the_model_rc = ModelRc::from(the_model.clone());
// Pass the model to the ui: The generated set_the_model setter from the
// the_model property takes a ModelRc.
let cloned_model_for_remove = the_model.clone(); // Clone for use inside the remove-task closure

ui.set_collected_tasks(the_model_rc);

    ui.on_current_task(move |string|{
      // Trim the parsed string
    let parsed = string.trim().to_string();
    println!("{}", parsed);
    the_model.push(parsed.into());
    
    });
    ui.on_remove_task(move |idx| {
        cloned_model_for_remove.remove(idx.try_into().unwrap());
    });


    // code to handle the timer
    let mut timer = Timer::default();
    // these varibles will store the time values
    let mut seconds = 0;
    let mut minutes = 0;
    let mut hours = 0;

        
    

    // ui.on_stop_timer(move ||{
    //     timer.stop();
    //     println!("Stopped");

    // });
    ui.on_timer_state(move |string| {
        if string=="start" {
            timer.start(TimerMode::Repeated, std::time::Duration::from_millis(1000), move || {                
                seconds += 1;
                if seconds >= 60 {
                    minutes += 1;
                    seconds = 0;
                }
                if minutes >= 60{
                    hours += 1;
                    minutes = 0;
                } 
                if hours >= 60{
                    seconds = 0;
                    minutes = 0;
                    hours = 0;
                } 
        
                println!("Hours: {}, Minutes: {}, Seconds: {}", hours, minutes, seconds);
                
                });
        }else if string=="reset" {
            println!("Hours:");
            seconds = 0;
            minutes = 0;
            hours = 0;
            
            println!("reset pressed-- Hours: {}, Minutes: {}, Seconds: {}", hours, minutes, seconds);

            timer.restart();
            timer.restart();
        }else{
            timer.stop();
        }
        
    });
    ui.run()
}
