use slint::ToSharedString;
use slint_app::calc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let main_window_weak_1 = main_window.as_weak();
    let main_window_weak_2 = main_window_weak_1.clone();

    main_window.on_x_input(move |msg| {
        let main_window = main_window_weak_1.unwrap();
        main_window.set_x_corner(calc::calc_chunk_corner(msg.as_str().parse::<isize>().unwrap_or(0)).to_shared_string() );
    });

    main_window.on_y_input(move |msg| {
        let main_window = main_window_weak_2.unwrap();
        main_window.set_y_corner(calc::calc_chunk_corner(msg.as_str().parse::<isize>().unwrap_or(0)).to_shared_string() );
    });

    main_window.run()
}
