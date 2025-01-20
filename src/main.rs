use iced::widget::{container, text, text_editor};
use iced::{application, run, Element, Theme};

fn main() -> iced::Result {
    application("Editor", Editor::update, Editor::view).run()
}

#[derive(Default)]
struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Editor {
    fn new() -> Self {
        Self {
            content: text_editor::Content::with_text(include_str!("main.rs")),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input: Element<'_, Message> = text_editor::<Message, Theme, iced::Renderer>(&self.content) // Explicit generic arguments
            .on_action(Message::Edit)
            .into();

        container(input).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
