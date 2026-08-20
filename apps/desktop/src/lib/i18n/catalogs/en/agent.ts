/**
 * English agent messages catalog.
 */

export const agent = {
  // Task status badges
  taskStatusCreated: "Created",
  taskStatusQueued: "Queued",
  taskStatusRunning: "Running",
  taskStatusWaiting: "Waiting",
  taskStatusCompleted: "Completed",
  taskStatusFailed: "Failed",
  taskStatusCancelled: "Cancelled",

  // Create agent task
  newTask: "New Task",
  createAgentTask: "Create Agent Task",
  titleLabel: "Title",
  titlePlaceholder: "Research Tesla's Q4 earnings",
  descriptionLabel: "Description (optional)",
  descriptionPlaceholder: "Provide more details about what you want to research...",
  taskTitleRequired: "Task title is required",
  failedToCreateTask: "Failed to create task",
  cancel: "Cancel",
  creating: "Creating...",
  createTask: "Create Task",

  // Agent task list
  failedToLoadAgentTasks: "Failed to load agent tasks",
  noTasksYet: "No tasks yet",
  noTasksDescription: "Create your first agent task to start researching.",

  // Task execution
  startTask: "Start",
  retryStartTask: "Retry Start",
  cancelTask: "Cancel",
  startingTask: "Starting...",
  cancellingTask: "Cancelling...",
  taskStartFailed: "Unable to start this task. It remains queued; retry when ready.",
  taskQueueFailed: "Unable to queue this task. Please try again.",

  // Task failure & progress
  taskFailureReason: "Failure reason",
  taskCompleted: "Research completed",
  taskProgressPrefix: "Agent",
  taskCancelledMessage: "Task was cancelled",

  // Structured research results
  researchSummary: "Summary",
  researchClaims: "Key Claims",
  researchEvidence: "Evidence",
  researchRisks: "Risks",
  researchConfidence: "Confidence",
  noResultsAvailable: "No structured results available",
  viewResearchResults: "View Results",

  // App readiness
  appInitializing: "Initializing...",
  appInitFailed: "Application failed to initialize",
} as const;

export type AgentKey = keyof typeof agent;
