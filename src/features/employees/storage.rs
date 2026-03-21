use super::model::Employee;

#[cfg(not(target_arch = "wasm32"))]
use crate::infra::DatabaseConnection;
#[cfg(not(target_arch = "wasm32"))]
use redb::{ReadableDatabase, ReadableTable, TableDefinition};

#[cfg(target_arch = "wasm32")]
use anyhow::anyhow;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(not(target_arch = "wasm32"))]
const TABLE_EMPLOYEES: TableDefinition<&str, &str> = TableDefinition::new("employees");

#[cfg(target_arch = "wasm32")]
const STORAGE_KEY: &str = "meal_manager:employees";

pub struct EmployeeStorage;

#[cfg(not(target_arch = "wasm32"))]
impl EmployeeStorage {
    fn init_table() {
        let db = DatabaseConnection::get();
        let write_txn = db.begin_write().unwrap();
        {
            let _ = write_txn.open_table(TABLE_EMPLOYEES).unwrap();
        }
        write_txn.commit().unwrap();
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
            for (_, v) in iter.flatten() {
                if let Ok(emp) = serde_json::from_str::<Employee>(v.value()) {
                    list.push(emp);
                }
            }
        }
        list
    }
}

#[cfg(target_arch = "wasm32")]
impl EmployeeStorage {
    fn storage() -> anyhow::Result<web_sys::Storage> {
        let window = window().ok_or_else(|| anyhow!("window is not available"))?;
        let storage = window
            .local_storage()
            .map_err(|err| anyhow!("{err:?}"))?
            .ok_or_else(|| anyhow!("localStorage is not available"))?;
        Ok(storage)
    }

    fn load_all() -> Vec<Employee> {
        let storage = match Self::storage() {
            Ok(storage) => storage,
            Err(_) => return Vec::new(),
        };
        match storage.get_item(STORAGE_KEY) {
            Ok(Some(json)) => serde_json::from_str::<Vec<Employee>>(&json).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    fn save_all(employees: &[Employee]) -> anyhow::Result<()> {
        let storage = Self::storage()?;
        let json = serde_json::to_string(employees)?;
        storage
            .set_item(STORAGE_KEY, &json)
            .map_err(|err| anyhow!("{err:?}"))?;
        Ok(())
    }

    pub fn add_employee(emp: Employee) -> anyhow::Result<()> {
        let mut employees = Self::load_all();
        employees.retain(|item| item.name != emp.name);
        employees.push(emp);
        Self::save_all(&employees)
    }

    pub fn update_employee(old_name: &str, new_emp: Employee) -> anyhow::Result<()> {
        let mut employees = Self::load_all();
        employees.retain(|item| item.name != old_name);
        employees.retain(|item| item.name != new_emp.name);
        employees.push(new_emp);
        Self::save_all(&employees)
    }

    pub fn delete_employee(name: &str) -> anyhow::Result<()> {
        let mut employees = Self::load_all();
        employees.retain(|item| item.name != name);
        Self::save_all(&employees)
    }

    pub fn load_employees() -> Vec<Employee> {
        Self::load_all()
    }
}
