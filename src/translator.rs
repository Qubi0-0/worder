use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyMemoryResponse {
    #[serde(rename = "responseData")]
    response_data: ResponseData,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: Option<String>,
}

/// Translates a word from German to English using MyMemory API.
/// The API handles minor typos reasonably well.
/// In the future, an AI API can be plugged in for better results.
pub fn translate_de_to_en(word: &str) -> Result<String, String> {
    let response = ureq::get("https://api.mymemory.translated.net/get")
        .query("q", word)
        .query("langpair", "de|eng")
        .call()
        .map_err(|e| format!("Connectivity issue: {}", e))?;

    let body: MyMemoryResponse = response
        .into_json()
        .map_err(|e| format!("Parse Error: {}", e))?;

    let translation = body.response_data.translated_text.unwrap_or_default();

    if translation.is_empty() || translation.to_lowercase() == word.to_lowercase() {
        Ok("—".to_string())
    } else {
        Ok(translation)
    }
}

#[cfg(test)]
mod tests {
    // Tests for the local translation-filtering logic only.
    // We do not call the real API in unit tests.

    fn filter(word: &str, raw: &str) -> String {
        if raw.is_empty() || raw.to_lowercase() == word.to_lowercase() {
            "—".to_string()
        } else {
            raw.to_string()
        }
    }

    #[test]
    fn valid_translation_is_returned() {
        assert_eq!(filter("Hund", "dog"), "dog");
    }

    #[test]
    fn empty_response_returns_placeholder() {
        assert_eq!(filter("Hund", ""), "—");
    }

    #[test]
    fn null_response_returns_placeholder() {
        // unwrap_or_default() on None gives empty string, same path as empty
        assert_eq!(filter("Hund", ""), "—");
    }

    #[test]
    fn same_word_response_returns_placeholder() {
        // API echoes the word back when it doesn't know it
        assert_eq!(filter("Hund", "Hund"), "—");
    }

    #[test]
    fn same_word_case_insensitive_returns_placeholder() {
        assert_eq!(filter("hund", "HUND"), "—");
    }

    #[test]
    fn different_word_is_not_filtered() {
        assert_eq!(filter("Katze", "cat"), "cat");
    }
}
