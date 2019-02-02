use crate::segment::Segment;
use amethyst::ui::UiText;
use chrono::Local;
use uuid::Uuid;

#[derive(Default)]
pub struct Date {
    id: String,
}

impl Segment for Date {
    type Item = UiText;

    fn new() -> Self {
        Date {
            id: Uuid::new_v4().to_string(),
        }
    }

    fn update(&mut self, t: &mut UiText) {
        t.text = Local::now().format(" %T  %a %e %b W%V").to_string();
    }

    fn id(&self) -> &str {
        &self.id
    }
}
