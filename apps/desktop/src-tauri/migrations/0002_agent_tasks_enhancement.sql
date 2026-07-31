-- Phase 2: Enhance agent_tasks table with workspace relationship
-- and additional fields for better task management.

-- Add workspace_id column
ALTER TABLE agent_tasks ADD COLUMN workspace_id TEXT REFERENCES workspaces(id);

-- Add title and description columns
ALTER TABLE agent_tasks ADD COLUMN title TEXT;
ALTER TABLE agent_tasks ADD COLUMN description TEXT;

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_agent_tasks_workspace_id ON agent_tasks(workspace_id);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_status ON agent_tasks(status);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_created_at ON agent_tasks(created_at);

CREATE INDEX IF NOT EXISTS idx_agent_task_events_task_id ON agent_task_events(task_id);
CREATE INDEX IF NOT EXISTS idx_agent_task_events_created_at ON agent_task_events(created_at);