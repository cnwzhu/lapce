use std::rc::Rc;

use floem::{
    View,
    event::EventListener,
    peniko::Color,
    reactive::{SignalGet, SignalUpdate, create_rw_signal},
    style::CursorStyle,
    views::{Decorators, label, scroll, stack},
};

use crate::{
    config::color::LapceColor,
    text_input::TextInputBuilder,
    window_tab::{Focus, WindowTabData},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Main,
    SshTunnel,
    Ssl,
    Advanced,
}

pub fn database_editor_view(window_tab_data: Rc<WindowTabData>) -> impl View {
    let config = window_tab_data.common.config;
    let focus = window_tab_data.common.focus;
    let editors = window_tab_data.main_split.editors;
    let common = window_tab_data.common.clone();
    let cx = window_tab_data.common.scope;

    let active_tab = create_rw_signal(Tab::Main);

    let input_field = move |label_text: &str,
                            placeholder_text: &str,
                            _is_password: bool,
                            flex_grow: bool| {
        let label_text = label_text.to_string();
        let placeholder_text = placeholder_text.to_string();

        stack((
            label(move || label_text.clone()).style(move |s| {
                s.font_bold()
                    .font_size(12.0) // Small label size
                    .margin_bottom(4.0)
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
            }),
            TextInputBuilder::new()
                .build(cx, editors, common.clone())
                .placeholder(move || placeholder_text.clone())
                .keyboard_navigable()
                .style(move |s| {
                    s.width_pct(100.0)
                        .padding_horiz(8.0)
                        .padding_vert(6.0)
                        .border(1.0)
                        .border_radius(4.0)
                        .border_color(config.get().color(LapceColor::LAPCE_BORDER))
                        .background(
                            config.get().color(LapceColor::EDITOR_BACKGROUND),
                        )
                }),
        ))
        .style(move |s| s.flex_col().apply_if(flex_grow, |s| s.flex_grow(1.0)))
    };

    let title = label(|| "MySQL".to_string()).style(move |s| {
        s.font_bold()
            .font_size(20.0)
            .margin_bottom(10.0)
            .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
    });

    let alert_box = stack((label(|| {
        "Connection Failed: connect ENOENT /var/run/mysqld/mysqld.sock".to_string()
    })
    .style(move |s| s.color(Color::from_rgb8(200, 50, 50))),))
    .style(move |s| {
        s.width_pct(100.0)
            .padding(10.0)
            .margin_bottom(20.0)
            .border(1.0)
            .border_radius(4.0)
            .border_color(Color::from_rgb8(255, 200, 200))
            .background(Color::from_rgba8(255, 50, 50, 20))
    });

    // Top Section: Connection Name and Group
    let top_section = stack((
        input_field("CONNECTION NAME", "Local MySQL", false, true)
            .style(|s| s.margin_right(20.0)),
        input_field("GROUP", "Parent/Sub", false, true),
    ))
    .style(|s| s.width_pct(100.0).margin_bottom(15.0));

    // Tabs Placeholder
    let tabs = stack((
        label(|| "Main".to_string())
            .on_click_stop(move |_| active_tab.set(Tab::Main))
            .style(move |s| {
                s.padding_horiz(10.0)
                    .padding_vert(5.0)
                    .border_bottom(2.0)
                    .border_color(if active_tab.get() == Tab::Main {
                        config.get().color(LapceColor::LAPCE_TAB_ACTIVE_FOREGROUND)
                    } else {
                        Color::TRANSPARENT
                    })
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                    .font_bold()
                    .cursor(CursorStyle::Pointer)
            }),
        label(|| "SSH Tunnel".to_string())
            .on_click_stop(move |_| active_tab.set(Tab::SshTunnel))
            .style(move |s| {
                s.padding_horiz(10.0)
                    .padding_vert(5.0)
                    .border_bottom(2.0)
                    .border_color(if active_tab.get() == Tab::SshTunnel {
                        config.get().color(LapceColor::LAPCE_TAB_ACTIVE_FOREGROUND)
                    } else {
                        Color::TRANSPARENT
                    })
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                    .cursor(CursorStyle::Pointer)
            }),
        label(|| "SSL".to_string())
            .on_click_stop(move |_| active_tab.set(Tab::Ssl))
            .style(move |s| {
                s.padding_horiz(10.0)
                    .padding_vert(5.0)
                    .border_bottom(2.0)
                    .border_color(if active_tab.get() == Tab::Ssl {
                        config.get().color(LapceColor::LAPCE_TAB_ACTIVE_FOREGROUND)
                    } else {
                        Color::TRANSPARENT
                    })
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                    .cursor(CursorStyle::Pointer)
            }),
        label(|| "Advanced".to_string())
            .on_click_stop(move |_| active_tab.set(Tab::Advanced))
            .style(move |s| {
                s.padding_horiz(10.0)
                    .padding_vert(5.0)
                    .border_bottom(2.0)
                    .border_color(if active_tab.get() == Tab::Advanced {
                        config.get().color(LapceColor::LAPCE_TAB_ACTIVE_FOREGROUND)
                    } else {
                        Color::TRANSPARENT
                    })
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                    .cursor(CursorStyle::Pointer)
            }),
    ))
    .style(move |s| {
        s.width_pct(100.0)
            .margin_bottom(15.0)
            .border_bottom(1.0)
            .border_color(config.get().color(LapceColor::LAPCE_BORDER))
    });

    // Main Form
    let main_form = stack((
        // Host & Port
        stack((
            input_field("Host", "127.0.0.1", false, true)
                .style(|s| s.flex_grow(3.0).margin_right(10.0)),
            input_field("Port", "3306", false, true).style(|s| s.flex_grow(1.0)),
        ))
        .style(|s| s.width_pct(100.0).margin_bottom(10.0)),
        // User & Password
        stack((
            input_field("Username", "root", false, true)
                .style(|s| s.flex_grow(1.0).margin_right(10.0)),
            input_field("Password", "........", true, true)
                .style(|s| s.flex_grow(1.0)),
        ))
        .style(|s| s.width_pct(100.0).margin_bottom(10.0)),
        // Database
        input_field("Database", "Default database (optional)", false, false)
            .style(|s| s.width_pct(100.0).margin_bottom(10.0)),
        // Socket Path
        input_field(
            "Socket Path",
            "e.g. /var/run/mysqld/mysqld.sock",
            false,
            false,
        )
        .style(|s| s.width_pct(100.0).margin_bottom(20.0)),
    ))
    .style(move |s| {
        s.flex_col()
            .width_pct(100.0)
            .apply_if(active_tab.get() != Tab::Main, |s| s.hide())
    });

    // Footer
    let footer = stack((
        // Toggles can be implemented later or just placeholders

        // Buttons
        stack((
            label(|| "Test Connection".to_string()).style(move |s| {
                s.padding_horiz(12.0)
                    .padding_vert(6.0)
                    .margin_right(10.0)
                    .border(1.0)
                    .border_radius(4.0)
                    .border_color(config.get().color(LapceColor::LAPCE_BORDER))
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                    .cursor(CursorStyle::Pointer)
            }),
            label(|| "Connect".to_string()).style(move |s| {
                s.padding_horiz(12.0)
                    .padding_vert(6.0)
                    .background(
                        config
                            .get()
                            .color(LapceColor::LAPCE_BUTTON_PRIMARY_BACKGROUND),
                    )
                    .color(
                        config
                            .get()
                            .color(LapceColor::LAPCE_BUTTON_PRIMARY_FOREGROUND),
                    )
                    .border_radius(4.0)
                    .cursor(CursorStyle::Pointer)
            }),
        ))
        .style(|s| s.justify_end().flex_grow(1.0).items_center()),
    ))
    .style(|s| s.width_pct(100.0).items_center().margin_top(10.0));

    let content = stack((
        title,
        alert_box, // Showing the error state as requested/mocked
        top_section,
        tabs,
        main_form,
        footer,
    ))
    .on_event_cont(EventListener::PointerDown, move |_| {
        if focus.get_untracked() != Focus::Workbench {
            focus.set(Focus::Workbench);
        }
    })
    .style(|s| s.flex_col().width_pct(100.0).padding(20.0).max_width(800.0));

    scroll(content).style(|s| s.size_pct(100.0, 100.0))
}
