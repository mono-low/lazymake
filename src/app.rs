use crate::parser::{Task, Parser, FileType};
use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct App {
    pub tasks: Vec<Task>,
    pub file_type: FileType,
    pub selected_index: usize,
    pub filter: String,
    pub param_input: String,
    pub filtered_tasks: Vec<usize>,
    pub task_history: Vec<TaskExecution>,
    pub current_output: String,
    pub show_output: bool,
    pub output_scroll: u16,
    pub working_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct TaskExecution {
    pub task_name: String,
    pub timestamp: String,
    pub exit_code: i32,
    pub output: String,
}

impl App {
    pub fn new(working_dir: PathBuf) -> Result<Self> {
        let (tasks, file_type) = Parser::detect_and_parse(&working_dir)?;
        let filtered_tasks: Vec<usize> = (0..tasks.len()).collect();

        Ok(Self {
            tasks,
            file_type,
            selected_index: 0,
            filter: String::new(),
            param_input: String::new(),
            filtered_tasks,
            task_history: Vec::new(),
            current_output: String::new(),
            show_output: false,
            output_scroll: 0,
            working_dir,
        })
    }

    pub fn update_filter(&mut self, filter: String) {
        self.filter = filter;
        self.apply_filter();
        self.selected_index = 0;
    }

    pub fn apply_filter(&mut self) {
        if self.filter.is_empty() {
            self.filtered_tasks = (0..self.tasks.len()).collect();
        } else {
            use fuzzy_matcher::FuzzyMatcher;
            use fuzzy_matcher::skim::SkimMatcherV2;
            
            let matcher = SkimMatcherV2::default();
            self.filtered_tasks = self
                .tasks
                .iter()
                .enumerate()
                .filter_map(|(i, task)| {
                    let name_score = matcher.fuzzy_match(&task.name, &self.filter).unwrap_or(0);
                    let desc_score = matcher.fuzzy_match(&task.description, &self.filter).unwrap_or(0);
                    if name_score > 0 || desc_score > 0 {
                        Some((i, name_score.max(desc_score)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|(i, _)| i)
                .collect();
        }
    }

    pub fn selected_task(&self) -> Option<&Task> {
        self.filtered_tasks
            .get(self.selected_index)
            .and_then(|&idx| self.tasks.get(idx))
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.filtered_tasks.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn page_up(&mut self, lines: usize) {
        if self.selected_index > lines {
            self.selected_index -= lines;
        } else {
            self.selected_index = 0;
        }
    }

    pub fn page_down(&mut self, lines: usize) {
        let len = self.filtered_tasks.len();
        if len == 0 {
            return;
        }
        let max_index = len - 1;
        self.selected_index = (self.selected_index + lines).min(max_index);
    }

    pub fn goto_top(&mut self) {
        if !self.filtered_tasks.is_empty() {
            self.selected_index = 0;
        }
    }

    pub fn goto_bottom(&mut self) {
        let len = self.filtered_tasks.len();
        if len > 0 {
            self.selected_index = len - 1;
        }
    }

    pub fn add_to_history(&mut self, task_name: String, exit_code: i32, output: String) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.task_history.push(TaskExecution {
            task_name,
            timestamp,
            exit_code,
            output,
        });
        
        if self.task_history.len() > 100 {
            self.task_history.remove(0);
        }
    }

    pub fn scroll_output_up(&mut self, lines: u16) {
        self.output_scroll = self.output_scroll.saturating_sub(lines);
    }

    pub fn scroll_output_down(&mut self, lines: u16) {
        self.output_scroll = self.output_scroll.saturating_add(lines);
    }

    pub fn has_tasks(&self) -> bool {
        !self.tasks.is_empty()
    }

    pub fn filtered_count(&self) -> usize {
        self.filtered_tasks.len()
    }

    pub fn get_task_dependencies(&self, task_name: &str) -> Vec<&Task> {
        if let Some(task) = self.tasks.iter().find(|t| t.name == task_name) {
            task.dependencies
                .iter()
                .filter_map(|dep| self.tasks.iter().find(|t| t.name == *dep))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn dependency_tree(&self, task_name: &str) -> String {
        let mut visited = HashSet::new();
        let mut lines = String::new();
        self.build_dependency_lines(task_name, 0, &mut visited, &mut lines);
        lines
    }

    fn build_dependency_lines(
        &self,
        task_name: &str,
        depth: usize,
        visited: &mut HashSet<String>,
        lines: &mut String,
    ) {
        let indent = "  ".repeat(depth);

        if !visited.insert(task_name.to_string()) {
            lines.push_str(&format!("{indent}- {task_name} (cycle)\n"));
            return;
        }

        lines.push_str(&format!("{indent}- {task_name}\n"));

        for dep in self.get_task_dependencies(task_name) {
            self.build_dependency_lines(&dep.name, depth + 1, visited, lines);
        }
    }
}
