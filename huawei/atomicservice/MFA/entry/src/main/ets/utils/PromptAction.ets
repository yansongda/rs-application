import { hilog } from '@kit.PerformanceAnalysisKit';
import { LogDomain } from '../App';
import { AppStorageV2, UIContext } from '@kit.ArkUI';
import promptAction from '@ohos.promptAction'

class PromptAction {
  private static uiContext: UIContext | undefined;

  protected static getUIContext(): UIContext {
    if ('undefined' === typeof PromptAction.uiContext) {
      PromptAction.uiContext = AppStorageV2.connect(UIContext)!
    }

    return PromptAction.uiContext;
  }
}

export class Dialog extends PromptAction {
  static async open(builder: CustomBuilder): Promise<number | undefined> {

    try {
      return await PromptAction.getUIContext().getPromptAction().openCustomDialog({ builder, autoCancel: false });
    } catch (e) {
      hilog.error(LogDomain.UTILS, 'utils/dialog', '调用 `openCustomDialog` 失败，调用链路: %{public}s',
        JSON.stringify(e));
    }

    return undefined;
  }

  static close(dialogId: number | undefined) {
    if ('undefined' === typeof dialogId) {
      return;
    }

    try {
      PromptAction.getUIContext().getPromptAction().closeCustomDialog(dialogId);
    } catch (e) {
      hilog.error(LogDomain.UTILS, 'utils/dialog', '调用 `closeCustomDialog` 失败，调用链路: %{public}s',
        JSON.stringify(e));
    }
  }

  static async execute<T>(builder: CustomBuilder, callback: () => Promise<T>): Promise<T> {
    const dialogId = await Dialog.open(builder);

    const result = await callback();

    Dialog.close(dialogId);

    return result;
  }
}

export class Toast extends PromptAction {
  static show(options: promptAction.ShowToastOptions): void {
    try {
      PromptAction.getUIContext().getPromptAction().showToast(options);
    } catch (error) {
    }
  }
}
