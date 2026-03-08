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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn empty_dict() -> Dictionary {
        Dictionary {
            entries: Vec::new(),
            file_path: PathBuf::from(""),
        }
    }

    #[test]
    fn add_new_entry() {
        let mut dict = empty_dict();
        dict.add_entry("Hund".into(), "dog".into());
        assert_eq!(dict.entries.len(), 1);
        assert_eq!(dict.entries[0].german, "Hund");
        assert_eq!(dict.entries[0].translation, "dog");
    }

    #[test]
    fn add_duplicate_overwrites_translation() {
        let mut dict = empty_dict();
        dict.add_entry("Hund".into(), "dog".into());
        dict.add_entry("hund".into(), "hound".into()); // same word, different case
        assert_eq!(dict.entries.len(), 1);
        assert_eq!(dict.entries[0].translation, "hound");
    }

    #[test]
    fn remove_entry_valid_index() {
        let mut dict = empty_dict();
        dict.add_entry("Hund".into(), "dog".into());
        dict.add_entry("Katze".into(), "cat".into());
        dict.remove_entry(0);
        assert_eq!(dict.entries.len(), 1);
        assert_eq!(dict.entries[0].german, "Katze");
    }

    #[test]
    fn remove_entry_out_of_bounds_does_nothing() {
        let mut dict = empty_dict();
        dict.add_entry("Hund".into(), "dog".into());
        dict.remove_entry(99);
        assert_eq!(dict.entries.len(), 1);
    }

    #[test]
    fn remove_all_entries() {
        let mut dict = empty_dict();
        dict.add_entry("Hund".into(), "dog".into());
        dict.remove_entry(0);
        assert!(dict.entries.is_empty());
    }

    #[test]
    fn load_nonexistent_file_returns_empty() {
        let dict = Dictionary::load(PathBuf::from("/tmp/nonexistent_worder_test.json"));
        assert!(dict.entries.is_empty());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let path = PathBuf::from("/tmp/worder_test_roundtrip.json");
        let mut dict = Dictionary {
            entries: Vec::new(),
            file_path: path.clone(),
        };
        dict.add_entry("Wasser".into(), "water".into());
        dict.save().expect("save failed");

        let loaded = Dictionary::load(path.clone());
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.entries[0].german, "Wasser");
        assert_eq!(loaded.entries[0].translation, "water");

        std::fs::remove_file(path).ok();
    }
}
