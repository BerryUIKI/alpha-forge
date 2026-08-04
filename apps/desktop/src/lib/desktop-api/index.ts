// Unified desktop API layer.
// All IPC calls go through this module. Components must not call invoke() directly.

import * as agentApi from "./agent";
import * as artifactsApi from "./artifacts";
import * as researchApi from "./research";
import * as journalApi from "./journal";
import * as portfolioApi from "./portfolio";
import * as pluginsApi from "./plugins";
import * as settingsApi from "./settings";
import * as workspaceApi from "./workspace";
import * as thesisApi from "./thesis";
import * as knowledgeGraphApi from "./knowledge-graph";
import * as systemApi from "./system";
import * as optionsApi from "./options";
import * as gooseApi from "./goose";

export const desktopApi = {
  agent: agentApi,
  artifacts: artifactsApi,
  research: researchApi,
  journal: journalApi,
  portfolio: portfolioApi,
  plugins: pluginsApi,
  settings: settingsApi,
  workspace: workspaceApi,
  thesis: thesisApi,
  knowledgeGraph: knowledgeGraphApi,
  system: systemApi,
  options: optionsApi,
  goose: gooseApi,
} as const;

export type DesktopApi = typeof desktopApi;
