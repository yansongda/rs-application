import { bundleManager } from '@kit.AbilityKit';
import { hilog } from '@kit.PerformanceAnalysisKit';

export enum LogDomain {
  ABILITY = 0,

  UTILS = 1,

  MODELS = 2,

  PAGES = 3,
}

export enum HttpDomain {
  TEST = 'https://application.test.yanda.net.cn',

  PROD = 'https://application.yanda.net.cn',
}

export enum AppEnv {
  TEST,

  PROD,
}

export enum PageState {
  HIDE = 0,

  SHOW = 1,
}

export class App {
  private static clientId: string | undefined;
  private static env: AppEnv | undefined;

  static getMetadata(name: string, module?: string): string | undefined {
    if (typeof module === 'undefined') {
      module = 'entry';
    }

    const bundleInfo: bundleManager.BundleInfo = bundleManager.getBundleInfoForSelfSync(
      bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_APPLICATION |
      bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_METADATA
    );

    hilog.info(LogDomain.UTILS, 'utils/app', 'metadata 获取信息: %{public}o', bundleInfo.appInfo.metadataArray);

    return bundleInfo.appInfo.metadataArray
      .find(item => item.moduleName === module)?.metadata
      .find(metadata => metadata.name === name)?.value;
  }

  static getBundleInfo(): bundleManager.BundleInfo {
    const bundleInfo: bundleManager.BundleInfo = bundleManager.getBundleInfoForSelfSync(
      bundleManager.BundleFlag.GET_BUNDLE_INFO_DEFAULT
    );

    hilog.info(LogDomain.UTILS, 'utils/app', 'BundleInfo 获取信息: %{public}o', bundleInfo);

    return bundleInfo;
  }

  static getClientId(): string {
    if ('undefined' !== typeof App.clientId) {
      return App.clientId;
    }

    const clientId = App.getMetadata('client_id');

    if ('undefined' === typeof clientId) {
      return 'undefined';
    }

    App.clientId = clientId

    return clientId;
  }

  static getEnv(): AppEnv {
    if ('undefined' !== typeof App.env) {
      return App.env;
    }

    const bundleInfo = App.getBundleInfo();

    App.env = AppEnv.PROD;

    if (bundleInfo.versionName.toLowerCase().includes('-beta')) {
      App.env = AppEnv.TEST;
    }

    return App.env;
  }
}
