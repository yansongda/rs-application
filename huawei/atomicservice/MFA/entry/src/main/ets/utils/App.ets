import { bundleManager } from "@kit.AbilityKit";

export const getClientId = async (): Promise<String> => {
  try {
    const info = (await bundleManager.getBundleInfoForSelf(bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_METADATA));
    const metadata: bundleManager.ModuleMetadata[] = info.appInfo.metadataArray;
    info.name

    if (metadata.length > 0) {
      const clientIdObj = metadata.find(item => item. === "client_id");
      return clientIdObj?.value ?? ""; // 返回client_id或空字符串
    }
    return "";
  } catch (error) {
    console.error("获取client_id失败: " + error);
    return "";
  }
}