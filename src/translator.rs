use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyMemoryResponse {
    #[serde(rename = "responseData")]
    response_data: ResponseData,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
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

    let translation = body.response_data.translated_text;

    if translation.is_empty() || translation.to_lowercase() == word.to_lowercase() {
        Err(format!("No translation found for '{}'", word))
    } else {
        Ok(translation)
    }
}
