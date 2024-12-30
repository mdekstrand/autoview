//! Access to the MIME database.
use std::cell::OnceCell;
use std::rc::Rc;
use std::thread_local;

use log::*;
#[cfg(not(feature = "xdg-embedded"))]
use shared_mime::load_mime_db;
use shared_mime::MimeDB;
#[cfg(feature = "xdg-embedded")]
use shared_mime_embedded::load_mime_db;

thread_local! {
    static MIME_DB: OnceCell<Rc<MimeDB>> = OnceCell::new();
}

/// Load or retrieve the MIME database.
pub fn mime_db() -> Rc<MimeDB> {
    MIME_DB.with(|c| {
        let dbref = c.get_or_init(|| {
            info!("loading MIME database");
            match load_mime_db() {
                Ok(db) => Rc::new(db),
                Err(e) => {
                    error!("error loading MIME database: {}", e);
                    std::process::exit(5)
                }
            }
        });
        dbref.clone()
    })
}
