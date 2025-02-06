// NOTE: the buttons, layout and text
use iced::widget::{button, column, text, Column};
use iced::Center;

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

// NOTE: state
#[derive(Default)]
struct Counter {
    value: i64,
}

// NOTE: Messages
#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

// NOTE: update logic
impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_initialization() {
        let counter = Counter::default();
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = Counter::default();
        counter.update(Message::Increment);
        assert_eq!(counter.value, 1);
    }

    #[test]
    fn test_decrement() {
        let mut counter = Counter::default();
        counter.update(Message::Increment);
        counter.update(Message::Decrement);
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_multiple_operations() {
        let mut counter = Counter::default();
        counter.update(Message::Increment);
        counter.update(Message::Increment);
        counter.update(Message::Decrement);
        counter.update(Message::Increment);
        assert_eq!(counter.value, 2);
    }
}
