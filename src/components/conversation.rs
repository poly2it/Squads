use iced::widget::{column, container, mouse_area, text};
use iced::{border, Color, Element};

use crate::Message;

use crate::api::Conversation;
use crate::components::message::c_message;

pub fn c_conversation<'a>(conversation: Conversation, show_replies: bool) -> Element<'a, Message> {
    let mut message_chain = column![].spacing(20);

    let ordered_conversation: Vec<_> = conversation.messages.iter().rev().cloned().collect();

    let first_message = ordered_conversation.get(0).unwrap().clone();
    if let Some(message_element) = c_message(first_message) {
        message_chain = message_chain.push(message_element);
    }

    if show_replies && ordered_conversation.len() > 1 {
        message_chain = message_chain.push(
            mouse_area(
                text("show replies")
                    .color(Color::from_rgb(0.4, 0.5961, 0.851))
                    .size(14),
            )
            .on_release(Message::Join),
        );

        for message in ordered_conversation.iter().skip(1).cloned() {
            if let Some(message_element) = c_message(message) {
                message_chain = message_chain.push(message_element);
            }
        }
    }

    container(message_chain)
        .style(|_| container::Style {
            background: Some(
                Color::parse("#333")
                    .expect("Background color is invalid.")
                    .into(),
            ),
            border: border::rounded(8),
            ..Default::default()
        })
        .width(iced::Length::Fill)
        .padding(20)
        .into()
}
