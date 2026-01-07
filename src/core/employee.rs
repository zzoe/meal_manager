//! 员工管理模块 - 处理员工数据和昵称管理

use serde::{Deserialize, Serialize};
use redb::{ReadableDatabase, ReadableTable, TableDefinition};
use anyhow::{Context, Result};

use super::db;

const TABLE_EMPLOYEES: TableDefinition<&str, &str> = TableDefinition::new("employees");

/// 员工数据
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmployeeData {
    pub name: String,
    pub nicknames: String,
}

impl EmployeeData {
    /// 创建新员工
    pub fn new(name: String, nicknames: String) -> Self {
        Self { name, nicknames }
    }
    
    /// 获取所有昵称列表
    pub fn get_nicknames(&self) -> Vec<&str> {
        self.nicknames.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
    }
    
    /// 添加昵称
    pub fn add_nickname(&mut self, nickname: &str) {
        let nickname = nickname.trim();
        if !nickname.is_empty() {
            if self.nicknames.is_empty() {
                self.nicknames = nickname.to_string();
            } else {
                self.nicknames.push_str(",");
                self.nicknames.push_str(nickname);
            }
        }
    }
}

/// 员工管理器 - 包含数据库操作
pub struct EmployeeManager;

impl EmployeeManager {
    /// 加载所有员工数据
    pub fn load_all() -> Result<Vec<EmployeeData>> {
        let db = db::init_db();
        let read_txn = db
            .begin_read()
            .context("开始读事务失败")?;
        
        let table = read_txn
            .open_table(TABLE_EMPLOYEES)
            .context("打开表失败")?;
        
        let mut list = Vec::new();
        
        for result in table.iter().context("迭代表失败")? {
            let (_key, value) = result.context("读取记录失败")?;
            
            let emp: EmployeeData = serde_json::from_str(value.value())?;
            list.push(emp);
        }
        
        Ok(list)
    }

    /// 保存员工数据
    pub fn save(emp: &EmployeeData) -> Result<()> {
        let db = db::init_db();
        let write_txn = db
            .begin_write()
            .context("开始写事务失败")?;
        
        {
            let mut table = write_txn
                .open_table(TABLE_EMPLOYEES)
                .context("打开表失败")?;
            
            let json = serde_json::to_string(emp)?;
            
            table
                .insert(emp.name.as_str(), json.as_str())
                .context("插入记录失败")?;
        }
        
        write_txn.commit().context("提交事务失败")?;
        Ok(())
    }
}