use std::usize;

use iced::{
    widget::{button, column, container, image, row, scrollable, text, Column, Row},
    Element, Length,
};

use crate::file;

#[derive(Debug, Clone)]
pub enum Message {
    SelectImage(usize),
    SelectTag(String),
}

#[derive(Debug, Default)]
pub struct TagthingX {
    files: Vec<crate::file::File>,
    selected: usize,
}

impl TagthingX {
    pub fn view(&self) -> Element<Message> {
        let filter_view = { container(column![]) };

        let gallery_view = {
            const IMAGES_PER_ROW: usize = 3;

            container(scrollable({
                let mut image_column: Column<_> = Column::<Message>::new();
                let mut image_row: Row<_> = Row::new();

                for (i, file) in self.files.iter().enumerate() {
                    image_row = image_row.push(
                        button(match &file.thumbnail_path.clone() {
                            Some(image_path) => {
                                container(image(image_path.clone().into_os_string()))
                            }
                            None => container(text!("could not find thumbnail")),
                        })
                        .width(Length::Fill)
                        .on_press(Message::SelectImage(i)),
                    );

                    if (i + 1) % IMAGES_PER_ROW == 0 {
                        image_column = image_column.push(image_row);
                        image_row = Row::new();
                    }
                }

                image_column.push(image_row)
            }))
        };

        let details_view = container(scrollable(column![
            image(self.files[self.selected].path.clone().into_os_string()),
            row![{
                let mut tags = Row::new();

                for tag in self.files[self.selected].tags.clone().unwrap() {
                    tags = tags.push(button(text!("{}", tag)).on_press(Message::SelectTag(tag)));
                }

                tags
            }],
        ]));

        row![
            filter_view.width(Length::FillPortion(1)).padding(10),
            gallery_view.width(Length::FillPortion(3)).padding(10),
            details_view.width(Length::FillPortion(2)).padding(10)
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectImage(id) => self.selected = id,
            Message::SelectTag(_) => {}
        }
    }

    pub fn new(files: Vec<file::File>) -> Self {
        Self {
            files,
            ..Default::default()
        }
    }
}
