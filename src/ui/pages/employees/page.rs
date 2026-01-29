use makepad_widgets::*;
use crate::employees::{Employee, load_config, add_employee_config, update_employee_config, delete_employee_config};
use super::config_item::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::components::common::*;
    use crate::ui::pages::employees::config_item::EmployeeConfigItem;

    // 配置页面 - 使用PortalList实现虚拟滚动
    pub ConfigPage = {{ConfigPage}} {
        width: Fill, height: Fill
        
        main_view = <View> {
            width: Fill, height: Fill
            flow: Down, spacing: 5.0, padding: 30.0
 
            <View> {
                width: Fill, height: Fit
                flow: Right, align: {y: 0.5}
                
                <View> {
                    width: Fill, height: Fit
                    flow: Down, spacing: 2.0
                    <Label> {
                        text: "员工配置"
                        draw_text: { color: (COLOR_TEXT_PRIMARY), text_style: { font_size: 18.0 } }
                    }
                }

                btn_refresh = <Button> {
                    width: 32.0, height: 32.0
                    text: "↻"
                    draw_text: {
                        color: (COLOR_PRIMARY)
                        color_hover: (COLOR_TEXT_PRIMARY)
                        color_focus: (COLOR_PRIMARY)
                        text_style: { font_size: 18.0 }
                    }
                    draw_bg: {
                        color: #0000
                        border_radius: 16.0
                        border_size: 0.0
                    }
                }
            }

            <View> {
                width: Fill, height: Fill
                flow: Down, spacing: 0.0, margin: { top: 10.0 }
                
                // 固定表头
                <View> {
                    width: Fill, height: 35.0
                    flow: Right, spacing: 10.0, padding: {left: 15.0, right: 15.0}, align: {y: 0.5}
                    show_bg: true, draw_bg: { color: (COLOR_BG_APP) }
                    
                    <Label> {
                        width: 40.0, text: "#"
                        draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 10.0 } }
                    }
                    <Label> {
                        width: 140.0, text: "姓名"
                        draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 10.0 } }
                    }
                    <Label> {
                        width: Fill, text: "昵称"
                        draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 10.0 } }
                    }
                    <Label> {
                        width: 50.0, text: "操作"
                        draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 10.0 } }
                    }
                }

                // PortalList是虚拟列表，只渲染可见项
                employee_list = <PortalList> {
                    width: Fill, height: Fill
                    item = <EmployeeConfigItem> {}
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ConfigPageAction {
    ValidationError(String),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ConfigPage {
    #[deref]
    view: View,

    #[rust]
    employees: Vec<Employee>,
    
    #[rust]
    db_employees: Vec<Employee>,

    #[rust]
    draft_new_employee: Employee,
}

impl Widget for ConfigPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 先绘制基础视图
        let step = self.view.draw_walk(cx, scope, walk);

        let list_ref = self.portal_list(id!(employee_list));

        if let Some(mut list) = list_ref.borrow_mut() {
            // 列表长度：员工数 + 1个待填写的空行
            list.set_item_range(cx, 0, self.employees.len() + 1);

            while let Some(index) = list.next_visible_item(cx) {
                let widget = list.item(cx, index, live_id!(item));
                let employee_item = widget.as_employee_config_item();
                if !employee_item.is_empty() {
                    if index < self.employees.len() {
                        // 传实时数据和对应的 DB 原始数据进行对比
                        let original = self.db_employees.get(index).unwrap_or(&self.employees[index]);
                        employee_item.set_employee(cx, index, &self.employees[index], original, false);
                    } else {
                        // 最后一项是待填写的空行，从草稿箱中读取数据
                        employee_item.set_employee(cx, index, &self.draft_new_employee, &Employee::new(String::new(), Vec::new()), true);
                    }
                    widget.draw_all(cx, scope);
                }
            }
        }

        step
    }
}

