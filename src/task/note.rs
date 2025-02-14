#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-support", serde(untagged))]
pub enum Note {
    None,
    Short(String),
    Long { filename: String, content: String },
}

impl Note {
    pub fn from_file(filename: &str) -> Self {
        use std::io::Read;

        if filename.is_empty() {
            return Note::None;
        }

        let note_file = match Self::note_file(filename) {
            Ok(note_file) => note_file,
            Err(err) => {
                error!("{}", err);
                return Note::Short(filename.to_string());
            }
        };

        let file = match std::fs::File::open(note_file.clone()) {
            Ok(file) => file,
            Err(_) => {
                error!("Unable to open {:?}", note_file);
                return Note::Short(filename.to_string());
            }
        };

        let mut buffer = std::io::BufReader::new(file);
        let mut content = String::new();

        match buffer.read_to_string(&mut content) {
            Ok(_) => (),
            Err(_) => {
                error!("Unable to read {:?}", note_file);
                return Note::Short(filename.to_string());
            }
        };

        Note::Long {
            filename: filename.to_string(),
            content,
        }
    }

    pub fn content(&self) -> Option<String> {
        match *self {
            Note::None => None,
            Note::Short(ref content) | Note::Long { ref content, .. } => Some(content.clone()),
        }
    }
    pub fn write(&self) -> Result<Self, String> {
        let mut note = self.clone();

        if self == &Note::None {
            return Ok(note);
        }

        if let Note::Short(ref content) = *self {
            note = Note::Long {
                filename: Self::new_filename(),
                content: content.clone(),
            }
        }

        if let Note::Long {
            ref filename,
            ref content,
        } = note
        {
            if content.is_empty() {
                match std::fs::remove_file(Self::note_file(filename)?) {
                    Ok(_) => (),
                    Err(err) => error!("Unable to delete note: {}", err),
                };

                return Ok(Note::None);
            }
        }

        if let Note::Long {
            ref filename,
            ref content,
        } = note
        {
            use std::io::Write;

            let note_file = Self::note_file(filename)?;

            let mut f = match std::fs::File::create(note_file) {
                Ok(f) => f,
                Err(err) => return Err(format!("{}", err)),
            };

            match f.write(content.as_bytes()) {
                Ok(_) => (),
                Err(err) => return Err(format!("{}", err)),
            };
        }

        Ok(note)
    }

    fn new_filename() -> String {
        let ext = match std::env::var("TODO_NOTE_EXT") {
            Ok(ext) => ext,
            Err(_) => ".txt".to_owned(),
        };

        let name = Self::new_note_id();

        format!("{}{}", name, ext)
    }

    fn new_note_id() -> String {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(3)
            .collect()
    }

    fn note_file(filename: &str) -> Result<String, String> {
        let todo_dir = match std::env::var("TODO_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(_) => return Err("Launch this program via todo.sh".to_owned()),
        };

        let note_dir = match std::env::var("TODO_NOTES_DIR") {
            Ok(note_dir) => note_dir,
            Err(_) => format!("{}/notes", todo_dir),
        };

        Ok(format!("{}/{}", note_dir, filename))
    }
}

impl Default for Note {
    fn default() -> Self {
        Note::None
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tag = match std::env::var("TODO_NOTE_TAG") {
            Ok(tag) => tag,
            Err(_) => "note".to_owned(),
        };

        let tag = match *self {
            Note::None => String::new(),
            Note::Short(ref content) => format!("{}:{}", tag, content),
            Note::Long { ref filename, .. } => format!("{}:{}", tag, filename),
        };

        f.write_str(tag.as_str())
    }
}
