// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The root transaction wanted to commit, but transaction aborted
pub const E_ABORTED : HResultError = HResultError::from_constant(0x8004E002); // CONTEXT_E_ABORTED

/// You made a method call on a COM+ component that has a transaction that has already aborted or in the process of aborting.
pub const E_ABORTING : HResultError = HResultError::from_constant(0x8004E003); // CONTEXT_E_ABORTING

/// There is no MTS object context
pub const E_NOCONTEXT : HResultError = HResultError::from_constant(0x8004E004); // CONTEXT_E_NOCONTEXT

/// The component is configured to use synchronization and this method call would cause a deadlock to occur.
pub const E_WOULD_DEADLOCK : HResultError = HResultError::from_constant(0x8004E005); // CONTEXT_E_WOULD_DEADLOCK

/// The component is configured to use synchronization and a thread has timed out waiting to enter the context.
pub const E_SYNCH_TIMEOUT : HResultError = HResultError::from_constant(0x8004E006); // CONTEXT_E_SYNCH_TIMEOUT

/// You made a method call on a COM+ component that has a transaction that has already committed or aborted.
pub const E_OLDREF : HResultError = HResultError::from_constant(0x8004E007); // CONTEXT_E_OLDREF

/// The specified role was not configured for the application
pub const E_ROLENOTFOUND : HResultError = HResultError::from_constant(0x8004E00C); // CONTEXT_E_ROLENOTFOUND

/// COM+ was unable to talk to the Microsoft Distributed Transaction Coordinator
pub const E_TMNOTAVAILABLE : HResultError = HResultError::from_constant(0x8004E00F); // CONTEXT_E_TMNOTAVAILABLE

/// The requested operation requires that JIT be in the current context and it is not
pub const E_NOJIT : HResultError = HResultError::from_constant(0x8004E026); // CONTEXT_E_NOJIT

/// The requested operation requires that the current context have a Transaction, and it does not
pub const E_NOTRANSACTION : HResultError = HResultError::from_constant(0x8004E027); // CONTEXT_E_NOTRANSACTION
