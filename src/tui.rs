use crate::app::App;
use crate::executor::Executor;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;

enum InputMode {
    Normal,
    Filter,
    Params,
}

pub async fn run(app: &mut App) -> Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let executor = Executor::new(app.working_dir.clone());
    let result = event_loop(&mut terminal, app, &executor).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    executor: &Executor,
) -> Result<()> {
    let mut mode = InputMode::Normal;

    loop {
        let in_filter_mode = matches!(mode, InputMode::Filter);
        let in_param_mode = matches!(mode, InputMode::Params);
        terminal.draw(|f| ui(f, app, in_filter_mode, in_param_mode))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match mode {
                    InputMode::Filter => {
                        match key.code {
                            KeyCode::Esc => {
                                mode = InputMode::Normal;
                                app.update_filter(String::new());
                            }
                            KeyCode::Enter => {
                                mode = InputMode::Normal;
                            }
                            KeyCode::Char(c) if app.filter.len() < 50 => {
                                app.filter.push(c);
                                app.apply_filter();
                                app.selected_index = 0;
                            }
                            KeyCode::Backspace => {
                                app.filter.pop();
                                app.apply_filter();
                                app.selected_index = 0;
                            }
                            _ => {}
                        }
                    }
                    InputMode::Params => {
                        match key.code {
                            KeyCode::Esc => {
                                mode = InputMode::Normal;
                                app.param_input.clear();
                            }
                            KeyCode::Enter => {
                                mode = InputMode::Normal;
                            }
                            KeyCode::Char(c) if app.param_input.len() < 100 => {
                                app.param_input.push(c);
                            }
                            KeyCode::Backspace => {
                                app.param_input.pop();
                            }
                            _ => {}
                        }
                    }
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                            KeyCode::Up => app.move_selection_up(),
                            KeyCode::Down => app.move_selection_down(),
                            KeyCode::PageUp => {
                                if app.show_output {
                                    app.scroll_output_up(5);
                                } else {
                                    app.page_up(5);
                                }
                            }
                            KeyCode::PageDown => {
                                if app.show_output {
                                    app.scroll_output_down(5);
                                } else {
                                    app.page_down(5);
                                }
                            }
                            KeyCode::Home => app.goto_top(),
                            KeyCode::End => app.goto_bottom(),
                            KeyCode::Char('/') => {
                                mode = InputMode::Filter;
                                app.update_filter(String::new());
                            }
                            KeyCode::Char('p') => {
                                mode = InputMode::Params;
                            }
                            KeyCode::Enter => {
                                if let Some(task) = app.selected_task() {
                                    let task_name = task.name.clone();
                                    let base_commands = task.commands.clone();
                                    let commands = if app.param_input.trim().is_empty() {
                                        base_commands
                                    } else {
                                        base_commands
                                            .into_iter()
                                            .map(|c| format!("{} {}", c, app.param_input))
                                            .collect()
                                    };

                                    match executor.execute_task(&task_name, &commands).await {
                                        Ok((exit_code, output)) => {
                                            app.add_to_history(task_name, exit_code, output.clone());
                                            app.current_output = output;
                                            app.output_scroll = 0;
                                            app.show_output = true;
                                        }
                                        Err(e) => {
                                            app.current_output = format!("Error: {}", e);
                                            app.output_scroll = 0;
                                            app.show_output = true;
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('o') => {
                                app.show_output = !app.show_output;
                            }
                            KeyCode::Char('h') => {
                                if !app.task_history.is_empty() {
                                    let history_text = app
                                        .task_history
                                        .iter()
                                        .rev()
                                        .take(10)
                                        .map(|e| format!("{} - {} (exit: {})", e.timestamp, e.task_name, e.exit_code))
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    app.current_output = history_text;
                                    app.output_scroll = 0;
                                    app.show_output = true;
                                }
                            }
                            KeyCode::Char('g') => {
                                if let Some(task) = app.selected_task() {
                                    let tree = app.dependency_tree(&task.name);
                                    let header = format!("dependency graph for {}:\n\n", task.name);
                                    app.current_output = header + &tree;
                                    app.output_scroll = 0;
                                    app.show_output = true;
                                }
                            }
                            KeyCode::Char('?') => {
                                app.current_output = get_help_text();
                                app.output_scroll = 0;
                                app.show_output = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn get_help_text() -> String {
    "LazyMake - Interactive Make/Justfile TUI\n\n\
     KEYBINDINGS:\n\
     ↑/↓       Navigate tasks\n\
     /         Start filtering (fuzzy search)\n\
     p         Edit task parameters\n\
     Esc       Cancel filter/param input\n\
     Enter     Execute selected task\n\
     g         Show dependency graph for task\n\
     PageUp    Page up (task list or output)\n\
     PageDown  Page down (task list or output)\n\
     Home      Jump to first task\n\
     End       Jump to last task\n\
     o         Toggle output panel\n\
     h         Show task history\n\
     ?         Show this help\n\
     q/Esc     Quit\n\n\
     FEATURES:\n\
     • Browse all tasks with descriptions\n\
     • Fuzzy search filtering\n\
     • View task dependencies and graph\n\
     • Execute tasks with live output\n\
     • Task execution history\n\
     • Interactive parameter input\n\
     • Support for Makefile and Justfile"
        .to_string()
}

fn ui(f: &mut Frame, app: &App, in_filter_mode: bool, in_param_mode: bool) {
    if !app.has_tasks() {
        draw_empty_state(f);
        return;
    }

    let constraints = if app.show_output {
        vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(f.size());

    draw_task_list(f, app, chunks[0], in_filter_mode, in_param_mode);

    if app.show_output {
        draw_output_panel(f, app, chunks[1]);
    }
}

fn draw_empty_state(f: &mut Frame) {
    let area = f.size();
    let message = "no tasks found\n\nmake sure you have a makefile or justfile in this directory\n\npress 'q' to quit";
    
    let paragraph = Paragraph::new(message)
        .block(Block::default().title(" lazymake ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    let centered_area = Rect {
        x: area.width / 4,
        y: area.height / 3,
        width: area.width / 2,
        height: 10,
    };

    f.render_widget(paragraph, centered_area);
}

fn draw_task_list(f: &mut Frame, app: &App, area: Rect, in_filter_mode: bool, in_param_mode: bool) {
    if app.filtered_tasks.is_empty() {
        let message = if app.filter.is_empty() {
            "no tasks found"
        } else {
            "no tasks match current filter"
        };

        let paragraph = Paragraph::new(message)
            .block(
                Block::default()
                    .title(" tasks ")
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        f.render_widget(paragraph, area);
    } else {
        let items: Vec<ListItem> = app
            .filtered_tasks
            .iter()
            .enumerate()
            .map(|(idx, &task_idx)| {
                let task = &app.tasks[task_idx];
                let is_selected = idx == app.selected_index;

                let mut content = vec![Span::styled(
                    format!("  {}", task.name),
                    if is_selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Cyan)
                    },
                )];

                if !task.description.is_empty() {
                    content.push(Span::raw(" "));
                    content.push(Span::styled(
                        format!("({})", task.description),
                        if is_selected {
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::DIM)
                        } else {
                            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)
                        },
                    ));
                }

                if !task.dependencies.is_empty() {
                    content.push(Span::raw(" "));
                    content.push(Span::styled(
                        format!("[deps: {}]", task.dependencies.join(", ")),
                        if is_selected {
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::DIM)
                        } else {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM)
                        },
                    ));
                }

                ListItem::new(Line::from(content))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(
                        " {} tasks ({} shown) ",
                        if app.file_type == crate::parser::FileType::Makefile {
                            "makefile"
                        } else {
                            "justfile"
                        },
                        app.filtered_count()
                    ))
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    let filter_text = if in_filter_mode {
        format!("Filter: {} _", app.filter)
    } else if in_param_mode {
        format!("Params: {} _", app.param_input)
    } else if app.filter.is_empty() && app.param_input.is_empty() {
        "↑↓ Navigate | / Filter | p Params | Enter Run | g Graph | o Output | h History | ? Help | q Quit".to_string()
    } else if !app.filter.is_empty() {
        format!("Filter: {} (Esc to clear)", app.filter)
    } else {
        format!("Params: {} (Esc to clear)", app.param_input)
    };

    let footer = Paragraph::new(filter_text)
        .style(if in_filter_mode {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if in_param_mode {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .alignment(Alignment::Center);

    let footer_area = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };

    f.render_widget(footer, footer_area);
}

fn draw_output_panel(f: &mut Frame, app: &App, area: Rect) {
    let output = Paragraph::new(app.current_output.as_str())
        .block(Block::default().title(" output ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .wrap(Wrap { trim: true })
        .scroll((app.output_scroll, 0));

    f.render_widget(output, area);
}
