use std::rc::Rc;

use floem::{
    View,
    reactive::SignalGet,
    style::CursorStyle,
    views::{Decorators, empty, label, stack, text},
};

use crate::{
    app::clickable_icon,
    config::{color::LapceColor, icon::LapceIcons},
    panel::position::PanelPosition,
    window_tab::WindowTabData,
};

pub fn database_panel(
    window_tab_data: Rc<WindowTabData>,
    _position: PanelPosition,
) -> impl View {
    let config = window_tab_data.common.config;

    stack((
        stack((
            text("Database").style(move |s| {
                s.font_bold()
                    .color(config.get().color(LapceColor::PANEL_FOREGROUND))
            }),
            empty().style(|s| s.flex_grow(1.0)),
            clickable_icon(
                || LapceIcons::ADD,
                {
                    let window_tab_data = window_tab_data.clone();
                    move || {
                        window_tab_data.main_split.open_database();
                    }
                },
                || false,
                || false,
                || "New",
                config,
            )
            .style(|s| s.padding(4.0)),
            clickable_icon(
                || LapceIcons::SETTINGS,
                || {},
                || false,
                || false,
                || "Data Source Properties",
                config,
            )
            .style(|s| s.padding(4.0)),
            clickable_icon(
                || LapceIcons::DEBUG_RESTART,
                || {},
                || false,
                || false,
                || "Refresh",
                config,
            )
            .style(|s| s.padding(4.0)),
        ))
        .style(move |s| {
            s.width_pct(100.0)
                .padding_horiz(10.0)
                .padding_vert(2.0)
                .items_center()
                .background(config.get().color(LapceColor::EDITOR_BACKGROUND))
                .border_bottom(1.0)
                .border_color(config.get().color(LapceColor::LAPCE_BORDER))
        }),
        stack((stack((
            text("No data sources").style(move |s| {
                s.color(config.get().color(LapceColor::EDITOR_FOREGROUND))
            }),
            label(|| "Create data source... (Alt+Insert)".to_string())
                .style(move |s| {
                    s.color(config.get().color(LapceColor::EDITOR_LINK))
                        .cursor(CursorStyle::Pointer)
                        .margin_top(6.0)
                })
                .on_click_stop(move |_| {
                    window_tab_data.main_split.open_database();
                }),
        ))
        .style(|s| {
            s.flex_col()
                .items_center()
                .justify_center()
                .size_pct(100.0, 100.0)
                .padding(20.0)
        }),))
        .style(|s| s.flex_col().size_pct(100.0, 100.0)),
    ))
    .style(|s| s.flex_col().size_pct(100.0, 100.0))
}
