import { hilog } from '@kit.PerformanceAnalysisKit';
import { LogDomain } from '../App';
import { AppStorageV2, UIContext } from '@kit.ArkUI';

export class Dialog {
  private static uiContext = AppStorageV2.connect(UIContext)!;

  static async open(builder: CustomBuilder): Promise<number | undefined> {
    try {
      return await Dialog.uiContext.getPromptAction().openCustomDialog({ builder, autoCancel: false });
    } catch (e) {
      hilog.error(LogDomain.UTILS, 'utils/dialog', '调用 `openCustomDialog` 失败');
    }

    return undefined;
  }

  static close(dialogId: number | undefined) {
    if ('undefined' === typeof dialogId) {
      return;
    }

    try {
      Dialog.uiContext.getPromptAction().closeCustomDialog(dialogId);
    } catch (error) {
    }
  }

  static async execute<T>(builder: CustomBuilder, callback: () => Promise<T>): Promise<T> {
    const dialogId = await Dialog.open(builder);

    const result = await callback();

    Dialog.close(dialogId);

    return result;
  }
}
