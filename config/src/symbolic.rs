//! Configuration for symbolic testing

use serde::{Deserialize, Serialize};

/// Contains for invariant testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicConfig {
    /// The flag indicating whether to assume that default storage values are symbolic
    pub symbolic_storage: bool,
    /// The flag indicating whether to perform geth-based concrete counterexample validation
    pub concrete_validation: bool,
    /// The SMT solver to be used during symbolic analysis {0: z3, 1: boolector, 2: yices2}
    pub solver: u8,
    /// The timeout (ms) for the solver
    pub solver_timeout: u32,
    /// The number of loops to be unrolled in a single execution
    pub loop_bound: u32,
    /// The number of calls symbolically analyzed in a sequence
    pub call_bound: u32,
}

impl Default for SymbolicConfig {
    fn default() -> Self {
        SymbolicConfig {
            symbolic_storage: false,
            concrete_validation: true,
            solver: 2, // yices2
            solver_timeout: 10_000,
            loop_bound: 5,
            call_bound: 1, // symbolically executing tests
        }
    }
}
