mod file;

use std::path::PathBuf;

use iced::widget::{column, container, pick_list, row, text, text_editor};
use iced::{Element, Font, Length, Theme};
use rfd::FileDialog;

const CUSTOM_FONT: Font = Font::with_name("CaskaydiaCove Nerd Font Mono");

#[derive(Default)]
struct State {
    file_path: Option<PathBuf>,
    content: text_editor::Content,
    selected_file_action: Option<FileAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileAction {
    Save,
    SaveAs,
    Open,
}

impl FileAction {
    const ALL: &'static [FileAction] = &[FileAction::SaveAs, FileAction::Open, FileAction::Save];
}

impl std::fmt::Display for FileAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileAction::Save => write!(f, "Save"),
            FileAction::SaveAs => write!(f, "Save As... "),
            FileAction::Open => write!(f, "Open"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileActionSelected(FileAction),
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}

fn view(state: &State) -> Element<'_, Message> {
    let file_menu = pick_list(
        FileAction::ALL,
        state.selected_file_action,
        Message::FileActionSelected,
    )
    .placeholder("File");

    let action_bar = container(row![file_menu]);

    let editor = text_editor(&state.content)
        .height(Length::Fill)
        .on_action(Message::Edit);

    let editor_container = container(row![editor]).height(Length::Fill);

    let cursor_position = state.content.cursor().position;
    let cursor_display_text = format!(
        "Ln {}, Col {}",
        cursor_position.line, cursor_position.column
    );
    let cursor_text = text(cursor_display_text);

    let file_path_display_text = match &state.file_path {
        Some(path) => path.to_string_lossy().to_string(),
        None => String::new(),
    };

    let file_path_text = text(file_path_display_text);

    let status_bar = container(row![file_path_text, cursor_text].spacing(10));

    container(column![action_bar, editor_container, status_bar].spacing(10))
        .padding(10)
        .into()
}

fn save_file(path: Option<PathBuf>, text: String) -> Option<PathBuf> {
    let mut save_path = path.clone();

    if path == None {
        save_path = FileDialog::new().set_directory("/").save_file();
    }

    match save_path {
        Some(p) => {
            let sp = p.clone();
            file::save_file_to_disk(sp, text);
            Some(p)
        }
        None => None,
    }
}

fn save_file_as(text: String) {
    let files = FileDialog::new().set_directory("/").save_file();

    match files {
        Some(path) => file::save_file_to_disk(path, text),
        None => {
            println!("No file to save to");
        }
    }
}

fn open_file() -> (PathBuf, String) {
    let file = FileDialog::new().set_directory("/").pick_file();

    match file {
        Some(path) => {
            let content = file::load_file_from_disk(path.clone());
            (path, content)
        }
        None => (PathBuf::new(), String::new()),
    }
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
        }

        Message::FileActionSelected(action) => {
            state.selected_file_action = None;

            match action {
                FileAction::SaveAs => save_file_as(state.content.text()),
                FileAction::Open => {
                    let (path, content) = open_file();
                    state.content = text_editor::Content::with_text(&content);
                    state.file_path = Some(path);
                }
                FileAction::Save => {
                    state.file_path = save_file(state.file_path.clone(), state.content.text());
                }
            }
        }
    }
}

fn boot() -> State {
    State::default()
}

pub fn main() -> iced::Result {
    iced::application(boot, update, view)
        .font(include_bytes!("./fonts/CaskaydiaCoveNFM-Regular.ttf"))
        .theme(theme)
        .settings(iced::Settings {
            default_font: CUSTOM_FONT,
            ..Default::default()
        })
        .run()
}
