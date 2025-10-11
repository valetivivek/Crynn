use anyhow::Result;
use crynn_memory_profiler::{MemoryProfiler, MemoryBudget, OptimizationSuggestion};

pub struct MemoryManager {
    profiler: MemoryProfiler,
    budget: MemoryBudget,
}

impl MemoryManager {
    pub fn new(process_name: &str) -> Self {
        Self {
            profiler: MemoryProfiler::new(process_name),
            budget: MemoryBudget::default(),
        }
    }

    pub async fn start_monitoring(&mut self) -> Result<()> {
        // Start background memory monitoring
        self.profiler.start_monitoring(std::time::Duration::from_secs(5)).await?;
        Ok(())
    }

    pub async fn get_current_usage(&self) -> Result<usize> {
        let snapshot = self.profiler.take_snapshot().await?;
        
        // Find our process in the snapshot
        let process_name = "crynn-shell"; // TODO: Get actual process name
        let processes = self.profiler.get_process_memory(process_name).await?;
        
        let total_rss: usize = processes.iter().map(|p| p.resident_memory as usize).sum();
        Ok(total_rss)
    }

    pub async fn get_budget_report(&self) -> Result<crynn_memory_profiler::BudgetReport> {
        self.profiler.monitor_budget().await
    }

    pub async fn get_optimization_suggestions(&self) -> Result<Vec<OptimizationSuggestion>> {
        let optimizer = crynn_memory_profiler::MemoryOptimizer::new("crynn-shell");
        optimizer.suggest_optimizations().await
    }

    pub async fn export_report(&self, path: &str) -> Result<()> {
        self.profiler.export_report(path).await
    }

    pub async fn check_budget_compliance(&self) -> Result<BudgetCompliance> {
        let report = self.get_budget_report().await?;
        
        let mut violations = Vec::new();
        
        if report.total_rss_mb > self.budget.cold_start_mb {
            violations.push(BudgetViolation {
                metric: "cold_start".to_string(),
                limit_mb: self.budget.cold_start_mb,
                actual_mb: report.total_rss_mb,
                severity: ViolationSeverity::High,
            });
        }

        if report.total_rss_mb > self.budget.idle_tab_mb {
            violations.push(BudgetViolation {
                metric: "idle_tab".to_string(),
                limit_mb: self.budget.idle_tab_mb,
                actual_mb: report.total_rss_mb,
                severity: ViolationSeverity::Medium,
            });
        }

        if report.total_rss_mb > self.budget.ten_tabs_mb {
            violations.push(BudgetViolation {
                metric: "ten_tabs".to_string(),
                limit_mb: self.budget.ten_tabs_mb,
                actual_mb: report.total_rss_mb,
                severity: ViolationSeverity::Critical,
            });
        }

        Ok(BudgetCompliance {
            is_compliant: violations.is_empty(),
            violations,
            current_usage_mb: report.total_rss_mb,
        })
    }

    pub fn get_budget(&self) -> &MemoryBudget {
        &self.budget
    }
}

#[derive(Debug, Clone)]
pub struct BudgetCompliance {
    pub is_compliant: bool,
    pub violations: Vec<BudgetViolation>,
    pub current_usage_mb: u64,
}

#[derive(Debug, Clone)]
pub struct BudgetViolation {
    pub metric: String,
    pub limit_mb: u64,
    pub actual_mb: u64,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}
