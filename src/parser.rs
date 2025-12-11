use anyhow::{anyhow, Result};
use std::path::Path;
use std::fs;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub commands: Vec<String>,
    pub file_type: FileType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Makefile,
    Justfile,
}

pub struct Parser {
    file_type: FileType,
}

impl Parser {
    pub fn detect_and_parse(dir: &Path) -> Result<(Vec<Task>, FileType)> {
        if dir.join("justfile").exists() || dir.join("Justfile").exists() {
            let path = if dir.join("justfile").exists() {
                dir.join("justfile")
            } else {
                dir.join("Justfile")
            };
            let content = fs::read_to_string(&path)?;
            Parser::parse_justfile(&content)
        } else if dir.join("Makefile").exists() {
            let content = fs::read_to_string(dir.join("Makefile"))?;
            Parser::parse_makefile(&content)
        } else {
            Err(anyhow!("No Makefile or Justfile found in current directory"))
        }
    }

    fn parse_makefile(content: &str) -> Result<(Vec<Task>, FileType)> {
        let mut tasks = Vec::new();
        let mut current_target: Option<String> = None;
        let mut current_deps: Vec<String> = Vec::new();
        let mut current_commands: Vec<String> = Vec::new();
        let mut current_description = String::new();

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            if line.starts_with('#') && i + 1 < lines.len() {
                let next_line = lines[i + 1];
                if !next_line.starts_with('\t') && !next_line.starts_with(' ') && next_line.contains(':') {
                    current_description = line.trim_start_matches('#').trim().to_string();
                }
            }

            if !line.starts_with('\t') && !line.starts_with(' ') && line.contains(':') && !line.starts_with('#') {
                if let Some(target) = current_target.take() {
                    tasks.push(Task {
                        name: target,
                        description: current_description.clone(),
                        dependencies: current_deps.clone(),
                        commands: current_commands.clone(),
                        file_type: FileType::Makefile,
                    });
                    current_deps.clear();
                    current_commands.clear();
                    current_description.clear();
                }

                let parts: Vec<&str> = line.split(':').collect();
                current_target = Some(parts[0].trim().to_string());
                if parts.len() > 1 {
                    current_deps = parts[1]
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                }
            } else if line.starts_with('\t') {
                let cmd = line.trim_start_matches('\t').to_string();
                if !cmd.is_empty() {
                    current_commands.push(cmd);
                }
            }

            i += 1;
        }

        if let Some(target) = current_target {
            tasks.push(Task {
                name: target,
                description: current_description,
                dependencies: current_deps,
                commands: current_commands,
                file_type: FileType::Makefile,
            });
        }

        Ok((tasks, FileType::Makefile))
    }

    fn parse_justfile(content: &str) -> Result<(Vec<Task>, FileType)> {
        let mut tasks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.starts_with('#') && i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if next_line.starts_with(|c: char| c.is_alphabetic() || c == '_') && next_line.contains(':') {
                    let description = line.trim_start_matches('#').trim().to_string();
                    let recipe_line = next_line;

                    let parts: Vec<&str> = recipe_line.split(':').collect();
                    let name = parts[0].trim().to_string();
                    let deps_str = if parts.len() > 1 { parts[1] } else { "" };

                    let dependencies: Vec<String> = deps_str
                        .split_whitespace()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();

                    let mut commands = Vec::new();
                    i += 2;

                    while i < lines.len() {
                        let cmd_line = lines[i];
                        if cmd_line.starts_with(' ') || cmd_line.starts_with('\t') {
                            let cmd = cmd_line.trim().to_string();
                            if !cmd.is_empty() {
                                commands.push(cmd);
                            }
                            i += 1;
                        } else {
                            break;
                        }
                    }

                    tasks.push(Task {
                        name,
                        description,
                        dependencies,
                        commands,
                        file_type: FileType::Justfile,
                    });
                    continue;
                }
            }

            if line.starts_with(|c: char| c.is_alphabetic() || c == '_') && line.contains(':') && !line.starts_with('#') {
                let parts: Vec<&str> = line.split(':').collect();
                let name = parts[0].trim().to_string();
                let deps_str = if parts.len() > 1 { parts[1] } else { "" };

                let dependencies: Vec<String> = deps_str
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();

                let mut commands = Vec::new();
                i += 1;

                while i < lines.len() {
                    let cmd_line = lines[i];
                    if cmd_line.starts_with(' ') || cmd_line.starts_with('\t') {
                        let cmd = cmd_line.trim().to_string();
                        if !cmd.is_empty() {
                            commands.push(cmd);
                        }
                        i += 1;
                    } else {
                        break;
                    }
                }

                tasks.push(Task {
                    name,
                    description: String::new(),
                    dependencies,
                    commands,
                    file_type: FileType::Justfile,
                });
                continue;
            }

            i += 1;
        }

        Ok((tasks, FileType::Justfile))
    }
}
