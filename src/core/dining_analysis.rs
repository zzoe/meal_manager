//! 就餐数据分析模块 - 处理报餐数据的解析和统计

use regex::Regex;
use std::collections::{HashMap, HashSet};
use anyhow::Result;

use super::employee::EmployeeData;

/// 报表结果
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub lunch_total: u32,
    pub dinner_total: u32,
    pub lunch_details: String,
    pub dinner_details: String,
    pub missing: String,
    pub unknown: String,
}

/// 就餐数据分析器
pub struct DiningAnalyzer {
    pattern: Regex,
}

impl DiningAnalyzer {
    pub fn new() -> Result<Self> {
        let pattern = Regex::new(r"^(.*?)[：:\s]+(\d{2})([*＊]\d+)?$")?;
        
        Ok(Self { pattern })
    }
    
    /// 分析就餐数据
    pub fn analyze(
        &self,
        input_text: &str,
        employees: &[EmployeeData]
    ) -> Result<Report> {
        let mut all_names: HashSet<&str> = HashSet::new();
        let mut nick_map: HashMap<String, &str> = HashMap::new();

        for emp in employees {
            all_names.insert(&emp.name);
            for nick in emp.get_nicknames() {
                nick_map.insert(nick.to_string(), &emp.name);
            }
        }
        
        let mut employee_orders: HashMap<String, (u32, u32)> = HashMap::new();
        let mut unknown_set: HashSet<String> = HashSet::new();

        for line in input_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(caps) = self.pattern.captures(line) {
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
                    let orders = employee_orders.entry(name).or_insert((0, 0));
                    
                    if code.starts_with("10") {
                        orders.0 += 1;
                    } else if code.starts_with("20") {
                        orders.1 += 1;
                    }
                } else {
                    unknown_set.insert(cleaned_nick);
                }
            }
        }

        // 计算总数
        let lunch_total: u32 = employee_orders.values().map(|&(l, _)| l).sum();
        let dinner_total: u32 = employee_orders.values().map(|&(_, d)| d).sum();

        // 生成详情
        let mut lunch_details = String::new();
        let mut dinner_details = String::new();
        
        for (name, &(lunch, dinner)) in &employee_orders {
            if lunch > 0 {
                lunch_details.push_str(&format!("{}: {}份\n", name, lunch));
            }
            if dinner > 0 {
                dinner_details.push_str(&format!("{}: {}份\n", name, dinner));
            }
        }

        // 找出未报餐人员
        let mut missing: Vec<&str> = all_names
            .iter()
            .filter(|name| !employee_orders.contains_key(**name))
            .copied()
            .collect();
        missing.sort();

        let missing_str = if missing.is_empty() {
            "无".to_string()
        } else {
            missing.join(", ")
        };

        let unknown_str = if unknown_set.is_empty() {
            "无".to_string()
        } else {
            unknown_set.iter().cloned().collect::<Vec<_>>().join(", ")
        };

        Ok(Report {
            lunch_total,
            dinner_total,
            lunch_details: if lunch_details.is_empty() { "暂无数据".to_string() } else { lunch_details },
            dinner_details: if dinner_details.is_empty() { "暂无数据".to_string() } else { dinner_details },
            missing: missing_str,
            unknown: unknown_str,
        })
    }
}