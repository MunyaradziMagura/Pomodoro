use std::{
    borrow::{Borrow, BorrowMut},
    process::exit,
    rc::Rc,
    time::Duration,
};

use slint::{ModelRc, Timer, TimerMode, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // this vector will store all of the users tasks

    let ui: AppWindow = AppWindow::new().unwrap();

    // https://docs.rs/slint/latest/slint/struct.ModelRc.html
    let the_model: Rc<VecModel<slint::SharedString>> =
        Rc::new(VecModel::from(vec!["Task 1".into(), "Task 2".into()]));
    // Convert it to a ModelRc.
    let the_model_rc = ModelRc::from(the_model.clone());
    // Pass the model to the ui: The generated set_the_model setter from the
    // the_model property takes a ModelRc.
    let cloned_model_for_remove = the_model.clone(); // Clone for use inside the remove-task closure

    ui.set_collected_tasks(the_model_rc);

    ui.on_current_task(move |string| {
        // Trim the parsed string
        let parsed = string.trim().to_string();
        the_model.push(parsed.into());
    });
    ui.on_remove_task(move |idx| {
        cloned_model_for_remove.remove(idx.try_into().unwrap());
    });

    // function for calculating when a user should take a break,

    // code to handle the timer
    // code to handle the timer
    let timer = Timer::default();
    // these varibles will store the time values
    let mut seconds = 0;
    let mut minutes = 0;
    let mut hours = 0;

    let handle_weak = ui.as_weak();
    // code to handle the timer
    ui.on_timer_state(move |state_string, count_down| {
        if state_string == "start" {
            let handle_copy = handle_weak.clone();
            let mut remaining_time = Duration::from_secs((count_down * 60).try_into().unwrap());
            // track changes in the min hand
            let mut prev_minutes = 0;

            // track when the user should take a rest

            let mut working = 25;
            let mut resting: i32 = 5;

            timer.start(
                TimerMode::Repeated,
                std::time::Duration::from_millis(1),
                move || {
                    // flag for capturing when the timer should stop
                    if remaining_time.as_secs() > 0 {
                        hours = remaining_time.as_secs() / 3600;
                        minutes = (remaining_time.as_secs() % 3600) / 60;
                        seconds = remaining_time.as_secs() % 60;

                        remaining_time -= Duration::from_secs(1);

                        //let time_result = format!("Hours: {}, Minutes: {}, Seconds: {}", 0, 0, 0);

                        let time_result = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
                        handle_copy
                            .unwrap()
                            .set_my_time(time_result.trim().to_string().into());
                        // println!(
                        //     "Hours: {}, Minutes: {}, Seconds: {}",
                        //     hours, minutes, seconds
                        // );
                        // track when the user should work and when they should rest
                        if minutes != prev_minutes {
                            prev_minutes = minutes;
                            if working > 0 {
                                // user is working
                                let working_time_result = format!("Rest in {} min", working);

                                // current-pomodoro-state
                                handle_copy.unwrap().set_current_pomodoro_state(
                                    working_time_result.trim().to_string().into(),
                                );
                                working = working - 1;

                                println!("WORKING")
                            } else if working <= 0 && resting >= 1 {
                                // user is resting
                                let resting_time_result = format!("Working in {} min", resting);

                                handle_copy.unwrap().set_current_pomodoro_state(
                                    resting_time_result.trim().to_string().into(),
                                );
                                resting = resting - 1;

                                println!("RESTING")
                            } else {
                                // reset working and resting
                                working = 25;
                                resting = 5;
                                let working_time_result = format!("Rest in {} min", working);

                                // current-pomodoro-state
                                handle_copy.unwrap().set_current_pomodoro_state(
                                    working_time_result.trim().to_string().into(),
                                );
                            }
                        }
                    } else {
                        handle_copy.unwrap().set_my_time(
                            format!("{:02}:{:02}:{:02}", 0, 0, 0)
                                .trim()
                                .to_string()
                                .into(),
                        );
                        handle_copy
                            .unwrap()
                            .set_current_pomodoro_state("Complete!!".trim().to_string().into());
                    }
                },
            );
        } else if state_string == "reset" {
            println!(
                "reset pressed-- Hours: {}, Minutes: {}, Seconds: {}",
                hours, minutes, seconds
            );

            timer.stop();
            timer.restart();
        } else {
            timer.stop();
        }
    });
    ui.run()
}
