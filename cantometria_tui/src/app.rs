use ratatui::widgets::ListState;
use walkdir::{DirEntry, WalkDir};

pub enum CurrentScreen {
    Main,
    SelectingMidi,
    SelectingWav,
    ConfirmingSelection,
    Grading,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub midi_path_list: PathList,
    pub wav_path_list: PathList,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Main,
            midi_path_list: WalkDir::new("./midi")
                .follow_links(true)
                .into_iter()
                .filter_map(|x| {
                    x.ok().filter(|y| {
                        y.file_type().is_file() && y.path().extension().is_some_and(|z| z == "mid")
                    })
                })
                .collect(),
            wav_path_list: WalkDir::new("./test")
                .follow_links(true)
                .into_iter()
                .filter_map(|x| {
                    x.ok().filter(|y| {
                        y.file_type().is_file() && y.path().extension().is_some_and(|z| z == "wav")
                    })
                })
                .collect(),
        }
    }
}

pub struct PathList {
    pub items: Box<[DirEntry]>,
    pub state: ListState,
}

impl FromIterator<DirEntry> for PathList {
    fn from_iter<T: IntoIterator<Item = DirEntry>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
            state: ListState::default(),
        }
    }
}
