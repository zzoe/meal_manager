use redb::{Database, TableDefinition, ReadableTable, ReadableDatabase};
use crate::backend::domain::Employee;
use std::sync::OnceLock;

const TABLE_EMPLOYEES: TableDefinition<&str, &str> = TableDefinition::new("employees");
static DB_INSTANCE: OnceLock<Database> = OnceLock::new();

pub fn get_db() -> &'static Database {
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

pub fn save_employees_to_db(employees: Vec<Employee>) -> anyhow::Result<()> {
    let db = get_db();
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

pub fn load_employees_from_db() -> Vec<Employee> {
    let db = get_db();
    let read_txn = db.begin_read().unwrap(); // ✅ 现在 begin_read 可以找到了
    let table = read_txn.open_table(TABLE_EMPLOYEES).unwrap();
    
    let mut list = Vec::new();
    for item in table.iter().unwrap() {
        if let Ok((_, v)) = item {
             // ✅ 修复：v.value() 是 &[u8]，用 from_slice
             // 或者 v.value() 是 &str (因为表定义是<&str, &str>)，直接用 from_str
             if let Ok(emp) = serde_json::from_str::<Employee>(v.value()) {
                 list.push(emp);
             }
        }
    }
    list
}

pub fn parse_config_text(text: &str) -> Vec<Employee> {
    let mut list = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        let parts: Vec<&str> = line.split(&[':', '：'][..]).collect();
        if !parts.is_empty() {
            let name = parts[0].trim().to_string();
            let mut aliases = Vec::new();
            if parts.len() > 1 {
                aliases = parts[1].split(&[',', '，'][..])
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            list.push(Employee { name, aliases });
        }
    }
    list
}

pub fn employees_to_text(list: &[Employee]) -> String {
    list.iter().map(|e| {
        if e.aliases.is_empty() {
            e.name.clone()
        } else {
            format!("{}: {}", e.name, e.aliases.join(", "))
        }
    }).collect::<Vec<_>>().join("\n")
}