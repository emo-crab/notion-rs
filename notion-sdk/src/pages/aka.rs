use crate::database::properties::PropertyValue;
use crate::pages::Page;

impl Page {
    pub fn get_title(&self) -> String {
        if let Some(PropertyValue::Title { title, .. }) = self.properties.properties.get("Title") {
            let tit: Vec<String> = title.iter().map(|r| r.plain_text().to_string()).collect();
            return tit.join("");
        }
        String::new()
    }
}
