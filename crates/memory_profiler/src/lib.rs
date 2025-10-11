use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sysinfo::System;
use tokio::sync::RwLock;
use tokio::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub total_memory: u64,
    pub available_memory: u64,
    pub processes: HashMap<String, ProcessMemoryInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMemoryInfo {
    pub pid: u32,
    pub name: String,
    pub virtual_memory: u64,
    pub resident_memory: u64,
    pub cpu_usage: f32,
    pub threads: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBudget {
    pub cold_start_mb: u64,
    pub idle_tab_mb: u64,
    pub ten_tabs_mb: u64,
    pub email_sync_mb: u64,
    pub vpn_control_mb: u64,
}

impl Default for MemoryBudget {
    fn default() -> Self {
        Self {
            cold_start_mb: 80,
            idle_tab_mb: 120,
            ten_tabs_mb: 450,
            email_sync_mb: 40,
            vpn_control_mb: 10,
        }
    }
}

pub struct MemoryProfiler {
    system: System,
    snapshots: RwLock<Vec<MemorySnapshot>>,
    budget: MemoryBudget,
    process_name: String,
}

impl MemoryProfiler {
    pub fn new(process_name: &str) -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system,
            snapshots: RwLock::new(Vec::new()),
            budget: MemoryBudget::default(),
            process_name: process_name.to_string(),
        }
    }

    pub async fn take_snapshot(&mut self) -> Result<MemorySnapshot> {
        self.system.refresh_all();

        let mut processes = HashMap::new();
        
        for (pid, process) in self.system.processes() {
            let process_info = ProcessMemoryInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                virtual_memory: process.virtual_memory(),
                resident_memory: process.memory(),
                cpu_usage: process.cpu_usage(),
                threads: 1, // sysinfo doesn't expose thread count in this version
            };
            
            processes.insert(format!("{}:{}", process.name(), pid.as_u32()), process_info);
        }

        let snapshot = MemorySnapshot {
            timestamp: chrono::Utc::now(),
            total_memory: self.system.total_memory(),
            available_memory: self.system.available_memory(),
            processes,
        };

        // Store snapshot
        {
            let mut snapshots = self.snapshots.write().await;
            snapshots.push(snapshot.clone());
            
            // Keep only last 100 snapshots
            if snapshots.len() > 100 {
                snapshots.remove(0);
            }
        }

        Ok(snapshot)
    }

    pub async fn get_process_memory(&mut self, process_name: &str) -> Result<Vec<ProcessMemoryInfo>> {
        self.system.refresh_processes();
        
        let mut process_infos = Vec::new();
        
        for (pid, process) in self.system.processes() {
            if process.name().contains(process_name) {
                process_infos.push(ProcessMemoryInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    virtual_memory: process.virtual_memory(),
                    resident_memory: process.memory(),
                    cpu_usage: process.cpu_usage(),
                    threads: 1, // sysinfo doesn't expose thread count in this version
                });
            }
        }

        Ok(process_infos)
    }

    pub async fn monitor_budget(&mut self) -> Result<BudgetReport> {
        let process_name = self.process_name.clone();
        let processes = self.get_process_memory(&process_name).await?;
        
        let total_rss = processes.iter().map(|p| p.resident_memory).sum::<u64>();
        let total_virtual = processes.iter().map(|p| p.virtual_memory).sum::<u64>();

        let mut violations = Vec::new();
        
        if total_rss > self.budget.cold_start_mb * 1024 * 1024 {
            violations.push(BudgetViolation {
                metric: "cold_start".to_string(),
                limit_mb: self.budget.cold_start_mb,
                actual_mb: total_rss / (1024 * 1024),
            });
        }

        if total_rss > self.budget.idle_tab_mb * 1024 * 1024 {
            violations.push(BudgetViolation {
                metric: "idle_tab".to_string(),
                limit_mb: self.budget.idle_tab_mb,
                actual_mb: total_rss / (1024 * 1024),
            });
        }

        Ok(BudgetReport {
            timestamp: chrono::Utc::now(),
            total_rss_mb: total_rss / (1024 * 1024),
            total_virtual_mb: total_virtual / (1024 * 1024),
            process_count: processes.len(),
            violations,
            processes,
        })
    }

    pub async fn start_monitoring(&self, interval: Duration) -> Result<()> {
        let mut profiler = self.clone();
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                if let Err(e) = profiler.take_snapshot().await {
                    eprintln!("Failed to take memory snapshot: {}", e);
                }
            }
        });

        Ok(())
    }

    pub async fn get_snapshots(&self) -> Vec<MemorySnapshot> {
        let snapshots = self.snapshots.read().await;
        snapshots.clone()
    }

    pub async fn export_report(&mut self, path: &str) -> Result<()> {
        let snapshots = self.get_snapshots().await;
        let budget_report = self.monitor_budget().await?;
        
        let report = MemoryReport {
            snapshots,
            budget_report,
            generated_at: chrono::Utc::now(),
        };

        let content = serde_json::to_string_pretty(&report)?;
        tokio::fs::write(path, content).await?;
        
        Ok(())
    }
}

impl Clone for MemoryProfiler {
    fn clone(&self) -> Self {
        Self {
            system: System::new_all(),
            snapshots: RwLock::new(Vec::new()),
            budget: self.budget.clone(),
            process_name: self.process_name.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetViolation {
    pub metric: String,
    pub limit_mb: u64,
    pub actual_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub total_rss_mb: u64,
    pub total_virtual_mb: u64,
    pub process_count: usize,
    pub violations: Vec<BudgetViolation>,
    pub processes: Vec<ProcessMemoryInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryReport {
    pub snapshots: Vec<MemorySnapshot>,
    pub budget_report: BudgetReport,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

// Memory optimization utilities
pub struct MemoryOptimizer {
    profiler: MemoryProfiler,
}

impl MemoryOptimizer {
    pub fn new(process_name: &str) -> Self {
        Self {
            profiler: MemoryProfiler::new(process_name),
        }
    }

    pub async fn suggest_optimizations(&mut self) -> Result<Vec<OptimizationSuggestion>> {
        let budget_report = self.profiler.monitor_budget().await?;
        let mut suggestions = Vec::new();

        if !budget_report.violations.is_empty() {
            suggestions.push(OptimizationSuggestion {
                priority: OptimizationPriority::High,
                category: "Memory Budget".to_string(),
                description: "Process exceeds memory budget limits".to_string(),
                action: "Implement lazy loading and process sharing".to_string(),
            });
        }

        if budget_report.total_virtual_mb > budget_report.total_rss_mb * 3 {
            suggestions.push(OptimizationSuggestion {
                priority: OptimizationPriority::Medium,
                category: "Virtual Memory".to_string(),
                description: "High virtual memory usage detected".to_string(),
                action: "Review memory allocation patterns".to_string(),
            });
        }

        if budget_report.process_count > 10 {
            suggestions.push(OptimizationSuggestion {
                priority: OptimizationPriority::Medium,
                category: "Process Count".to_string(),
                description: "High number of processes detected".to_string(),
                action: "Consider process consolidation".to_string(),
            });
        }

        Ok(suggestions)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub priority: OptimizationPriority,
    pub category: String,
    pub description: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
}
