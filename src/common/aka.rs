use crate::common::file::{ExternalFileObject, FileObject, FileOrEmojiObject};
use reqwest::Url;

impl FileOrEmojiObject {
    pub fn emoji_from_shortcode(name: &str) -> Self {
        let default = FileOrEmojiObject::Emoji {
            emoji: String::new(),
        };
        if let Some(e) = emojis::get_by_shortcode(name) {
            return FileOrEmojiObject::Emoji {
                emoji: e.to_string(),
            };
        }
        default
    }
    pub fn external_file_from_url(url: Url) -> Self {
        FileOrEmojiObject::External {
            external: ExternalFileObject {
                url: url.to_string(),
            },
        }
    }
}

impl FileObject {
    pub fn external_file_from_url(url: Url) -> Self {
        FileObject::External {
            external: ExternalFileObject {
                url: url.to_string(),
            },
        }
    }
}
