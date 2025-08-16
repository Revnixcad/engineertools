pub fn get_local_storage() -> Option<web_sys::Storage> {
    leptos::prelude::window().local_storage().ok().flatten()
}

pub fn get_local_storage_item(key: &str) -> Option<String> {
    if let Some(storage) = get_local_storage() {
        storage.get_item(key).ok().flatten()
    } else {
        None
    }
}

pub fn set_local_storage_item(key: &str, value: &str) -> Result<(), web_sys::ErrorEvent> {
    if let Some(storage) = get_local_storage() {
        Ok(storage.set_item(key, value)?)
    } else {
        Err(web_sys::ErrorEvent::new("Local storage not available").unwrap())
    }
}
