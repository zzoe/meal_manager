//! 数据库公共模块 - 包含数据库初始化和配置

use redb::Database;
use anyhow::{Context, Result};

/// 数据库配置常量
pub const DB_PATH: &str = "meal_data.redb";

/// 初始化数据库，如果不存在则创建
pub fn init_db() -> Database {
    Database::create(DB_PATH)
        .unwrap_or_else(|e| {
            eprintln!("数据库初始化失败: {}", e);
            panic!("数据库初始化失败: {}", e);
        })
}

/// 数据库管理工具
pub struct DbManager;

impl DbManager {
    /// 检查数据库是否存在
    pub fn exists() -> bool {
        std::path::Path::new(DB_PATH).exists()
    }
    
    /// 删除数据库文件（用于测试或重置）
    pub fn delete() -> Result<()> {
        if Self::exists() {
            std::fs::remove_file(DB_PATH).context("删除数据库文件失败")?;
        }
        Ok(())
    }
}