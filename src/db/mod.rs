//! 数据库操作模块
//! 
//! 提供对本地 redb 数据库的 CRUD 操作，封装所有数据库相关错误

use crate::app::state::EmployeeData;
use crate::error::{AppError, Result};
use anyhow::Context;
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde_json;

const TABLE_EMPLOYEES: TableDefinition<&str, &str> = TableDefinition::new("employees");

/// 初始化数据库，如果不存在则创建
/// 如果初始化失败，程序将 panic
pub fn init_db() -> Database {
    Database::create("meal_data.redb")
        .unwrap_or_else(|e| {
            eprintln!("数据库初始化失败: {}", e);
            panic!("数据库初始化失败: {}", e);
        })
}

/// 加载所有员工数据
pub fn load_all_employees(db: &Database) -> Result<Vec<EmployeeData>> {
    let read_txn = db
        .begin_read()
        .context("开始读事务失败")?;
    
    let table = read_txn
        .open_table(TABLE_EMPLOYEES)
        .context("打开表失败")?;
    
    let mut list = Vec::new();
    
    for result in table.iter().context("迭代表失败")? {
        let (_key, value) = result.context("读取记录失败")?;
        
        let emp: EmployeeData = serde_json::from_str::<EmployeeData>(value.value())
            .map_err(|e| AppError::Serialization(e))?;
        
        list.push(emp);
    }
    
    Ok(list)
}

/// 保存员工数据
pub fn save_employee(db: &Database, emp: &EmployeeData) -> Result<()> {
    let write_txn = db
        .begin_write()
        .context("开始写事务失败")?;
    
    {
        let mut table = write_txn
            .open_table(TABLE_EMPLOYEES)
            .context("打开表失败")?;
        
        let val = serde_json::to_string(emp)
            .map_err(|e| AppError::Serialization(e))?;
        
        table
            .insert(emp.name.as_str(), val.as_str())
            .context("插入记录失败")?;
    }
    
    write_txn
        .commit()
        .context("提交事务失败")?;
    
    Ok(())
}

/// 删除指定员工
pub fn delete_employee(db: &Database, name: &str) -> Result<()> {
    let write_txn = db
        .begin_write()
        .context("开始写事务失败")?;
    
    {
        let mut table = write_txn
            .open_table(TABLE_EMPLOYEES)
            .context("打开表失败")?;
        
        table
            .remove(name)
            .context("删除记录失败")?;
    }
    
    write_txn
        .commit()
        .context("提交事务失败")?;
    
    Ok(())
}
