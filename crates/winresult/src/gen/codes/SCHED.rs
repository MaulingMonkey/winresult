// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The Task Scheduler service must be configured to run in the System account to function properly. Individual tasks may be configured to run in other accounts.
pub const E_SERVICE_NOT_LOCALSYSTEM : ErrorCode = ErrorCode::from_constant(6200); // SCHED_E_SERVICE_NOT_LOCALSYSTEM

/// The task is ready to run at its next scheduled time.
pub const S_TASK_READY : HResultSuccess = HResultSuccess::from_constant(0x00041300); // SCHED_S_TASK_READY

/// The task is currently running.
pub const S_TASK_RUNNING : HResultSuccess = HResultSuccess::from_constant(0x00041301); // SCHED_S_TASK_RUNNING

/// The task will not run at the scheduled times because it has been disabled.
pub const S_TASK_DISABLED : HResultSuccess = HResultSuccess::from_constant(0x00041302); // SCHED_S_TASK_DISABLED

/// The task has not yet run.
pub const S_TASK_HAS_NOT_RUN : HResultSuccess = HResultSuccess::from_constant(0x00041303); // SCHED_S_TASK_HAS_NOT_RUN

/// There are no more runs scheduled for this task.
pub const S_TASK_NO_MORE_RUNS : HResultSuccess = HResultSuccess::from_constant(0x00041304); // SCHED_S_TASK_NO_MORE_RUNS

/// One or more of the properties that are needed to run this task on a schedule have not been set.
pub const S_TASK_NOT_SCHEDULED : HResultSuccess = HResultSuccess::from_constant(0x00041305); // SCHED_S_TASK_NOT_SCHEDULED

/// The last run of the task was terminated by the user.
pub const S_TASK_TERMINATED : HResultSuccess = HResultSuccess::from_constant(0x00041306); // SCHED_S_TASK_TERMINATED

/// Either the task has no triggers or the existing triggers are disabled or not set.
pub const S_TASK_NO_VALID_TRIGGERS : HResultSuccess = HResultSuccess::from_constant(0x00041307); // SCHED_S_TASK_NO_VALID_TRIGGERS

/// Event triggers don't have set run times.
pub const S_EVENT_TRIGGER : HResultSuccess = HResultSuccess::from_constant(0x00041308); // SCHED_S_EVENT_TRIGGER

/// Trigger not found.
pub const E_TRIGGER_NOT_FOUND : HResultError = HResultError::from_constant(0x80041309); // SCHED_E_TRIGGER_NOT_FOUND

/// One or more of the properties that are needed to run this task have not been set.
pub const E_TASK_NOT_READY : HResultError = HResultError::from_constant(0x8004130A); // SCHED_E_TASK_NOT_READY

/// There is no running instance of the task.
pub const E_TASK_NOT_RUNNING : HResultError = HResultError::from_constant(0x8004130B); // SCHED_E_TASK_NOT_RUNNING

/// The Task Scheduler Service is not installed on this computer.
pub const E_SERVICE_NOT_INSTALLED : HResultError = HResultError::from_constant(0x8004130C); // SCHED_E_SERVICE_NOT_INSTALLED

/// The task object could not be opened.
pub const E_CANNOT_OPEN_TASK : HResultError = HResultError::from_constant(0x8004130D); // SCHED_E_CANNOT_OPEN_TASK

/// The object is either an invalid task object or is not a task object.
pub const E_INVALID_TASK : HResultError = HResultError::from_constant(0x8004130E); // SCHED_E_INVALID_TASK

/// No account information could be found in the Task Scheduler security database for the task indicated.
pub const E_ACCOUNT_INFORMATION_NOT_SET : HResultError = HResultError::from_constant(0x8004130F); // SCHED_E_ACCOUNT_INFORMATION_NOT_SET

/// Unable to establish existence of the account specified.
pub const E_ACCOUNT_NAME_NOT_FOUND : HResultError = HResultError::from_constant(0x80041310); // SCHED_E_ACCOUNT_NAME_NOT_FOUND

/// Corruption was detected in the Task Scheduler security database; the database has been reset.
pub const E_ACCOUNT_DBASE_CORRUPT : HResultError = HResultError::from_constant(0x80041311); // SCHED_E_ACCOUNT_DBASE_CORRUPT

/// Task Scheduler security services are available only on Windows NT.
pub const E_NO_SECURITY_SERVICES : HResultError = HResultError::from_constant(0x80041312); // SCHED_E_NO_SECURITY_SERVICES

/// The task object version is either unsupported or invalid.
pub const E_UNKNOWN_OBJECT_VERSION : HResultError = HResultError::from_constant(0x80041313); // SCHED_E_UNKNOWN_OBJECT_VERSION

