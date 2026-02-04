use crate::employees::Employee;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::components::common::*;

    pub EmployeeConfigItem = {{EmployeeConfigItem}} {
        width: Fill, height: 45.0
        flow: Overlay, margin: {bottom: 1.0}

        bg = <RoundedView> {
            width: Fill, height: Fill
            draw_bg: {
                color: (COLOR_BG_CARD)
                border_color: (COLOR_BORDER)
                border_size: 0.5
                border_radius: 0.0
            }
        }

        content = <View> {
            width: Fill, height: Fill
            flow: Right, spacing: 10.0, padding: {left: 15.0, right: 15.0}, align: {y: 0.5}

            // 序号
            id_label = <Label> {
                width: 40.0, height: Fit
                text: "0"
                draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 10.0 } }
            }

            // 姓名
            name_input = <StyledTextInput> {
                width: 140.0, height: Fit
                empty_text: "输入姓名..."
            }

            // 昵称
            aliases_input = <StyledTextInput> {
                width: Fill, height: Fit
                empty_text: "例如: 小明, zhangsan..."
            }

            // 操作区域
            action_btn = <View> {
                width: 50.0, height: Fit
                save = <RowSaveButton> { visible: false }
                delete = <RowDeleteButton> { visible: true }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct EmployeeConfigItem {
    #[deref]
    view: View,

    #[rust]
    index: usize,

    #[rust]
    is_new_item: bool,

    #[rust]
    employee_cache: Option<Employee>,
}

impl Widget for EmployeeConfigItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum EmployeeConfigItemAction {
    Delete(usize),
    Save(usize, Employee),
    Changed(usize, Employee),
    None,
}

impl WidgetMatchEvent for EmployeeConfigItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // 处理删除按钮
        if self.button(id!(action_btn.delete)).clicked(actions) {
            let action = EmployeeConfigItemAction::Delete(self.index);
            cx.widget_action(self.widget_uid(), &scope.path, action);
            return;
        }

        // 处理保存按钮
        if self.button(id!(action_btn.save)).clicked(actions) {
            let employee = self.get_current_employee();
            let action = EmployeeConfigItemAction::Save(self.index, employee);
            cx.widget_action(self.widget_uid(), &scope.path, action);
            return;
        }

        // 监听输入变化，实时同步到页面并切换 保存/删除 按钮
        let name_input = self.text_input(id!(name_input));
        let aliases_input = self.text_input(id!(aliases_input));

        if name_input.changed(actions).is_some() || aliases_input.changed(actions).is_some() {
            // 发出变化通知，让 Page 同步 Live 数据
            let employee = self.get_current_employee();
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                EmployeeConfigItemAction::Changed(self.index, employee),
            );

            self.check_modifications(cx);
        }
    }
}

impl EmployeeConfigItem {
    fn get_current_employee(&self) -> Employee {
        let name = self.text_input(id!(name_input)).text();
        let aliases_text = self.text_input(id!(aliases_input)).text();

        let names: Vec<String> = aliases_text
            .split(&[',', '，'][..])
            .map(|s: &str| s.trim().to_string())
            .filter(|s: &String| !s.is_empty())
            .collect();

        Employee::new(name, names)
    }

    fn check_modifications(&mut self, cx: &mut Cx) {
        // 如果是新行，永远显示保存按钮
        if self.is_new_item {
            self.button(id!(action_btn.save)).set_visible(cx, true);
            self.button(id!(action_btn.delete)).set_visible(cx, false);
            self.view.redraw(cx);
            return;
        }

        if let Some(cache) = &self.employee_cache {
            let current_emp = self.get_current_employee();

            let is_modified = current_emp.name.trim() != cache.name.trim()
                || current_emp.aliases != cache.aliases;

            let has_content = !current_emp.name.trim().is_empty();
            let show_save = is_modified && has_content;

            self.button(id!(action_btn.save)).set_visible(cx, show_save);
            self.button(id!(action_btn.delete))
                .set_visible(cx, !show_save);
            self.view.redraw(cx);
        }
    }
}

impl EmployeeConfigItemRef {
    pub fn set_employee(
        &self,
        cx: &mut Cx,
        index: usize,
        current: &Employee,
        original: &Employee,
        is_new_item: bool,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            let index_changed = inner.index != index;

            inner.index = index;
            inner.is_new_item = is_new_item;
            // 重要：必须先同步缓存，因为 check_modifications 依赖它
            inner.employee_cache = Some(original.clone());

            inner
                .label(id!(id_label))
                .set_text(cx, &format!("{}", index + 1));

            let name_input = inner.text_input(id!(name_input));
            // 姓名：简单的字符串，直接比对内容。如果内容一致，绝对不调用 set_text，防止光标跳动或双字符问题
            if name_input.text() != current.name {
                name_input.set_text(cx, &current.name);
            }

            let aliases_input = inner.text_input(id!(aliases_input));
            let new_aliases_text = current.aliases.join(", ");
            // 别名：涉及格式化。逻辑如下：
            // 1. 如果完全切行了 (index_changed)，必须强制刷新，无论有没有焦点（复用 widget）
            // 2. 如果没切行且有焦点，为了防止 auto-format 打断用户输入（如输入空格），只要还没有完全偏离（或者干脆信任用户），就不更新
            // 这里我们采用：有焦点就不更新，除非切行。
            if index_changed
                || (!aliases_input.key_focus(cx) && aliases_input.text() != new_aliases_text)
            {
                aliases_input.set_text(cx, &new_aliases_text);
            }

            // 初始状态判定：强制刷新按钮状态
            if is_new_item {
                inner.button(id!(action_btn.save)).set_visible(cx, true);
                inner.button(id!(action_btn.delete)).set_visible(cx, false);
            } else {
                inner.check_modifications(cx);
            }

            inner.view.redraw(cx);
        }
    }
}
