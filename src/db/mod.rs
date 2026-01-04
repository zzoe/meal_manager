use redb::{Database, TableDefinition, ReadableTable};
use crate::app::state::EmployeeData;
use serde_json;

const TABLE_EMPLOYEES: TableDefinition<&str, &str> = TableDefinition::new("employees");

pub fn init_db() -> Database {
    let db = Database::create("meal_data.redb").unwrap();
    let write_txn = db.begin_write().unwrap();
    {
        let _ = write_txn.open_table(TABLE_EMPLOYEES).unwrap();
    }
    write_txn.commit().unwrap();
    db
}

pub fn load_all_employees(db: &Database) -> Vec<EmployeeData> {
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(TABLE_EMPLOYEES).unwrap();
    let mut list = Vec::new();
    for result in table.iter().unwrap() {
        let (_key, value) = result.unwrap();
        let emp: EmployeeData = serde_json::from_str::<EmployeeData>(value.value()).unwrap();
        list.push(emp);
    }
    list
}

pub fn save_employee(db: &Database, emp: &EmployeeData) {
    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(TABLE_EMPLOYEES).unwrap();
        let val = serde_json::to_string(emp).unwrap();
        table.insert(emp.name.as_str(), val.as_str()).unwrap();
    }
    write_txn.commit().unwrap();
}

pub fn delete_employee(db: &Database, name: &str) {
    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(TABLE_EMPLOYEES).unwrap();
        table.remove(name).unwrap();
    }
    write_txn.commit().unwrap();
}
