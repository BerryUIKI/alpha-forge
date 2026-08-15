/**
 * Simplified Chinese artifacts messages catalog.
 */

export const artifacts = {
  artifactsTitle: "研究产物",
  createCompanyComparison: "创建公司对比",
  createCompanyComparisonDescription:
    "输入两个不同的股票代码和一项已记录指标。AlphaForge 不会获取市场数据或给出投资建议。",
  loadingCompanyComparisonPlugin: "正在加载公司对比插件…",
  failedToLoadCompanyComparisonPlugin: "加载公司对比插件失败。",
  companyComparisonPluginDisabled: "公司对比插件已禁用",
  companyComparisonPluginDisabledDescription:
    "请先在设置中启用内置的 company-comparison 插件，再创建研究产物。",
  manageInternalPlugins: "管理内部插件",
  firstCompanyTicker: "第一个股票代码",
  firstCompanyMetric: "第一个数值",
  secondCompanyTicker: "第二个股票代码",
  secondCompanyMetric: "第二个数值",
  comparisonDimension: "对比维度",
  comparisonDimensionRevenue: "营业收入",
  comparisonDimensionMarketCap: "市值",
  comparisonDimensionPeRatio: "市盈率",
  createAndOpenCompanyComparison: "创建并打开研究产物",
  creatingCompanyComparison: "正在创建…",
  invalidCompanyComparison: "请输入两个不同且有效的股票代码，并为每家公司输入有限数值。",
  failedToCreateCompanyComparison: "无法创建公司对比研究产物。",
  companyComparisonCreatedOpenFailed: "研究产物已创建，但无法打开其隔离窗口。",
  retryOpenArtifact: "重试打开",

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
