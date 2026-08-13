/**
 * Simplified Chinese agent messages catalog.
 */

export const agent = {
  // Task status badges
  taskStatusCreated: "已创建",
  taskStatusQueued: "排队中",
  taskStatusRunning: "运行中",
  taskStatusWaiting: "等待中",
  taskStatusCompleted: "已完成",
  taskStatusFailed: "已失败",
  taskStatusCancelled: "已取消",

  // Create agent task
  newTask: "新建任务",
  createAgentTask: "创建代理任务",
  titleLabel: "标题",
  titlePlaceholder: "研究特斯拉第四季度财报",
  descriptionLabel: "描述（可选）",
  descriptionPlaceholder: "提供更多关于您想研究的内容的细节...",
  taskTitleRequired: "任务标题为必填项",
  failedToCreateTask: "创建任务失败",
  cancel: "取消",
  creating: "创建中...",
  createTask: "创建任务",

  // Agent task list
  failedToLoadAgentTasks: "加载代理任务失败",
  noTasksYet: "暂无任务",
  noTasksDescription: "创建您的第一个代理任务以开始研究。",

  // Task execution
  startTask: "开始",
  retryStartTask: "重试启动",
  cancelTask: "取消",
  startingTask: "启动中...",
  cancellingTask: "取消中...",
  taskStartFailed: "任务启动失败，已保留在队列中；准备好后可以重试。",
  taskQueueFailed: "任务排队失败，请重试。",
} as const;

export type AgentKey = keyof typeof agent;
