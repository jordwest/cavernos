mod elements;

use crate::cavernos;
use crate::geometry::rect::Rect;
use crate::state::AppState;
use crate::state::UiSelectedElement;
use elements::Element;

pub fn frame(os: &mut cavernos::CavernOS, app_state: &mut AppState, location: Rect) {
    use elements::{Border, Clear, Padding, Selector, Stack, Style, Text, TextAlign, TextSegment};
    let style = Style {
        bg_color: Some(0),
        fg_color: Some(1),
        child: Clear {
            child: Border::new(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
                child: Stack {
                    children: vec![
                        Box::new(Style {
                            bg_color: None,
                            fg_color: Some(2),
                            child: Text::from_segments(
                                vec![TextSegment::Text(format!("Program"))],
                                TextAlign::Middle,
                            ),
                        }),
                        Box::new(Selector {
                            is_active: match app_state.selected_element {
                                UiSelectedElement::ProgramSelector => true,
                                _ => false,
                            },
                            text: vec![TextSegment::Text(app_state.program.display_name().into())],
                        }),
                        Box::new(Style {
                            bg_color: None,
                            fg_color: Some(2),
                            child: Text::from_segments(
                                vec![TextSegment::Text(format!("Rows"))],
                                TextAlign::Middle,
                            ),
                        }),
                        Box::new(Selector {
                            is_active: match app_state.selected_element {
                                UiSelectedElement::RowSelector => true,
                                _ => false,
                            },
                            text: vec![TextSegment::Text(format!("{}", app_state.rows))],
                        }),
                        Box::new(Style {
                            bg_color: None,
                            fg_color: Some(2),
                            child: Text::from_segments(
                                vec![TextSegment::Text(format!("Columns"))],
                                TextAlign::Middle,
                            ),
                        }),
                        Box::new(Selector {
                            is_active: match app_state.selected_element {
                                UiSelectedElement::ColumnSelector => true,
                                _ => false,
                            },
                            text: vec![TextSegment::Text(format!("{}", app_state.cols))],
                        }),
                    ],
                },
            }),
        },
    };
    let root = style;

    root.render(os, location);
}
