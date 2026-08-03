/**
 * Simplified Chinese artifacts messages catalog.
 */

export const artifacts = {
  artifactsTitle: "研究产物",

  // Artifact viewer states
  loadingArtifact: "正在加载研究产物…",
  errorLoadingArtifact: "加载研究产物失败",
  artifactNotFound: "未找到研究产物",
  artifactNotFoundDescription: "无法找到请求的研究产物。它可能已被删除或ID不正确。",
  noRendererAvailable: "无可用渲染器",
  noRendererAvailableDescription: "研究产物类型 {type} 没有可用的渲染器",
  artifactStatus: "状态",
  artifactCreated: "创建时间",
} as const;

export type ArtifactsKey = keyof typeof artifacts;