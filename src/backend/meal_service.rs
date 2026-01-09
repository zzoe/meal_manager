use crate::backend::db;
use fastant::Instant;
use makepad_widgets::{ActionDefaultRef, Cx, DefaultNone};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, DefaultNone)]
pub enum MealAnalysisResult {
    Success {
        // 中餐
        lunch_summary: String,
        lunch_details: String,
        // 晚餐
        dinner_summary: String,
        dinner_details: String,
        // 异常 + 未报
        exception_summary: String,
        exception_details: String,
    },
    ConfigLoaded(String),
    ConfigSaved,
    None,
}

pub fn load_config_task() {
    let list = db::load_employees_from_db();
    let text = db::employees_to_text(&list);
    Cx::post_action(MealAnalysisResult::ConfigLoaded(text));
}

pub fn save_config_task(text: String) {
    let list = db::parse_config_text(&text);
    let _ = db::save_employees_to_db(list);
    Cx::post_action(MealAnalysisResult::ConfigSaved);
}

pub fn analyze_meal_data(input_text: String) {
    let start = Instant::now();

    // 1. 加载配置
    let employees = db::load_employees_from_db();
    let mut alias_map = std::collections::HashMap::new();

    for emp in &employees {
        alias_map.insert(emp.name.clone(), emp.name.clone());
        for alias in &emp.aliases {
            alias_map.insert(alias.clone(), emp.name.clone());
        }
    }

    let re = Regex::new(r"^(.*?)[：:\s]+(\d{2})").unwrap();

    // Key: 真名, Value: (中餐份数, 晚餐份数, 原始code)
    let mut valid_data_map: HashMap<String, (u32, u32, String)> = HashMap::new();

    let mut unknown_people = Vec::new(); // 收集未知人员
    let mut error_lines = Vec::new(); // 收集格式错误

    // 2. 解析输入
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
                let l = chars.get(0).and_then(|c| c.to_digit(10)).unwrap_or(0);
                let d = chars.get(1).and_then(|c| c.to_digit(10)).unwrap_or(0);

                // 覆盖写入：以最后一次为准
                valid_data_map.insert(standard_name.clone(), (l, d, code.to_string()));
            } else {
                // 收集未知人员，稍后合并
                unknown_people.push(format!("{}({})", raw_name, code));
            }
        } else {
            error_lines.push(format!("格式异常: {}", line));
        }
    }

    // 3. 统计汇总
    let mut lunch_list = Vec::new();
    let mut dinner_list = Vec::new();
    let mut reported_names = HashSet::new();

    let mut total_lunch = 0;
    let mut total_dinner = 0;

    for (name, (l, d, _)) in &valid_data_map {
        reported_names.insert(name.clone());

        if *l > 0 {
            total_lunch += l;
            lunch_list.push(format!("{}({}份)", name, l));
        }
        if *d > 0 {
            total_dinner += d;
            dinner_list.push(format!("{}({}份)", name, d));
        }
    }

    lunch_list.sort();
    dinner_list.sort();

    // 4. 构建异常列表 (顺序：未报餐 -> 未知人员 -> 格式错误)
    let mut final_exception_lines = Vec::new();

    // A. 未报餐
    let mut missing_list = Vec::new();
    for emp in &employees {
        if !reported_names.contains(&emp.name) {
            missing_list.push(emp.name.clone());
        }
    }
    if !missing_list.is_empty() {
        final_exception_lines.push(format!(
            "未报餐 ({}人): {}",
            missing_list.len(),
            missing_list.join("、")
        ));
    }

    // B. 未知人员 (合并为一条)
    if !unknown_people.is_empty() {
        final_exception_lines.push(format!("未知人员: {}", unknown_people.join("、")));
    }

    // C. 格式错误
    final_exception_lines.extend(error_lines);

    let duration = start.elapsed();

    // 耗时仅日志输出
    println!(
        "统计完成 | 总人数: {} | 中餐: {} | 晚餐: {} | 耗时: {}ms",
        reported_names.len(),
        total_lunch,
        total_dinner,
        duration.as_millis()
    );

    Cx::post_action(MealAnalysisResult::Success {
        // summary removed
        lunch_summary: format!("中餐合计: {} 份", total_lunch),
        lunch_details: if lunch_list.is_empty() {
            "无".to_string()
        } else {
            lunch_list.join("、")
        },
        dinner_summary: format!("晚餐合计: {} 份", total_dinner),
        dinner_details: if dinner_list.is_empty() {
            "无".to_string()
        } else {
            dinner_list.join("、")
        },
        exception_summary: format!("异常/未报: {} 条", final_exception_lines.len()),
        exception_details: if final_exception_lines.is_empty() {
            "无".to_string()
        } else {
            final_exception_lines.join("\n")
        },
    });
}
