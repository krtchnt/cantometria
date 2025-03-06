use std::{hint::unreachable_unchecked, ops::Mul};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().black())
        .style(Style::default());

    let title = match app.current_screen {
        CurrentScreen::Main => Paragraph::new(Line::from(vec![
            "canto".magenta().bold(),
            "metria".white().bold(),
        ])),
        CurrentScreen::SelectingMidi => Paragraph::new("Selecting Midi File"),
        CurrentScreen::SelectingWav => Paragraph::new("Selecting Wav File"),
        CurrentScreen::ConfirmingSelection => Paragraph::new("Confirming File Selection"),
        CurrentScreen::Grading => Paragraph::new("Singing Analysis"),
    }
    .centered()
    .block(title_block);
    frame.render_widget(title, chunks[0]);

    match app.current_screen {
        CurrentScreen::Main => {
            let main_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().magenta());
            frame.render_widget(main_block, chunks[1]);
            let chunks_main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
                .split(chunks[1].inner(Margin::new(1, 2)));

            let big_text = BigText::builder()
                .pixel_size(PixelSize::Full)
                .style(Style::new().blue())
                .lines(vec![vec!["canto".magenta(), "metria".white()].into()])
                .centered()
                .build();
            frame.render_widget(big_text, chunks_main[0]);

            let desc = Text::from("A TUI application to grade your singing!").centered();
            frame.render_widget(desc, chunks_main[1]);
        }
        CurrentScreen::SelectingMidi => {
            render_select_midi(frame, app, &chunks);
        }
        CurrentScreen::SelectingWav => {
            render_select_wav(frame, app, &chunks);
        }
        CurrentScreen::ConfirmingSelection => {
            render_confirm_select(frame, app, &chunks);
        }
        CurrentScreen::Grading => {
            render_grading(frame, app, &chunks);
        }
    }

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().black())
        .style(Style::default());

    let footer = match app.current_screen {
        CurrentScreen::Main => Paragraph::new(Line::from(vec![
            "(q) to quit".red(),
            " / ".into(),
            "(→) to get started".green(),
        ])),
        CurrentScreen::SelectingMidi | CurrentScreen::SelectingWav => {
            Paragraph::new(Line::from(vec![
                "(←) to go back".yellow(),
                " / ".into(),
                "(↓↑) to move".blue(),
                " / ".into(),
                "(→) to continue".green(),
            ]))
        }
        CurrentScreen::ConfirmingSelection => Paragraph::new(Line::from(vec![
            "(←) to go back".yellow(),
            " / ".into(),
            "(→) to confirm and analyse".magenta(),
        ])),
        CurrentScreen::Grading => Paragraph::new("(→) to return to main menu".cyan()),
    }
    .centered()
    .block(footer_block);
    frame.render_widget(footer, chunks[2]);
}

fn color_grade(grade: f64) -> Color {
    match grade {
        0.0..0.2 => Color::Black,
        0.2..0.4 => Color::Red,
        0.4..0.6 => Color::Yellow,
        0.6..0.8 => Color::Green,
        0.8..0.9 => Color::Blue,
        0.9..1.0 => Color::Cyan,
        1.0 => Color::Magenta,
        // SAFETY: grade is in [0.0, 1.0] so this will branch is unreachable.
        _ => unsafe { unreachable_unchecked() },
    }
}

fn render_grading(frame: &mut Frame<'_>, app: &App, chunks: &[Rect]) {
    let (Some(midi_path_idx), Some(wav_path_idx)) = (
        app.midi_path_list.state.selected(),
        app.wav_path_list.state.selected(),
    ) else {
        // SAFETY: the previous two screens cannot be proceeded unless a file is selected
        // so this will never happen
        unsafe { unreachable_unchecked() }
    };

    let block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().cyan());
    frame.render_widget(block, chunks[1]);

    let chunks_grade = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 6),
            Constraint::Ratio(2, 6),
            Constraint::Ratio(3, 6),
        ])
        .split(chunks[1].inner(Margin::new(1, 2)));

    let midi_file = app.midi_path_list.items[midi_path_idx].path();
    let wav_file = app.wav_path_list.items[wav_path_idx].path();
    let accuracy = cantometria_lib::run(midi_file, wav_file).expect("msg");

    let desc = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .lines(vec![Line::from("Your accuracy is:")])
        .centered()
        .build();
    frame.render_widget(desc, chunks_grade[0]);

    let total_accuracy_f = accuracy.total_accuracy();
    let total_accuracy = format!("{:.2}", total_accuracy_f.mul(100.));
    let total_acc_txt = BigText::builder()
        .pixel_size(PixelSize::Full)
        .lines(vec![Line::from(vec![total_accuracy.into()])])
        .style(Style::new().fg(color_grade(total_accuracy_f)))
        .centered()
        .build();
    frame.render_widget(total_acc_txt, chunks_grade[1]);

    let coverage = format!("{:.2}", accuracy.coverage.mul(100.));
    let timing = format!("{:.2}", accuracy.timing.mul(100.));
    let pitch = format!("{:.2}", accuracy.pitch.mul(100.));
    let key = format!("{:.2}", accuracy.key.mul(100.));
    let accuracy_txt = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .lines(vec![
            Line::from(vec![
                "Coverage: ".into(),
                coverage.fg(color_grade(accuracy.coverage)),
            ]),
            Line::from(vec![
                "Timing: ".into(),
                timing.fg(color_grade(accuracy.timing)),
            ]),
            Line::from(vec![
                "Pitch: ".into(),
                pitch.fg(color_grade(accuracy.pitch)),
            ]),
            Line::from(vec!["Key: ".into(), key.fg(color_grade(accuracy.key))]),
        ])
        .centered()
        .build();
    frame.render_widget(accuracy_txt, chunks_grade[2]);
}

