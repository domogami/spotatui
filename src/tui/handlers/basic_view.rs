use crate::core::app::App;
use crate::tui::event::Key;

pub fn handler(key: Key, app: &mut App) {
  if let Key::Char('s') = key {
    super::playbar::toggle_like_currently_playing_item(app);
  }
}
