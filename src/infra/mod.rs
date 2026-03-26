use std::sync::OnceLock;

use redb::Database;

static DB: OnceLock<Database> = OnceLock::new();

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn get() -> &'static Database {
        DB.get_or_init(|| {
            // 使用项目目录或临时目录存储数据
            let path = if let Some(home) = std::env::var_os("HOME") {
                std::path::PathBuf::from(home).join(".meal_manager")
            } else if let Some(userprofile) = std::env::var_os("USERPROFILE") {
                std::path::PathBuf::from(userprofile).join(".meal_manager")
            } else {
                std::env::temp_dir().join("meal_manager")
            };
            std::fs::create_dir_all(&path).unwrap();
            Database::create(path.join("data.redb")).unwrap()
        })
    }
}
