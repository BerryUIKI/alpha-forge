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
  artifactWindowInvalidRoute: "研究产物窗口地址无效",
  artifactWindowInvalidRouteDescription: "此窗口地址中的研究产物 ID 或类型无效。",
  artifactWindowMismatch: "研究产物地址不匹配",
  artifactWindowMismatchDescription: "请求的研究产物与窗口地址中的类型不匹配。",
  artifactWindowNoData: "研究产物暂无可渲染数据",
  artifactWindowNoDataDescription: "此研究产物尚未生成输入或输出数据。",
  closeArtifactWindow: "关闭研究产物窗口",
  artifactWindowCloseFailed: "无法关闭研究产物窗口",
  noRendererAvailable: "无可用渲染器",
  noRendererAvailableDescription: "研究产物类型 {type} 没有可用的渲染器",
  artifactStatus: "状态",
  artifactCreated: "创建时间",
} as const;

export type ArtifactsKey = keyof typeof artifacts;
