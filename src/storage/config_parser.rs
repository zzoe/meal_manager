use crate::domain::Employee;

pub fn parse_employee_config(text: &str) -> Vec<Employee> {
    let mut list = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(&[':', '：'][..]).collect();
        if !parts.is_empty() {
            let name = parts[0].trim().to_string();
            let mut aliases = Vec::new();
            if parts.len() > 1 {
                aliases = parts[1]
                    .split(&[',', '，'][..])
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            list.push(Employee::new(name, aliases));
        }
    }
    list
}

pub fn format_employee_config(list: &[Employee]) -> String {
    list.iter()
        .map(|e| {
            if e.aliases.is_empty() {
                e.name.clone()
            } else {
                format!("{}: {}", e.name, e.aliases.join(", "))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
