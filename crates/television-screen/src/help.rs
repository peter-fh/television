use super::layout::HelpBarLayout;
use crate::colors::{Colorscheme, GeneralColorscheme};
use crate::logo::build_logo_paragraph;
use crate::metadata::build_metadata_table;
use crate::mode::{mode_color, Mode};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Table};
use ratatui::Frame;
use television_channels::channels::UnitChannel;
use television_utils::metadata::AppMetadata;

pub fn draw_logo_block(
    f: &mut Frame,
    area: Rect,
    mode_color: Color,
    general_colorscheme: &GeneralColorscheme,
) {
    let logo_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(general_colorscheme.border_fg))
        .style(
            Style::default()
                .fg(mode_color)
                .bg(general_colorscheme.background),
        )
        .padding(Padding::horizontal(1));

    let logo_paragraph = build_logo_paragraph().block(logo_block);

    f.render_widget(logo_paragraph, area);
}

fn draw_metadata_block(
    f: &mut Frame,
    area: Rect,
    mode: Mode,
    current_channel: UnitChannel,
    app_metadata: &AppMetadata,
    colorscheme: &Colorscheme,
) {
    let metadata_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colorscheme.general.border_fg))
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(colorscheme.general.background));

    let metadata_table =
        build_metadata_table(mode, current_channel, app_metadata, colorscheme)
            .block(metadata_block);

    f.render_widget(metadata_table, area);
}

fn draw_keymaps_block(
    f: &mut Frame,
    area: Rect,
    keymap_table: Table,
    colorscheme: &GeneralColorscheme,
) {
    let keymaps_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colorscheme.border_fg))
        .style(Style::default().bg(colorscheme.background))
        .padding(Padding::horizontal(1));

    let table = keymap_table.block(keymaps_block);

    f.render_widget(table, area);
}

pub fn draw_help_bar(
    f: &mut Frame,
    layout: &Option<HelpBarLayout>,
    current_channel: UnitChannel,
    keymap_table: Table,
    mode: Mode,
    app_metadata: &AppMetadata,
    colorscheme: &Colorscheme,
) {
    if let Some(help_bar) = layout {
        draw_metadata_block(
            f,
            help_bar.left,
            mode,
            current_channel,
            app_metadata,
            colorscheme,
        );
        draw_keymaps_block(
            f,
            help_bar.middle,
            keymap_table,
            &colorscheme.general,
        );
        draw_logo_block(
            f,
            help_bar.right,
            mode_color(mode, &colorscheme.mode),
            &colorscheme.general,
        );
    }
}
