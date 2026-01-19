use crate::domain::Employee;
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::sync::OnceLock;

const TABLE_EMPLOYEES: TableDefinition<&str, &str> = TableDefinition::new("employees");
static DB_INSTANCE: OnceLock<Database> = OnceLock::new();

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn get() -> &'static Database {
        DB_INSTANCE.get_or_init(|| {
            let db = Database::create("meal_manager.redb").expect("Failed to create DB");
            let write_txn = db.begin_write().unwrap();
            {
                let _ = write_txn.open_table(TABLE_EMPLOYEES).unwrap();
            }
            write_txn.commit().unwrap();
            db
        })
    }

    pub fn save_employees(employees: Vec<Employee>) -> anyhow::Result<()> {
        let db = Self::get();
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE_EMPLOYEES)?;
            for emp in employees {
                let json = serde_json::to_string(&emp)?;
                table.insert(emp.name.as_str(), json.as_str())?;
            }
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn load_employees() -> Vec<Employee> {
        let db = Self::get();
        let read_txn = db.begin_read().unwrap();
        let table = read_txn.open_table(TABLE_EMPLOYEES).unwrap();

        let mut list = Vec::new();
        for item in table.iter().unwrap() {
            if let Ok((_, v)) = item {
                if let Ok(emp) = serde_json::from_str::<Employee>(v.value()) {
                    list.push(emp);
                }
            }
        }
        list
    }
}

pub fn get_database() -> &'static Database {
    DatabaseConnection::get()
}
