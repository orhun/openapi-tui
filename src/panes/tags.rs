use std::sync::{Arc, RwLock};

use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
  prelude::*,
  widgets::{block::*, *},
};

use crate::{
  action::Action,
  pages::home::State,
  panes::Pane,
  tui::{EventResponse, Frame},
};

#[derive(Default)]
pub struct TagsPane {
  focused: bool,
  focused_border_style: Style,
  state: Arc<RwLock<State>>,
  current_tag_index: usize,
}

impl TagsPane {
  pub fn new(state: Arc<RwLock<State>>, focused: bool, focused_border_style: Style) -> Self {
    Self { focused, focused_border_style, state, current_tag_index: 0 }
  }

  fn border_style(&self) -> Style {
    match self.focused {
      true => self.focused_border_style,
      false => Style::default(),
    }
  }

  fn border_type(&self) -> BorderType {
    match self.focused {
      true => BorderType::Thick,
      false => BorderType::Plain,
    }
  }

  fn update_active_tag(&mut self) {
    let mut state = self.state.write().unwrap();
    if self.current_tag_index > 0 {
      if let Some(tag) = state.openapi_spec.tags.get(self.current_tag_index - 1) {
        state.active_tag_name = Some(tag.name.clone());
        state.active_operation_index = 0;
      }
    } else {
      state.active_tag_name = None;
      state.active_operation_index = 0;
    }
  }
}
impl Pane for TagsPane {
  fn init(&mut self) -> Result<()> {
    Ok(())
  }

  fn focus(&mut self) -> Result<()> {
    self.focused = true;
    Ok(())
  }

  fn unfocus(&mut self) -> Result<()> {
    self.focused = false;
    Ok(())
  }

  fn height_constraint(&self) -> Constraint {
    match self.focused {
      true => Constraint::Fill(3),
      false => Constraint::Fill(1),
    }
  }

  fn handle_key_events(&mut self, _key: KeyEvent) -> Result<Option<EventResponse<Action>>> {
    Ok(None)
  }

  #[allow(unused_variables)]
  fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<EventResponse<Action>>> {
    Ok(None)
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Down => {
        {
          let state = self.state.read().unwrap();
          let tags_list_len = state.openapi_spec.tags.len().saturating_add(1);
          if tags_list_len > 0 {
            self.current_tag_index = self.current_tag_index.saturating_add(1) % tags_list_len;
          }
        }
        self.update_active_tag();
        return Ok(Some(Action::Update));
      },
      Action::Up => {
        {
          let state = self.state.read().unwrap();
          let tags_list_len = state.openapi_spec.tags.len().saturating_add(1);
          if tags_list_len > 0 {
            self.current_tag_index = self.current_tag_index.saturating_add(tags_list_len - 1) % tags_list_len;
          }
        }
        self.update_active_tag();
        return Ok(Some(Action::Update));
      },
      Action::Submit => {},
      _ => {},
    }

    Ok(None)
  }

  fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
    let state = self.state.read().unwrap();
    let mut items: Vec<Line<'_>> = state
      .openapi_spec
      .tags
      .iter()
      .map(|tag| Line::from(vec![Span::styled(format!(" {}", tag.name), Style::default())]))
      .collect();

    items.insert(0, Line::styled(" [ALL]", Style::default()));

    let list = List::new(items)
      .block(Block::default().borders(Borders::ALL))
      .highlight_symbol(symbols::scrollbar::HORIZONTAL.end)
      .highlight_spacing(HighlightSpacing::Always)
      .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    let mut list_state = ListState::default().with_selected(Some(self.current_tag_index));

    frame.render_stateful_widget(list, area, &mut list_state);
    let items_len = state.openapi_spec.tags.len() + 1;
    frame.render_widget(
      Block::default()
        .title("Tags")
        .borders(Borders::ALL)
        .border_style(self.border_style())
        .border_type(self.border_type())
        .title_bottom(
          Line::from(format!("{} of {}", self.current_tag_index.saturating_add(1), items_len)).right_aligned(),
        ),
      area,
    );
    Ok(())
  }
}