fn render_select_midi(frame: &mut Frame<'_>, app: &mut App, chunks: &[Rect]) {
    let chunks_sel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)])
        .split(chunks[1]);

    let desc = Paragraph::new(
        "1. Select the MIDI file of the track of the reference melody or song used during singing.",
    )
    .bold()
    .centered()
    .green()
    .block(Block::new().padding(Padding::new(0, 0, chunks_sel[0].height / 3, 0)));

    frame.render_widget(desc, chunks_sel[0]);

    let list_block = Block::new()
        .title(Line::raw("MIDI File List").centered())
        .border_style(Style::new().green())
        .borders(Borders::ALL);

    let items: Vec<ListItem> = app
        .midi_path_list
        .items
        .iter()
        .map(|fp| ListItem::from(Text::from(fp.path().to_str().expect("msg"))))
        .collect();

    let list = List::new(items)
        .block(list_block)
        .highlight_style(Style::new().black().on_green().bold())
        .highlight_spacing(HighlightSpacing::WhenSelected);

    frame.render_stateful_widget(list, chunks_sel[1], &mut app.midi_path_list.state);
}

fn render_select_wav(frame: &mut Frame<'_>, app: &mut App, chunks: &[Rect]) {
    let chunks_sel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)])
        .split(chunks[1]);

    let desc = Paragraph::new("2. Select the WAV file of singing recording which will be graded.")
        .bold()
        .centered()
        .blue()
        .block(Block::new().padding(Padding::new(0, 0, chunks_sel[0].height / 3, 0)));

    frame.render_widget(desc, chunks_sel[0]);

    let list_block = Block::new()
        .title(Line::raw("WAV File List").centered())
        .border_style(Style::new().blue())
        .borders(Borders::ALL);

    let items: Vec<ListItem> = app
        .wav_path_list
        .items
        .iter()
        .map(|fp| ListItem::from(Text::from(fp.path().to_str().expect("msg"))))
        .collect();

    let list = List::new(items)
        .block(list_block)
        .highlight_style(Style::new().black().on_blue().bold())
        .highlight_spacing(HighlightSpacing::WhenSelected);

    frame.render_stateful_widget(list, chunks_sel[1], &mut app.wav_path_list.state);
}

fn render_confirm_select(frame: &mut Frame<'_>, app: &App, chunks: &[Rect]) {
    let (Some(midi_path_idx), Some(wav_path_idx)) = (
        app.midi_path_list.state.selected(),
        app.wav_path_list.state.selected(),
    ) else {
        // SAFETY: the previous two screens cannot be proceeded unless a file is selected
        // so this will never happen
        unsafe { unreachable_unchecked() }
    };
    let chunks_confirm = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(chunks[1]);

    let chunks_confirm_mid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(chunks_confirm[1]);

    let sel_midi_file_block = Block::new()
        .title(Line::raw("Selected MIDI File").centered())
        .border_style(Style::new().green())
        .borders(Borders::ALL)
        .padding(Padding::new(0, 0, chunks_confirm_mid[0].height / 3, 0));
    let sel_midi_file_para = Paragraph::new(
        app.midi_path_list.items[midi_path_idx]
            .path()
            .to_str()
            .expect("msg"),
    )
    .centered()
    .bold()
    .block(sel_midi_file_block);
    frame.render_widget(sel_midi_file_para, chunks_confirm_mid[0]);

    let sel_versus_block = Block::new()
        .border_style(Style::new().black())
        .borders(Borders::ALL)
        .padding(Padding::new(0, 0, chunks_confirm_mid[1].height / 3, 0));
    let versus = Paragraph::new("Versus")
        .centered()
        .bold()
        .block(sel_versus_block);
    frame.render_widget(versus, chunks_confirm_mid[1]);

    let sel_wav_file_block = Block::new()
        .title(Line::raw("Selected WAV File").centered())
        .border_style(Style::new().blue())
        .borders(Borders::ALL)
        .padding(Padding::new(0, 0, chunks_confirm_mid[2].height / 3, 0));
    let sel_wav_file_para = Paragraph::new(
        app.wav_path_list.items[wav_path_idx]
            .path()
            .to_str()
            .expect("msg"),
    )
    .centered()
    .bold()
    .block(sel_wav_file_block);
    frame.render_widget(sel_wav_file_para, chunks_confirm_mid[2]);

    let desc = Paragraph::new("3. Confirm that these are the files you meant to choose.")
        .bold()
        .centered()
        .yellow()
        .block(Block::new().padding(Padding::new(0, 0, chunks_confirm[0].height / 3, 0)));
    frame.render_widget(desc, chunks_confirm[0]);

    let desc_2 = Paragraph::new("Analysis will take no more than a few seconds.")
        .bold()
        .centered()
        .white()
        .block(Block::new().padding(Padding::new(0, 0, chunks_confirm[2].height / 2, 0)));
    frame.render_widget(desc_2, chunks_confirm[2]);
}
