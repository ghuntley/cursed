use crate::error::CursedError;
// Tokio process integration for CURSED async runtime
use std::process::Stdio;
use std::collections::HashMap;
use std::time::Duration;
use std::io::Result;

/// Async process command builder
#[derive(Debug)]
pub struct Command {
impl Command {
    pub fn new(program: &str) -> Self {
        Self {
        }
    }
    
    pub fn arg<S: AsRef<str>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    pub fn args<I, S>(&mut self, args: I) -> &mut Self 
    where
    {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self 
    where
    {
        self.env.insert(key.as_ref().to_string(), val.as_ref().to_string());
        self
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self 
    where
    {
        for (key, val) in vars {
            self.env.insert(key.as_ref().to_string(), val.as_ref().to_string());
        }
        self
    pub fn current_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Self {
        self.current_dir = Some(dir.as_ref().to_string());
        self
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stdin = Some(cfg.into());
        self
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stdout = Some(cfg.into());
        self
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stderr = Some(cfg.into());
        self
    pub fn kill_on_drop(&mut self, kill_on_drop: bool) -> &mut Self {
        self.kill_on_drop = kill_on_drop;
        self
    pub async fn spawn(&mut self) -> Result<Child> {
        let mut cmd = std::process::Command::new(&self.program);
        
        if !self.args.is_empty() {
            cmd.args(&self.args);
        for (key, val) in &self.env {
            cmd.env(key, val);
        if let Some(ref dir) = self.current_dir {
            cmd.current_dir(dir);
        if let Some(stdin) = self.stdin.take() {
            cmd.stdin(stdin);
        } else {
            cmd.stdin(Stdio::null());
        if let Some(stdout) = self.stdout.take() {
            cmd.stdout(stdout);
        } else {
            cmd.stdout(Stdio::piped());
        if let Some(stderr) = self.stderr.take() {
            cmd.stderr(stderr);
        } else {
            cmd.stderr(Stdio::piped());
        let child = cmd.spawn()?;
        
        Ok(Child {
        })
    pub async fn output(&mut self) -> Result<Output> {
        let mut child = self.spawn().await?;
        child.wait_with_output().await
    pub async fn status(&mut self) -> Result<std::process::ExitStatus> {
        let mut child = self.spawn().await?;
        child.wait().await
    }
}

/// Async process child
#[derive(Debug)]
pub struct Child {
impl Child {
    pub fn id(&self) -> Option<u32> {
        Some(self.inner.id())
    pub async fn wait(&mut self) -> Result<std::process::ExitStatus> {
        // In a real async implementation, this would be non-blocking
        self.inner.wait()
    pub async fn wait_with_output(self) -> Result<Output> {
        // In a real async implementation, this would be non-blocking
        let output = self.inner.wait_with_output()?;
        Ok(Output {
        })
    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        self.inner.try_wait()
    pub fn kill(&mut self) -> Result<()> {
        self.inner.kill()
    pub fn start_kill(&mut self) -> Result<()> {
        self.kill()
    pub fn stdin(&mut self) -> Option<&mut std::process::ChildStdin> {
        self.inner.stdin.as_mut()
    pub fn stdout(&mut self) -> Option<&mut std::process::ChildStdout> {
        self.inner.stdout.as_mut()
    pub fn stderr(&mut self) -> Option<&mut std::process::ChildStderr> {
        self.inner.stderr.as_mut()
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        if self.kill_on_drop {
            let _ = self.kill();
        }
    }
/// Process output
#[derive(Debug)]
pub struct Output {
impl Output {
    pub fn stdout_as_string(&self) -> Result<String> {
        String::from_utf8(self.stdout.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    pub fn stderr_as_string(&self) -> Result<String> {
        String::from_utf8(self.stderr.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

/// Async process utilities
pub async fn spawn_process<S: AsRef<str>>(program: S) -> Result<Child> {
    Command::new(program.as_ref()).spawn().await
pub async fn run_command<S: AsRef<str>>(program: S, args: &[S]) -> Result<Output> {
    let mut cmd = Command::new(program.as_ref());
    for arg in args {
        cmd.arg(arg.as_ref());
    }
    cmd.output().await
pub async fn run_command_with_timeout<S: AsRef<str>>(
    timeout: Duration
) -> Result<Output> {
    // In a real async implementation, this would use tokio::time::timeout
    let mut cmd = Command::new(program.as_ref());
    for arg in args {
        cmd.arg(arg.as_ref());
    // Simulate timeout behavior
    let start = std::time::Instant::now();
    let output = cmd.output().await?;
    
    if start.elapsed() > timeout {
        return Err(std::io::Error::new(
            "Process execution timed out"
        ));
    Ok(output)
pub async fn check_command_exists<S: AsRef<str>>(command: S) -> bool {
    let result = run_command("which", &[command.as_ref()]).await;
    match result {
    }
}

/// Process management utilities
pub struct ProcessManager {
impl ProcessManager {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub async fn spawn<S: AsRef<str>>(&mut self, program: S) -> Result<u32> {
        let mut child = spawn_process(program).await?;
        let id = child.id().unwrap_or(0);
        self.processes.insert(id, child);
        Ok(id)
    pub async fn wait(&mut self, id: u32) -> Result<std::process::ExitStatus> {
        if let Some(mut child) = self.processes.remove(&id) {
            child.wait().await
        } else {
            Err(std::io::Error::new(
                "Process not found"
            ))
        }
    }
    
    pub fn kill(&mut self, id: u32) -> Result<()> {
        if let Some(child) = self.processes.get_mut(&id) {
            child.kill()
        } else {
            Err(std::io::Error::new(
                "Process not found"
            ))
        }
    }
    
    pub fn kill_all(&mut self) -> Result<()> {
        for child in self.processes.values_mut() {
            child.kill()?;
        }
        self.processes.clear();
        Ok(())
    pub fn process_count(&self) -> usize {
        self.processes.len()
    pub fn list_processes(&self) -> Vec<u32> {
        self.processes.keys().cloned().collect()
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

