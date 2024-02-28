// this file contains utility code that doesn't neatly fit into any of the other categories
use crate::PasteCategory;

impl TryFrom<String> for PasteCategory {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "markdown" => PasteCategory::Markdown,
            "plaintext" => PasteCategory::Plaintext,
            "html" => PasteCategory::Html,
            "file" => PasteCategory::File,
            "url" => PasteCategory::Url,
            _ => {
                return Err(format!(
                    "Unknown category encountered when deserializing category: {:?}",
                    value
                ))
            }
        })
    }
}

impl Into<String> for PasteCategory {
    fn into(self) -> String {
        String::from(match self {
            PasteCategory::Markdown => "markdown",
            PasteCategory::Plaintext => "plaintext",
            PasteCategory::Html => "html",
            PasteCategory::File => "file",
            PasteCategory::Url => "url",
        })
    }
}