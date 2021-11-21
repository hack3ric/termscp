//! ## Log
//!
//! log tab component

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{Msg, UiMsg};

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, AttrValue, Attribute, Borders, Color, Style, Table};
use tuirealm::tui::layout::Corner;
use tuirealm::tui::widgets::{List as TuiList, ListItem, ListState};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, Props, State, StateValue};

pub struct Log {
    props: Props,
    states: OwnStates,
}

impl Log {
    pub fn new(lines: Table, fg: Color, bg: Color) -> Self {
        let mut props = Props::default();
        props.set(
            Attribute::Borders,
            AttrValue::Borders(Borders::default().color(fg)),
        );
        props.set(Attribute::Background, AttrValue::Color(bg));
        props.set(Attribute::Content, AttrValue::Table(lines));
        Self {
            props,
            states: OwnStates::default(),
        }
    }
}

impl MockComponent for Log {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        let width: usize = area.width as usize - 4;
        let focus = self
            .props
            .get_or(Attribute::Focus, AttrValue::Flag(false))
            .unwrap_flag();
        let fg = self
            .props
            .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
            .unwrap_color();
        let bg = self
            .props
            .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
            .unwrap_color();
        // Make list
        let list_items: Vec<ListItem> = self
            .props
            .get(Attribute::Content)
            .unwrap()
            .unwrap_table()
            .iter()
            .map(|row| ListItem::new(tui_realm_stdlib::utils::wrap_spans(row, width, &self.props)))
            .collect();
        let w = TuiList::new(list_items)
            .block(tui_realm_stdlib::utils::get_block(
                Borders::default().color(fg),
                Some(("Log".to_string(), Alignment::Left)),
                focus,
                None,
            ))
            .start_corner(Corner::BottomLeft)
            .highlight_symbol(">> ")
            .style(Style::default().bg(bg))
            .highlight_style(Style::default());
        let mut state: ListState = ListState::default();
        state.select(Some(self.states.get_list_index()));
        frame.render_stateful_widget(w, area, &mut state);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
        if matches!(attr, Attribute::Content) {
            self.states.set_list_len(
                match self.props.get(Attribute::Content).map(|x| x.unwrap_table()) {
                    Some(spans) => spans.len(),
                    _ => 0,
                },
            );
            self.states.reset_list_index();
        }
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.states.get_list_index()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Down) => {
                let prev = self.states.get_list_index();
                self.states.incr_list_index();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.states.get_list_index();
                self.states.decr_list_index();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Scroll(Direction::Down) => {
                let prev = self.states.get_list_index();
                (0..8).for_each(|_| self.states.incr_list_index());
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.states.get_list_index();
                (0..8).for_each(|_| self.states.decr_list_index());
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::Begin) => {
                let prev = self.states.get_list_index();
                self.states.reset_list_index();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.get_list_index();
                self.states.list_index_at_last();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<Msg, NoUserEvent> for Log {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::Ui(UiMsg::LogTabbed)),
            _ => None,
        }
    }
}

// -- states

/// ## OwnStates
///
/// OwnStates contains states for this component
#[derive(Clone)]
struct OwnStates {
    list_index: usize, // Index of selected element in list
    list_len: usize,   // Length of file list
    focus: bool,       // Has focus?
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates {
            list_index: 0,
            list_len: 0,
            focus: false,
        }
    }
}

impl OwnStates {
    /// ### set_list_len
    ///
    /// Set list length
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// ### get_list_index
    ///
    /// Return current value for list index
    pub fn get_list_index(&self) -> usize {
        self.list_index
    }

    /// ### incr_list_index
    ///
    /// Incremenet list index
    pub fn incr_list_index(&mut self) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        }
    }

    /// ### decr_list_index
    ///
    /// Decrement list index
    pub fn decr_list_index(&mut self) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        }
    }

    /// ### list_index_at_last
    ///
    /// Set list index at last item
    pub fn list_index_at_last(&mut self) {
        self.list_index = match self.list_len {
            0 => 0,
            len => len - 1,
        };
    }

    /// ### reset_list_index
    ///
    /// Reset list index to last element
    pub fn reset_list_index(&mut self) {
        self.list_index = 0; // Last element is always 0
    }
}
