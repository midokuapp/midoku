use midoku_store::Store;

pub fn app_data() -> Store {
    Store::open("app_data")
}
