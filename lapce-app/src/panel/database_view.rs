use std::rc::Rc;

use floem::{
    View,
    reactive::{ReadSignal, SignalGet},
    style::CursorStyle,
    views::{Decorators, container, label, stack, text},
};

use crate::{
    config::{LapceConfig, color::LapceColor},
    window_tab::WindowTabData,
};

use super::{position::PanelPosition, view::PanelBuilder};

pub fn database_panel(
    window_tab_data: Rc<WindowTabData>,
    position: PanelPosition,
) -> impl View {
    let config = window_tab_data.common.config;

    PanelBuilder::new(config, position)
        .add(
            "Database",
            database_content(config),
            window_tab_data
                .panel
                .section_open(super::data::PanelSection::Database),
        )
        .build()
}

fn database_content(config: ReadSignal<std::sync::Arc<LapceConfig>>) -> impl View {
    stack((
        text("You haven't created any connections learn more.").style(move |s| {
            s.color(config.get().color(LapceColor::EDITOR_FOREGROUND))
        }),
        container(label(|| "Create Connection".to_string()).style(move |s| {
            s.color(config.get().color(LapceColor::EDITOR_BACKGROUND))
        }))
        .style(move |s| {
            s.background(config.get().color(LapceColor::EDITOR_FOREGROUND))
                .padding_horiz(10.0)
                .padding_vert(6.0)
                .border_radius(4.0)
                .cursor(CursorStyle::Pointer)
                .margin_top(10.0)
        })
        .on_click_stop(|_| {
            println!("Create Connection clicked");
        }),
    ))
    .style(|s| {
        s.flex_col()
            .items_center()
            .justify_center()
            .size_pct(100.0, 100.0)
            .padding(20.0)
    })
    .style(|s| s.size_pct(100.0, 100.0))
}