impl WidgetMatchEvent for ConfigPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // 刷新按钮
        if self.button(id!(btn_refresh)).clicked(actions) {
            log!("ConfigPage: Refreshing employees...");
            load_config();
        }

        // 处理行动作
        for action in actions {
            if let Some(item_action) = action.as_widget_action().map(|wa| wa.cast::<EmployeeConfigItemAction>()) {
                match item_action {
                    EmployeeConfigItemAction::Delete(index) => {
                        if index < self.employees.len() {
                            let employee = self.employees.remove(index);
                            self.db_employees.remove(index);
                            log!("ConfigPage: Deleting employee {}", employee.name);
                            
                            // 仅删除该行数据
                            let name = employee.name.clone();
                            cx.spawn_thread(move || delete_employee_config(name));
                            
                            self.view.redraw(cx);
                        }
                    }
                    EmployeeConfigItemAction::Save(index, employee) => {
                        // 1. 数据清理与校验
                        let name = employee.name.trim().to_string();
                        if name.is_empty() {
                            cx.widget_action(self.widget_uid(), &scope.path, ConfigPageAction::ValidationError("姓名不能为空".to_string()));
                            return;
                        }

                        // 检查重名 (排除自身)
                        if self.employees.iter().enumerate().any(|(i, emp)| i != index && emp.name == name) {
                            cx.widget_action(self.widget_uid(), &scope.path, ConfigPageAction::ValidationError(format!("员工姓名 '{}' 已存在", name)));
                            return;
                        }

                        // 昵称去重逻辑
                        let mut aliases: Vec<String> = employee.aliases.iter()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        aliases.sort();
                        aliases.dedup();

                        let mut cleaned_employee = employee.clone();
                        cleaned_employee.name = name;
                        cleaned_employee.aliases = aliases;

                        // 2. 执行保存逻辑
                        let mut is_added = false;
                        if index < self.employees.len() {
                            let old_name = self.db_employees[index].name.clone();
                            self.employees[index] = cleaned_employee.clone();
                            self.db_employees[index] = cleaned_employee.clone();
                            log!("ConfigPage: Updating employee {} (old: {})", cleaned_employee.name, old_name);
                            
                            // 仅修改该行数据
                            let new_emp = cleaned_employee.clone();
                            cx.spawn_thread(move || update_employee_config(old_name, new_emp));
                        } else {
                            // 保存的是最后一行空行，即新增
                            self.employees.push(cleaned_employee.clone());
                            self.db_employees.push(cleaned_employee.clone());
                            // 重置草稿箱
                            self.draft_new_employee = Employee::default();
                            log!("ConfigPage: Adding new employee {}", cleaned_employee.name);
                            is_added = true;
                            
                            // 仅新增该行数据
                            let new_emp = cleaned_employee.clone();
                            cx.spawn_thread(move || add_employee_config(new_emp));
                        }
                        
                        if is_added {
                            // 自动滚动到新的空行 (此时 len 已经 push 过，所以 len 指向新的空行)
                            let list = self.portal_list(id!(employee_list));
                            list.set_first_id_and_scroll(self.employees.len(), 0.0);
                        }
                        self.view.redraw(cx);
                    }
                    EmployeeConfigItemAction::Changed(index, employee) => {
                        if index < self.employees.len() {
                            self.employees[index] = employee.clone();
                        } else {
                            // 更新最后一行新增行的草稿
                            self.draft_new_employee = employee.clone();
                        }
                    }
                    EmployeeConfigItemAction::None => {}
                }
            }
        }
    }
}

impl ConfigPageRef {
    pub fn set_employees(&self, cx: &mut Cx, employees: Vec<Employee>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.employees = employees.clone();
            inner.db_employees = employees;
            inner.draft_new_employee = Employee::default();
            inner.view.redraw(cx);
        }
    }

    pub fn get_employees(&self) -> Option<Vec<Employee>> {
        self.borrow().map(|inner| inner.employees.clone())
    }
}
