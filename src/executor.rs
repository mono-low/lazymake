use anyhow::Result;
use std::process::Command;
use std::path::PathBuf;

pub struct Executor {
    working_dir: PathBuf,
}

impl Executor {
    pub fn new(working_dir: PathBuf) -> Self {
        Self { working_dir }
    }

    pub async fn execute_task(&self, task_name: &str, commands: &[String]) -> Result<(i32, String)> {
        let mut output = String::new();
        let mut exit_code = 0;
        let start_time = std::time::Instant::now();

        output.push_str(&format!("executing task: {}\n", task_name));
        output.push_str(&format!("working directory: {}\n", self.working_dir.display()));
        output.push_str(&"─".repeat(60));
        output.push('\n');

        for cmd in commands {
            let output_result = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", cmd])
                    .current_dir(&self.working_dir)
                    .output()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .current_dir(&self.working_dir)
                    .output()
            };

            match output_result {
                Ok(result) => {
                    output.push_str(&format!("$ {}\n", cmd));
                    output.push_str(&String::from_utf8_lossy(&result.stdout));
                    if !result.stderr.is_empty() {
                        output.push_str(&String::from_utf8_lossy(&result.stderr));
                    }
                    exit_code = result.status.code().unwrap_or(1);
                    if exit_code != 0 {
                        output.push_str(&format!("\nerror: command failed with exit code {}\n", exit_code));
                        break;
                    }
                }
                Err(e) => {
                    output.push_str(&format!("error executing command: {}\n", e));
                    exit_code = 1;
                    break;
                }
            }
        }

        let elapsed = start_time.elapsed();
        output.push_str(&"─".repeat(60));
        output.push('\n');
        output.push_str(&format!("exit code: {}\n", exit_code));
        output.push_str(&format!("execution time: {:.2}s\n", elapsed.as_secs_f64()));

        Ok((exit_code, output))
    }
}
