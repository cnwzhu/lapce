use std::rc::Rc;

use floem::{
    View,
    kurbo::Point,
    reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal},
    style::{CursorStyle, Position},
    views::{Decorators, container, dyn_stack, empty, label, stack},
};

use crate::config::{LapceConfig, color::LapceColor};

/// A menu item entry for the custom dropdown
#[derive(Clone)]
pub struct DropdownMenuItem {
    pub label: String,
    pub shortcut: Option<String>,
    pub action: Option<Rc<dyn Fn()>>,
    pub separator: bool,
}

impl DropdownMenuItem {
    pub fn new(label: impl Into<String>, action: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            action: Some(Rc::new(action)),
            separator: false,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn separator() -> Self {
        Self {
            label: String::new(),
            shortcut: None,
            action: None,
            separator: true,
        }
    }
}

/// State for the dropdown menu
#[derive(Clone)]
pub struct DropdownMenuState {
    pub visible: RwSignal<bool>,
    pub position: RwSignal<Point>,
    pub items: RwSignal<Vec<DropdownMenuItem>>,
}

impl DropdownMenuState {
    pub fn new() -> Self {
        Self {
            visible: create_rw_signal(false),
            position: create_rw_signal(Point::ZERO),
            items: create_rw_signal(Vec::new()),
        }
    }

    pub fn show(&self, pos: Point, items: Vec<DropdownMenuItem>) {
        self.position.set(pos);
        self.items.set(items);
        self.visible.set(true);
    }

    pub fn hide(&self) {
        self.visible.set(false);
    }

    pub fn toggle(&self, pos: Point, items: Vec<DropdownMenuItem>) {
        if self.visible.get_untracked() {
            self.hide();
        } else {
            self.show(pos, items);
        }
    }
}

impl Default for DropdownMenuState {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates the dropdown menu popup view
pub fn dropdown_menu_popup(
    state: DropdownMenuState,
    config: floem::reactive::ReadSignal<std::sync::Arc<LapceConfig>>,
) -> impl View {
    let state_for_backdrop = state.clone();
    let state_for_menu = state.clone();

    // Backdrop that closes menu when clicked
    let backdrop = container(empty())
        .style(move |s| {
            let visible = state_for_backdrop.visible.get();
            s.position(Position::Absolute)
                .inset_left(0.0)
                .inset_top(0.0)
                .size_pct(100.0, 100.0)
                .apply_if(!visible, |s| s.hide())
        })
        .on_click_stop(move |_| {
            state_for_backdrop.hide();
        });

    // Menu content using dyn_stack for dynamic items
    let menu = container(
        dyn_stack(
            move || state_for_menu.items.get(),
            |item| item.label.clone(),
            move |item| {
                let state_clone = state_for_menu.clone();
                if item.separator {
                    // Separator line
                    container(empty()).style(move |s| {
                        let config = config.get();
                        s.width_pct(100.0)
                            .height(1.0)
                            .margin_vert(4.0)
                            .margin_horiz(8.0)
                            .background(config.color(LapceColor::LAPCE_BORDER))
                    })
                } else {
                    let label_text = item.label.clone();
                    let shortcut_text = item.shortcut.clone();
                    let action = item.action.clone();

                    // Menu item with label and optional shortcut
                    container(
                        stack((
                            // Label on the left
                            label(move || label_text.clone()).style(move |s| {
                                let config = config.get();
                                s.color(config.color(LapceColor::EDITOR_FOREGROUND))
                                    .flex_grow(1.0)
                            }),
                            // Shortcut on the right (if present)
                            label(move || shortcut_text.clone().unwrap_or_default())
                                .style(move |s| {
                                    let config = config.get();
                                    s.color(config.color(LapceColor::EDITOR_DIM))
                                        .margin_left(32.0)
                                }),
                        ))
                        .style(|s| s.width_pct(100.0).items_center()),
                    )
                    .style(move |s| {
                        let config = config.get();
                        s.padding_horiz(12.0)
                            .padding_vert(6.0)
                            .width_pct(100.0)
                            .items_center()
                            .hover(|s| {
                                s.background(
                                    config
                                        .color(LapceColor::PANEL_HOVERED_BACKGROUND),
                                )
                                .cursor(CursorStyle::Pointer)
                            })
                    })
                    .on_click_stop(move |_| {
                        if let Some(ref action) = action {
                            action();
                        }
                        state_clone.hide();
                    })
                }
            },
        )
        .style(move |s| {
            let config = config.get();
            s.flex_col()
                .min_width(200.0)
                .padding_vert(4.0)
                .background(config.color(LapceColor::PANEL_BACKGROUND))
                .border(1.0)
                .border_radius(6.0)
                .border_color(config.color(LapceColor::LAPCE_BORDER))
                .box_shadow_blur(4.0)
                .box_shadow_spread(1.0)
                .box_shadow_color(config.color(LapceColor::LAPCE_DROPDOWN_SHADOW))
        }),
    )
    .style(move |s| {
        let pos = state.position.get();
        let visible = state.visible.get();
        s.position(Position::Absolute)
            .inset_left(pos.x)
            .inset_top(pos.y)
            .apply_if(!visible, |s| s.hide())
    });

    let state_for_container = state.clone();
    stack((backdrop, menu)).style(move |s| {
        let visible = state_for_container.visible.get();
        s.position(Position::Absolute)
            .inset_left(0.0)
            .inset_top(0.0)
            .size_pct(100.0, 100.0)
            .apply_if(!visible, |s| s.hide())
    })
}
