use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub german: String,
    pub translation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
    pub entries: Vec<DictionaryEntry>,
    #[serde(skip)]
    file_path: PathBuf,
}

impl Dictionary {
    pub fn load(path: PathBuf) -> Self {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(mut dict) = serde_json::from_str::<Dictionary>(&data) {
                    dict.file_path = path;
                    return dict;
                }
            }
        }
        Dictionary {
            entries: Vec::new(),
            file_path: path,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(&self.file_path, data)?;
        Ok(())
    }

    pub fn add_entry(&mut self, german: String, translation: String) {
        // If the word already exists, overwrite the translation
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|e| e.german.to_lowercase() == german.to_lowercase())
        {
            entry.translation = translation;
        } else {
            self.entries.push(DictionaryEntry {
                german,
                translation,
            });
        }
    }

    pub fn remove_entry(&mut self, index: usize) {
        if index < self.entries.len() {
            self.entries.remove(index);
        }
    }
}
