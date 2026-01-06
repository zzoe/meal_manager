use crate::app::{Action, AppEvent, EmployeeData, Report};
use crate::db::{delete_employee, init_db, load_all_employees, save_employee};
use crate::error::{AppError, Result};
use crossbeam_channel::Receiver;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;

/// 分析就餐数据，生成统计报告
pub fn analyze_dining(input: &str, employees: &[EmployeeData]) -> Result<Report> {
    let mut all_names: HashSet<&str> = HashSet::new();
    let mut nick_map: HashMap<String, &str> = HashMap::new();

    for emp in employees {
        all_names.insert(&emp.name);
        for nick in emp.nicknames.split(',') {
            let n = nick.trim();
            if !n.is_empty() {
                nick_map.insert(n.to_string(), &emp.name);
            }
        }
    }

    let re = Regex::new(r"^(.*?)[：:\s]+(\d{2})([*＊]\d+)?$")
        .map_err(AppError::Regex)?;
    
    let mut employee_orders: HashMap<String, (u32, u32)> = HashMap::new(); // name -> (lunch, dinner)
    let mut unknown_set: HashSet<String> = HashSet::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = re.captures(line) {
            // 使用 get(1) 和 get(2) 是安全的，因为正则表达式保证了这些组存在
            let nick = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let cleaned_nick = nick.replace('\'', "");
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("");

            let real_name = if let Some(name) = nick_map.get(cleaned_nick.as_str()) {
                Some(name.to_string())
            } else if all_names.contains(cleaned_nick.as_str()) {
                Some(cleaned_nick.to_string())
            } else {
                None
            };

            if let Some(name) = real_name {
                let chars: Vec<char> = code.chars().collect();
                if chars.len() >= 2 {
                    let l_n = chars[0].to_digit(10).unwrap_or(0);
                    let d_n = chars[1].to_digit(10).unwrap_or(0);
                    // 后面的记录会覆盖前面的，以最后一次为准
                    employee_orders.insert(name, (l_n, d_n));
                }
            } else {
                // 格式正确但员工不存在
                unknown_set.insert(nick.to_string());
            }
        } else {
            // 格式完全不正确的行
            unknown_set.insert(line.to_string());
        }
    }

    let mut report = Report::default();
    let mut lunch_list = Vec::new();
    let mut dinner_list = Vec::new();
    let mut reported_names = HashSet::new();

    for (name, (l_n, d_n)) in employee_orders {
        reported_names.insert(name.clone());
        if l_n > 0 {
            report.lunch_total += l_n;
            lunch_list.push(format!("{}({})", name, l_n));
        }
        if d_n > 0 {
            report.dinner_total += d_n;
            dinner_list.push(format!("{}({})", name, d_n));
        }
        // 用餐数为00的员工，报了餐但不吃饭，不计入失踪
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

    // 失踪 = 全部员工 - 已报餐的员工
    let missing: Vec<_> = all_names
        .iter()
        .filter(|&&name| !reported_names.contains(name))
        .map(|name| name.to_string())
        .collect();
    report.missing = if missing.is_empty() {
        "全员已报".into()
    } else {
        missing.join("、")
    };

    let unknown_list: Vec<_> = unknown_set.into_iter().collect();
    report.unknown = if unknown_list.is_empty() {
        "".into()
    } else {
        unknown_list.join("、")
    };

    Ok(report)
}

