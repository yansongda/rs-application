import { cryptoFramework } from '@kit.CryptoArchitectureKit';
import { buffer } from '@kit.ArkTS';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { LogDomain } from '../App';

export class Crypto {
  static md5(data: string): string {
    try {
      let md = cryptoFramework.createMd('MD5');
      md.updateSync({ data: new Uint8Array(buffer.from(data, 'utf-8').buffer) });

      return md.digestSync().data.toString();
    } catch (e) {
      hilog.error(LogDomain.UTILS, 'utils/crypto', 'MD5 计算失败：%{public}s', e.message)
    }

    return data.slice(0, 32);
  }
}