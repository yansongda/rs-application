import { UIContext } from "@ohos.arkui.UIContext";
import { hilog } from "@kit.PerformanceAnalysisKit";
import { Domain } from "../Constant";

export function openDialog<T>(uiContext: UIContext, builder: CustomBuilder, callback?: (dialogId: number) => T | undefined): T | undefined {
  uiContext.getPromptAction().openCustomDialog({ builder })
    .then((dialogId) => {
      if (callback) {
        return callback(dialogId);
      }

      return;
    })
    .catch(() => {
      hilog.error(Domain.UTILS, 'utils/dialog', '调用 `openCustomDialog` 失败')
    })

  return;
}

export function closeDialog(uiContext: UIContext, dialogId: number) {
  try {
    uiContext.getPromptAction().closeCustomDialog(dialogId);
  } catch (error) {
  }
}