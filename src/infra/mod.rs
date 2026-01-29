use redb::Database;
use std::sync::OnceLock;

static DB_INSTANCE: OnceLock<Database> = OnceLock::new();

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn get() -> &'static Database {
        DB_INSTANCE.get_or_init(|| {
            let db = Database::create("meal_manager.redb").expect("Failed to create DB");
            // Tables should be opened by the modules that own them during initialization or on-demand
            db
        })
    }
}
