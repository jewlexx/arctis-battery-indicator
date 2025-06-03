use std::{cell::LazyCell, collections::HashMap, rc::Rc, sync::Mutex};

use include_dir::include_dir;

thread_local! {
    pub static ICONS: LazyCell<Icons> = LazyCell::new(Icons::new);
}

#[derive(Debug)]
pub struct Icons {
    icons: &'static include_dir::Dir<'static>,
    parsed_images: Mutex<HashMap<u16, Rc<image::RgbaImage>>>,
}

impl Default for Icons {
    fn default() -> Self {
        Self::new()
    }
}

impl Icons {
    pub fn new() -> Self {
        pub static ICONS: include_dir::Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/bat");

        Icons {
            icons: &ICONS,
            parsed_images: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_icon_path(&self, res_id: u16) -> String {
        format!("battery{res_id}.ico")
    }

    pub fn get_icon(&self, res_id: u16) -> Option<&'static include_dir::File<'static>> {
        self.icons.get_file(self.get_icon_path(res_id))
    }

    pub fn get_icon_image(&self, res_id: u16) -> Option<Rc<image::RgbaImage>> {
        self.get_icon(res_id).map(|file| {
            self.parsed_images
                .lock()
                .unwrap()
                .entry(res_id)
                .or_insert_with(|| {
                    Rc::new(
                        image::load_from_memory(file.contents())
                            .ok()
                            .map(|img| img.to_rgba8())
                            .unwrap(),
                    )
                })
                .clone()
        })
    }
}