/// 启动后台工作线程 (使用 compio)
pub fn spawn_worker(rx_action: Receiver<Action>, tx_event: Sender<AppEvent>) {
    std::thread::spawn(move || {
        // 初始化数据库
        let db = init_db();
        
        // 初始化运行时
        let runtime = match compio::runtime::Runtime::new() {
            Ok(runtime) => Some(runtime),
            Err(e) => {
                let error_msg = format!("运行时创建失败: {}，相关功能将不可用", e);
                eprintln!("{}", error_msg);
                let _ = tx_event.send(AppEvent::StatusMessage(error_msg));
                None
            }
        };
        
        // 处理操作的核心逻辑
        fn handle_action(db: &redb::Database, tx_event: &Sender<AppEvent>, action: Action) {
            match action {
                Action::LoadEmployees => {
                    match load_all_employees(db) {
                        Ok(list) => {
                            let _ = tx_event.send(AppEvent::EmployeesLoaded(list));
                        }
                        Err(e) => {
                            let _ = tx_event.send(AppEvent::StatusMessage(format!("加载员工数据失败: {}", e)));
                        }
                    }
                }
                Action::SaveEmployee(emp) => {
                    match save_employee(db, &emp) {
                        Ok(_) => {
                            if let Ok(list) = load_all_employees(db) {
                                let _ = tx_event.send(AppEvent::EmployeesLoaded(list));
                                let _ = tx_event.send(AppEvent::StatusMessage(format!("已保存: {}", emp.name)));
                            } else {
                                let _ = tx_event.send(AppEvent::StatusMessage("保存成功，但重新加载员工列表失败".into()));
                            }
                        }
                        Err(e) => {
                            let _ = tx_event.send(AppEvent::StatusMessage(format!("保存员工失败: {}", e)));
                        }
                    }
                }
                Action::DeleteEmployee(name) => {
                    match delete_employee(db, &name) {
                        Ok(_) => {
                            if let Ok(list) = load_all_employees(db) {
                                let _ = tx_event.send(AppEvent::EmployeesLoaded(list));
                                let _ = tx_event.send(AppEvent::StatusMessage(format!("已删除: {}", name)));
                            } else {
                                let _ = tx_event.send(AppEvent::StatusMessage("删除成功，但重新加载员工列表失败".into()));
                            }
                        }
                        Err(e) => {
                            let _ = tx_event.send(AppEvent::StatusMessage(format!("删除员工失败: {}", e)));
                        }
                    }
                }
                Action::UpdateEmployee { old_name, new_data } => {
                    match delete_employee(db, &old_name) {
                        Ok(_) => {
                            match save_employee(db, &new_data) {
                                Ok(_) => {
                                    if let Ok(list) = load_all_employees(db) {
                                        let _ = tx_event.send(AppEvent::EmployeesLoaded(list));
                                        let _ = tx_event.send(AppEvent::StatusMessage(format!("已更新: {}", new_data.name)));
                                    } else {
                                        let _ = tx_event.send(AppEvent::StatusMessage("更新成功，但重新加载员工列表失败".into()));
                                    }
                                }
                                Err(e) => {
                                    let _ = tx_event.send(AppEvent::StatusMessage(format!("保存新员工失败: {}", e)));
                                    // 尝试恢复旧记录
                                    let _ = save_employee(db, &EmployeeData {
                                        name: old_name.clone(),
                                        nicknames: "".to_string(),
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx_event.send(AppEvent::StatusMessage(format!("删除旧员工失败: {}", e)));
                        }
                    }
                }
                Action::Calculate(input) => {
                    match load_all_employees(db) {
                        Ok(employees) => {
                            match analyze_dining(&input, &employees) {
                                Ok(report) => {
                                    let _ = tx_event.send(AppEvent::ReportReady(report));
                                }
                                Err(e) => {
                                    let _ = tx_event.send(AppEvent::StatusMessage(format!("分析数据失败: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx_event.send(AppEvent::StatusMessage(format!("加载员工数据失败: {}", e)));
                        }
                    }
                }
            }
        }
        
        // 根据运行时是否可用选择同步或异步处理
        if let Some(runtime) = runtime {
            runtime.block_on(async move {
                while let Ok(action) = rx_action.recv() {
                    handle_action(&db, &tx_event, action);
                }
            });
        } else {
            // 同步处理循环
            while let Ok(action) = rx_action.recv() {
                handle_action(&db, &tx_event, action);
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
            EmployeeData {
                name: "张三".to_string(),
                nicknames: "小张,三哥".to_string(),
            },
            EmployeeData {
                name: "李四".to_string(),
                nicknames: "小李".to_string(),
            },
        ];
        let input = "张三: 11\n小李 10\n王五 01";
        let report = analyze_dining(input, &employees).unwrap();

        assert_eq!(report.lunch_total, 2);
        assert_eq!(report.dinner_total, 1);
        assert_contains(&report.lunch_details, "张三");
        assert_contains(&report.lunch_details, "李四");
        assert_contains(&report.unknown, "王五");
    }

    #[test]
    fn test_analyze_dining_with_duplicate_records() {
        let employees = vec![
            EmployeeData {
                name: "张三".to_string(),
                nicknames: "".to_string(),
            },
            EmployeeData {
                name: "李四".to_string(),
                nicknames: "".to_string(),
            },
        ];
        // 张三出现两次，以最后一次(10)为准
        let input = "张三: 11\n李四: 10\n张三: 10";
        let report = analyze_dining(input, &employees).unwrap();

        assert_eq!(report.lunch_total, 2); // 张三1 + 李四1
        assert_eq!(report.dinner_total, 0); 
        // 张三应该只出现一次
        assert_eq!(report.lunch_details.split("、").filter(|x| x.contains("张三")).count(), 1);
        assert!(!report.dinner_details.contains("张三")); // 张三最后一次是10，不吃晚餐
    }

    #[test]
    fn test_analyze_dining_with_days_marker() {
        let employees = vec![
            EmployeeData {
                name: "Ken".to_string(),
                nicknames: "Ken''".to_string(),
            },
            EmployeeData {
                name: "张三".to_string(),
                nicknames: "".to_string(),
            },
        ];
        let input = "Ken'': 00*2\n张三 11＊3";
        let report = analyze_dining(input, &employees).unwrap();

        assert_eq!(report.lunch_total, 1); // 张三1
        assert_eq!(report.dinner_total, 1); // 张三1
        // Ken报了00，不显示在列表中
        assert!(!report.lunch_details.contains("Ken"));
        assert!(!report.dinner_details.contains("Ken"));
        // Ken报了餐，所以不计入失踪
        assert!(!report.missing.contains("Ken"));
        assert_contains(&report.lunch_details, "张三(1)");
        assert_contains(&report.dinner_details, "张三(1)");
    }

    #[test]
    fn test_analyze_dining_with_unknown_names() {
        let employees = vec![
            EmployeeData {
                name: "张三".to_string(),
                nicknames: "".to_string(),
            },
        ];
        let input = "张三: 11\nunknown: 10\n王五: 01";
        let report = analyze_dining(input, &employees).unwrap();

        assert_eq!(report.lunch_total, 1); // 张三1（unknown不匹配）
        assert_eq!(report.dinner_total, 1); // 张三1（unknown不匹配）
        assert_contains(&report.unknown, "unknown");
        assert_contains(&report.unknown, "王五");
        // 未知员工不计入失踪
        assert!(!report.missing.contains("unknown"));
        assert!(!report.missing.contains("王五"));
    }

    fn assert_contains(haystack: &str, needle: &str) {
        assert!(
            haystack.contains(needle),
            "Expected '{}' to contain '{}'",
            haystack,
            needle
        );
    }
}
