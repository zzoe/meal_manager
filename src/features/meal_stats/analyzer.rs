use crate::features::employees::Employee;
use crate::features::employees::storage::EmployeeStorage;
use crate::features::meal_stats::MealAnalysisAction;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn analyze(input_text: String) -> MealAnalysisAction {
    let employees = EmployeeStorage::load_employees();
    let alias_map = build_alias_map(&employees);

    let re = Regex::new(r"^(.*?)[：:\s]+(\d{2})").unwrap();
    let mut valid_data_map: HashMap<String, (u32, u32, String)> = HashMap::new();
    let mut unknown_people = Vec::new();
    let mut error_lines = Vec::new();

    for line in input_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = re.captures(line) {
            let raw_name = caps.get(1).map_or("", |m| m.as_str()).trim();
            let code = caps.get(2).map_or("00", |m| m.as_str());

            if let Some(standard_name) = alias_map.get(raw_name) {
                let chars: Vec<char> = code.chars().collect();
                let lunch = chars.first().and_then(|c| c.to_digit(10)).unwrap_or(0);
                let dinner = chars.get(1).and_then(|c| c.to_digit(10)).unwrap_or(0);
                valid_data_map.insert(standard_name.clone(), (lunch, dinner, code.to_string()));
            } else {
                unknown_people.push(format!("{}({})", raw_name, code));
            }
        } else {
            error_lines.push(format!("格式异常: {}", line));
        }
    }

    let mut lunch_list = Vec::new();
    let mut dinner_list = Vec::new();
    let mut reported_names = HashSet::new();
    let mut total_lunch = 0;
    let mut total_dinner = 0;

    for (name, (lunch, dinner, _)) in &valid_data_map {
        reported_names.insert(name.clone());

        if *lunch > 0 {
            total_lunch += lunch;
            lunch_list.push(format!("{}({}份)", name, lunch));
        }
        if *dinner > 0 {
            total_dinner += dinner;
            dinner_list.push(format!("{}({}份)", name, dinner));
        }
    }

    lunch_list.sort();
    dinner_list.sort();

    let exception_details =
        build_exception_details(&employees, &reported_names, &unknown_people, &error_lines);

    MealAnalysisAction::AnalysisComplete {
        lunch_summary: format!("🍱 中餐 ({}份+工作组2份)", total_lunch),
        lunch_details: if lunch_list.is_empty() {
            "无".to_string()
        } else {
            lunch_list.join("、")
        },
        dinner_summary: format!("🥘 晚餐 ({}份)", total_dinner),
        dinner_details: if dinner_list.is_empty() {
            "无".to_string()
        } else {
            dinner_list.join("、")
        },
        exception_summary: format!("⚠️ 异常监控 ({}条)", exception_details.lines().count()),
        exception_details: if exception_details.is_empty() {
            "无".to_string()
        } else {
            exception_details
        },
    }
}

fn build_alias_map(employees: &[Employee]) -> HashMap<String, String> {
    let mut alias_map = HashMap::new();
    for emp in employees {
        alias_map.insert(emp.name.clone(), emp.name.clone());
        for alias in &emp.aliases {
            alias_map.insert(alias.clone(), emp.name.clone());
        }
    }
    alias_map
}

fn build_exception_details(
    employees: &[Employee],
    reported_names: &HashSet<String>,
    unknown_people: &[String],
    error_lines: &[String],
) -> String {
    let mut lines = Vec::new();

    let missing: Vec<_> = employees
        .iter()
        .filter(|emp| !reported_names.contains(&emp.name))
        .map(|emp| emp.name.clone())
        .collect();

    if !missing.is_empty() {
        lines.push(format!(
            "未报餐 ({}人): {}",
            missing.len(),
            missing.join("、")
        ));
    }

    if !unknown_people.is_empty() {
        lines.push(format!("未知人员: {}", unknown_people.join("、")));
    }

    lines.extend(error_lines.iter().cloned());

    lines.join("\n")
}
