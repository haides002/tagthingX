use crate::file;

use iced::{
    widget::{button, column, combo_box, container, image, row, scrollable, text, Column, Row},
    Element, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    SelectImage(usize),
    SelectTag(String),
    AddTag(String),
}

#[derive(Debug, Default)]
pub struct TagthingX {
    files: Vec<crate::file::File>,
    selected: usize,
    image_selected: Option<image::Handle>,
    tag_search: combo_box::State<String>,
}

impl TagthingX {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectImage(id) => {
                self.selected = id;
                self.image_selected =
                    Some(image::Handle::from_path(self.files[id].image_path.clone()))
            }
            Message::SelectTag(_) => {}
            Message::AddTag(_) => {}
        }
    }

    pub fn new(files: Vec<file::File>) -> Self {
        Self {
            files: files.clone(),
            tag_search: combo_box::State::new({
                let mut cache: Vec<String> = Vec::new();

                for file in files {
                    match file.tags {
                        Some(tags) => cache.append(&mut tags.clone()),
                        None => {}
                    }
                }

                cache.sort();
                cache.dedup();

                cache
            }),

            ..Default::default()
        }
    }

    pub fn view(&self) -> Element<Message> {
        let filter_view = { container(column![]) };

        let gallery_view = {
            const IMAGES_PER_ROW: usize = 3;

            container(scrollable({
                let mut image_column: Column<_> = Column::<Message>::new();
                let mut image_row: Row<_> = Row::new();

                for (i, file) in self.files.iter().enumerate() {
                    image_row = image_row.push(
                        button(match &file.thumbnail_handle.clone() {
                            Some(handle) => container(image(handle)),
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
            image(self.files[self.selected].image_handle.clone()),
            row![{
                let mut tag_buttons = Row::new();

                match self.files[self.selected].tags.clone() {
                    Some(tags) => {
                        for tag in tags {
                            tag_buttons = tag_buttons
                                .push(button(text!("{}", tag)).on_press(Message::SelectTag(tag)));
                        }
                    }
                    None => {}
                };

                tag_buttons
            }],
            combo_box(&self.tag_search, "add tag", None, |selected| -> Message {
                Message::AddTag(selected)
            })
        ]));

        row![
            filter_view.width(Length::FillPortion(1)).padding(10),
            gallery_view.width(Length::FillPortion(3)).padding(10),
            details_view.width(Length::FillPortion(2)).padding(10)
        ]
        .into()
    }
}
