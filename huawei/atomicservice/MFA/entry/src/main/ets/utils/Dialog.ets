import { UIContext } from '@ohos.arkui.UIContext';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { LogDomain } from '../App';

export class Dialog {
  static async open(uiContext: UIContext, builder: CustomBuilder): Promise<number | undefined> {
    try {
      return await uiContext.getPromptAction().openCustomDialog({ builder, autoCancel: false });
    } catch (e) {
      hilog.error(LogDomain.UTILS, 'utils/dialog', '调用 `openCustomDialog` 失败');
    }

    return undefined;
  }

  static close(uiContext: UIContext, dialogId: number | undefined) {
    if ('undefined' === typeof dialogId) {
      return;
    }

    try {
      uiContext.getPromptAction().closeCustomDialog(dialogId);
    } catch (error) {
    }
  }

  static async execute<T>(uiContext: UIContext, builder: CustomBuilder, callback: () => Promise<T>): Promise<T> {
    const dialogId = await Dialog.open(uiContext, builder);

    const result = await callback();

    Dialog.close(uiContext, dialogId);

    return result;
  }
}
