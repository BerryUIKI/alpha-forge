// Unified desktop API layer.
// All IPC calls go through this module. Components must not call invoke() directly.

import * as agentApi from "./agent";
import * as artifactsApi from "./artifacts";
import * as researchApi from "./research";
import * as journalApi from "./journal";
import * as portfolioApi from "./portfolio";
import * as settingsApi from "./settings";
import * as workspaceApi from "./workspace";

export const desktopApi = {
  agent: agentApi,
  artifacts: artifactsApi,
  research: researchApi,
  journal: journalApi,
  portfolio: portfolioApi,
  settings: settingsApi,
  workspace: workspaceApi,
} as const;

export type DesktopApi = typeof desktopApi;