/// The task has been configured with an unsupported combination of account settings and run time options.
pub const E_UNSUPPORTED_ACCOUNT_OPTION : HResultError = HResultError::from_constant(0x80041314); // SCHED_E_UNSUPPORTED_ACCOUNT_OPTION

/// The Task Scheduler Service is not running.
pub const E_SERVICE_NOT_RUNNING : HResultError = HResultError::from_constant(0x80041315); // SCHED_E_SERVICE_NOT_RUNNING

/// The task XML contains an unexpected node.
pub const E_UNEXPECTEDNODE : HResultError = HResultError::from_constant(0x80041316); // SCHED_E_UNEXPECTEDNODE

/// The task XML contains an element or attribute from an unexpected namespace.
pub const E_NAMESPACE : HResultError = HResultError::from_constant(0x80041317); // SCHED_E_NAMESPACE

/// The task XML contains a value which is incorrectly formatted or out of range.
pub const E_INVALIDVALUE : HResultError = HResultError::from_constant(0x80041318); // SCHED_E_INVALIDVALUE

/// The task XML is missing a required element or attribute.
pub const E_MISSINGNODE : HResultError = HResultError::from_constant(0x80041319); // SCHED_E_MISSINGNODE

/// The task XML is malformed.
pub const E_MALFORMEDXML : HResultError = HResultError::from_constant(0x8004131A); // SCHED_E_MALFORMEDXML

/// The task is registered, but not all specified triggers will start the task, check task scheduler event log for detailed information.
pub const S_SOME_TRIGGERS_FAILED : HResultSuccess = HResultSuccess::from_constant(0x0004131B); // SCHED_S_SOME_TRIGGERS_FAILED

/// The task is registered, but may fail to start. Batch logon privilege needs to be enabled for the task principal.
pub const S_BATCH_LOGON_PROBLEM : HResultSuccess = HResultSuccess::from_constant(0x0004131C); // SCHED_S_BATCH_LOGON_PROBLEM

/// The task XML contains too many nodes of the same type.
pub const E_TOO_MANY_NODES : HResultError = HResultError::from_constant(0x8004131D); // SCHED_E_TOO_MANY_NODES

/// The task cannot be started after the trigger's end boundary.
pub const E_PAST_END_BOUNDARY : HResultError = HResultError::from_constant(0x8004131E); // SCHED_E_PAST_END_BOUNDARY

/// An instance of this task is already running.
pub const E_ALREADY_RUNNING : HResultError = HResultError::from_constant(0x8004131F); // SCHED_E_ALREADY_RUNNING

/// The task will not run because the user is not logged on.
pub const E_USER_NOT_LOGGED_ON : HResultError = HResultError::from_constant(0x80041320); // SCHED_E_USER_NOT_LOGGED_ON

/// The task image is corrupt or has been tampered with.
pub const E_INVALID_TASK_HASH : HResultError = HResultError::from_constant(0x80041321); // SCHED_E_INVALID_TASK_HASH

/// The Task Scheduler service is not available.
pub const E_SERVICE_NOT_AVAILABLE : HResultError = HResultError::from_constant(0x80041322); // SCHED_E_SERVICE_NOT_AVAILABLE

/// The Task Scheduler service is too busy to handle your request. Please try again later.
pub const E_SERVICE_TOO_BUSY : HResultError = HResultError::from_constant(0x80041323); // SCHED_E_SERVICE_TOO_BUSY

/// The Task Scheduler service attempted to run the task, but the task did not run due to one of the constraints in the task definition.
pub const E_TASK_ATTEMPTED : HResultError = HResultError::from_constant(0x80041324); // SCHED_E_TASK_ATTEMPTED

/// The Task Scheduler service has asked the task to run.
pub const S_TASK_QUEUED : HResultSuccess = HResultSuccess::from_constant(0x00041325); // SCHED_S_TASK_QUEUED

/// The task is disabled.
pub const E_TASK_DISABLED : HResultError = HResultError::from_constant(0x80041326); // SCHED_E_TASK_DISABLED

/// The task has properties that are not compatible with previous versions of Windows.
pub const E_TASK_NOT_V1_COMPAT : HResultError = HResultError::from_constant(0x80041327); // SCHED_E_TASK_NOT_V1_COMPAT

/// The task settings do not allow the task to start on demand.
pub const E_START_ON_DEMAND : HResultError = HResultError::from_constant(0x80041328); // SCHED_E_START_ON_DEMAND

/// The combination of properties that task is using is not compatible with the scheduling engine.
pub const E_TASK_NOT_UBPM_COMPAT : HResultError = HResultError::from_constant(0x80041329); // SCHED_E_TASK_NOT_UBPM_COMPAT

/// The task definition uses a deprecated feature.
pub const E_DEPRECATED_FEATURE_USED : HResultError = HResultError::from_constant(0x80041330); // SCHED_E_DEPRECATED_FEATURE_USED
