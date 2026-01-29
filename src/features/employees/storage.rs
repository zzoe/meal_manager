use super::model::Employee;
use crate::infra::DatabaseConnection;
use redb::{ReadableDatabase, ReadableTable, TableDefinition};

const TABLE_EMPLOYEES: TableDefinition<&str, &str> = TableDefinition::new("employees");

pub struct EmployeeStorage;

impl EmployeeStorage {
    fn init_table() {
        let db = DatabaseConnection::get();
        let write_txn = db.begin_write().unwrap();
        {
            let _ = write_txn.open_table(TABLE_EMPLOYEES).unwrap();
        }
        write_txn.commit().unwrap();
    }

    pub fn save_employees(employees: Vec<Employee>) -> anyhow::Result<()> {
        Self::init_table();
        let db = DatabaseConnection::get();
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE_EMPLOYEES)?;
            
            // 先清空所有旧数据
            let mut keys_to_delete = Vec::new();
            for item in table.iter()? {
                let (k, _) = item?;
                keys_to_delete.push(k.value().to_string());
            }
            for k in keys_to_delete {
                table.remove(k.as_str())?;
            }

            // 存入新数据
            for emp in employees {
                let json = serde_json::to_string(&emp)?;
                table.insert(emp.name.as_str(), json.as_str())?;
            }
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn add_employee(emp: Employee) -> anyhow::Result<()> {
        Self::init_table();
        let db = DatabaseConnection::get();
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE_EMPLOYEES)?;
            let json = serde_json::to_string(&emp)?;
            table.insert(emp.name.as_str(), json.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn update_employee(old_name: &str, new_emp: Employee) -> anyhow::Result<()> {
        Self::init_table();
        let db = DatabaseConnection::get();
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE_EMPLOYEES)?;
            // 如果姓名变了，需要先删除旧键
            if old_name != new_emp.name {
                table.remove(old_name)?;
            }
            let json = serde_json::to_string(&new_emp)?;
            table.insert(new_emp.name.as_str(), json.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn delete_employee(name: &str) -> anyhow::Result<()> {
        Self::init_table();
        let db = DatabaseConnection::get();
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE_EMPLOYEES)?;
            table.remove(name)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn load_employees() -> Vec<Employee> {
        Self::init_table();
        let db = DatabaseConnection::get();
        let read_txn = db.begin_read().unwrap();
        let table = match read_txn.open_table(TABLE_EMPLOYEES) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };

        let mut list = Vec::new();
        if let Ok(iter) = table.iter() {
            for item in iter {
                if let Ok((_, v)) = item {
                    if let Ok(emp) = serde_json::from_str::<Employee>(v.value()) {
                        list.push(emp);
                    }
                }
            }
        }
        println!("EmployeeStorage: loaded {} employees", list.len());
        list
    }
}
