use std::str::FromStr;

mod file;
mod ui;

fn main() -> iced::Result {
    println!("Hello, world!");

    let path = std::path::PathBuf::from_str(std::env::args().nth(1).unwrap().as_str()).unwrap();

    iced::application("TagthingX", ui::TagthingX::update, ui::TagthingX::view)
        .theme(|_| iced::Theme::GruvboxDark)
        .run_with(|| {
            (
                ui::TagthingX::new(file::File::read_directory(path.into())),
                iced::task::Task::none(),
            )
        })
}
