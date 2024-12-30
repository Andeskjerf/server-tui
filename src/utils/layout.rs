use ratatui::layout::{Constraint, Direction, Flex, Layout};

pub fn make_layout(dir: Direction, count: u16) -> Layout {
    let percentage = 100 / count;
    Layout::default()
        .direction(dir)
        .flex(Flex::Center)
        .constraints(vec![Constraint::Percentage(percentage); count as usize])
}
