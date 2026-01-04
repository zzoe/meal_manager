use crate::app::{Report, EmployeeData, Action, AppEvent};
use crate::db::{init_db, load_all_employees, save_employee, delete_employee};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use crossbeam_channel::Receiver;
use std::sync::mpsc::Sender;

/// 分析就餐数据，生成统计报告
pub fn analyze_dining(input: &str, employees: &[EmployeeData]) -> Report {
    let mut all_names = HashSet::new();
    let mut nick_map = HashMap::new();

    for emp in employees {
        all_names.insert(emp.name.clone());
        for nick in emp.nicknames.split(',') {
            let n = nick.trim();
            if !n.is_empty() {
                nick_map.insert(n.to_string(), emp.name.clone());
            }
        }
    }

    let re = Regex::new(r"^(.*?)[：:\s]+(\d{2})$").unwrap();
    let mut report = Report::default();
    let mut reported_names = HashSet::new();
    let mut lunch_list = Vec::new();
    let mut dinner_list = Vec::new();
    let mut unknown_list = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = re.captures(line) {
            let nick = caps.get(1).unwrap().as_str().trim();
            let code = caps.get(2).unwrap().as_str();

            let real_name = if let Some(name) = nick_map.get(nick) {
                Some(name.clone())
            } else if all_names.contains(nick) {
                Some(nick.to_string())
            } else {
                None
            };

            if let Some(name) = real_name {
                reported_names.insert(name.clone());
                let chars: Vec<char> = code.chars().collect();
                if chars.len() >= 2 {
                    let l_n = chars[0].to_digit(10).unwrap_or(0);
                    let d_n = chars[1].to_digit(10).unwrap_or(0);

                    if l_n > 0 {
                        report.lunch_total += l_n;
                        lunch_list.push(format!("{}({})", name, l_n));
                    }
                    if d_n > 0 {
                        report.dinner_total += d_n;
                        dinner_list.push(format!("{}({})", name, d_n));
                    }
                }
            } else {
                unknown_list.push(nick.to_string());
            }
        }
    }

    report.lunch_details = if lunch_list.is_empty() {
        "无".into()
    } else {
        lunch_list.join("、")
    };
    report.dinner_details = if dinner_list.is_empty() {
        "无".into()
    } else {
        dinner_list.join("、")
    };

    let missing: Vec<_> = all_names.difference(&reported_names).cloned().collect();
    report.missing = if missing.is_empty() {
        "全员已报".into()
    } else {
        missing.join("、")
    };

    report.unknown = if unknown_list.is_empty() {
        "".into()
    } else {
        unknown_list.join("、")
    };

    report
}

/// 启动后台工作线程 (使用 compio)
pub fn spawn_worker(rx_action: Receiver<Action>, tx_event: Sender<AppEvent>) {
    std::thread::spawn(move || {
        let db = init_db();
        
        while let Ok(action) = rx_action.recv() {
            match action {
                Action::LoadEmployees => {
                    let list = load_all_employees(&db);
                    tx_event.send(AppEvent::EmployeesLoaded(list)).unwrap();
                }
                Action::SaveEmployee(emp) => {
                    save_employee(&db, &emp);
                    let list = load_all_employees(&db);
                    tx_event.send(AppEvent::EmployeesLoaded(list)).unwrap();
                    tx_event.send(AppEvent::StatusMessage(format!("已保存: {}", emp.name))).unwrap();
                }
                Action::DeleteEmployee(name) => {
                    delete_employee(&db, &name);
                    let list = load_all_employees(&db);
                    tx_event.send(AppEvent::EmployeesLoaded(list)).unwrap();
                    tx_event.send(AppEvent::StatusMessage(format!("已删除: {}", name))).unwrap();
                }
                Action::Calculate(input) => {
                    let employees = load_all_employees(&db);
                    let report = analyze_dining(&input, &employees);
                    tx_event.send(AppEvent::ReportReady(report)).unwrap();
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::EmployeeData;

    #[test]
    fn test_analyze_dining() {
        let employees = vec![
            EmployeeData { name: "张三".to_string(), nicknames: "小张,三哥".to_string() },
            EmployeeData { name: "李四".to_string(), nicknames: "小李".to_string() },
        ];
        let input = "张三: 11\n小李 10\n王五 01";
        let report = analyze_dining(input, &employees);

        assert_eq!(report.lunch_total, 2);
        assert_eq!(report.dinner_total, 1);
        assert_contains(&report.lunch_details, "张三");
        assert_contains(&report.lunch_details, "李四");
        assert_contains(&report.unknown, "王五");
    }

    fn assert_contains(haystack: &str, needle: &str) {
        assert!(haystack.contains(needle), "Expected '{}' to contain '{}'", haystack, needle);
    }
}
