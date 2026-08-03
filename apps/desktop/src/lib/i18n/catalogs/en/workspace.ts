/**
 * English workspace messages catalog.
 */

export const workspace = {
  // Workspace list
  noWorkspaces: "No workspaces yet",
  noWorkspacesDescription: "Create your first workspace to start organizing your research.",
  createWorkspace: "Create Workspace",
  failedToLoadWorkspaces: "Failed to load workspaces",
  created: "Created {date}",

  // Create workspace dialog
  createWorkspaceTitle: "Create Workspace",
  workspaceName: "Workspace Name",
  workspaceNamePlaceholder: "My Research",
  workspaceNameRequired: "Workspace name is required",
  workspaceNameTooLong: "Workspace name must be 200 characters or less",
  failedToCreateWorkspace: "Failed to create workspace",
  cancel: "Cancel",
  creating: "Creating…",
  create: "Create",

  // Today page
  today: "Today",
  todayDescription: "Your investment research dashboard",
  selectWorkspace: "Select a Workspace",
  selectWorkspaceDescription:
    "Choose a workspace to view and manage your research, or create a new one to get started.",
  workspaceSelected: "Workspace selected. Research features coming in Phase 2+.",
  changeWorkspace: "Change workspace",
} as const;

export type WorkspaceKey = keyof typeof workspace;