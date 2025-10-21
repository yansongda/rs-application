import { UIContext } from "@ohos.arkui.UIContext";
import { hilog } from "@kit.PerformanceAnalysisKit";
import { Domain } from "../Constant";

export function openDialog(uiContext: UIContext, builder: CustomBuilder, callback?: (dialogId: number) => void) {
  uiContext.getPromptAction().openCustomDialog({ builder })
    .then((dialogId) => {
      if (callback) {
        callback(dialogId);
      }
    })
    .catch(() => {
      hilog.error(Domain.UTILS, 'utils/dialog', '调用 `openCustomDialog` 失败')
    })
}

export function closeDialog(uiContext: UIContext, dialogId: number) {
  try {
    uiContext.getPromptAction().closeCustomDialog(dialogId);
  } catch (error) {
  }
}