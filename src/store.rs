use midoku_store::{use_store, UseStore};

pub fn use_app_data() -> UseStore {
    use_store("app_data")
}
