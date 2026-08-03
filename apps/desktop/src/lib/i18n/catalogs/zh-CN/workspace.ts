/**
 * Simplified Chinese workspace messages catalog.
 */

export const workspace = {
  // Workspace list
  noWorkspaces: "暂无工作区",
  noWorkspacesDescription: "创建你的第一个工作区，开始整理你的研究。",
  createWorkspace: "创建工作区",
  failedToLoadWorkspaces: "加载工作区失败",
  created: "创建于 {date}",

  // Create workspace dialog
  createWorkspaceTitle: "创建工作区",
  workspaceName: "工作区名称",
  workspaceNamePlaceholder: "我的研究",
  workspaceNameRequired: "工作区名称为必填项",
  workspaceNameTooLong: "工作区名称不能超过 200 个字符",
  failedToCreateWorkspace: "创建工作区失败",
  cancel: "取消",
  creating: "创建中…",
  create: "创建",

  // Today page
  today: "今日",
  todayDescription: "你的投资研究仪表板",
  selectWorkspace: "选择工作区",
  selectWorkspaceDescription: "选择一个工作区来查看和管理你的研究，或创建一个新的工作区开始使用。",
  workspaceSelected: "已选择工作区。研究功能将在第二阶段推出。",
  changeWorkspace: "切换工作区",
} as const;

export type WorkspaceKey = keyof typeof workspace;