use bevy_tasks::Task;

/// TODO take file and ask about reload.
pub struct FileHandler {
    file: FileState,
}

#[derive(Default)]
pub enum FileState {
    #[default]
    None,
    Task(Task<Option<Vec<u8>>>),
    File(Vec<u8>),
}

impl FileHandler {
    pub fn new() -> Self {
        Self {
            file: FileState::None,
        }
    }

    pub fn load(&mut self) {
        let file_future = async {
            let file = rfd::AsyncFileDialog::new()
                .add_filter("ROMs", &["gb", "bin"])
                .set_title("Choose ROM")
                .pick_file()
                .await;

            match file {
                Some(file) => Some(file.read().await),
                None => None,
            }
        };

        let task =  bevy_tasks::IoTaskPool::get().spawn(file_future);

        self.file = FileState::Task(task);
    }

    pub fn alive(&self) -> bool {
        matches!(self.file, FileState::Task(..))
    }

    pub fn tick(&mut self) -> bool {
        match &mut self.file {
            FileState::None => (),
            FileState::Task(task) => {
                if let Some(task) = futures_lite::future::block_on(futures_lite::future::poll_once(task)) {
                    self.file = match task {
                        Some(file) => FileState::File(file),
                        None => FileState::None,
                    };
                }
            }
            FileState::File(..) => (),
        }
        matches!(self.file, FileState::File(..))
    }

    pub fn take(&mut self) -> Vec<u8> {
        if let FileState::File(file) = std::mem::take(&mut self.file) {
            file
        } else {
            panic!();
        }
    }
}
