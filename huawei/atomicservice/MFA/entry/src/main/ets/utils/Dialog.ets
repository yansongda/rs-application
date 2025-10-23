import { UIContext } from "@ohos.arkui.UIContext";
import { hilog } from "@kit.PerformanceAnalysisKit";
import { Domain } from "../Constant";

export const openDialog = async <T>(uiContext: UIContext, builder: CustomBuilder, callback?: (dialogId: number) => Promise<T | undefined>): Promise<T | undefined> => {
  try {
    const dialogId = await uiContext.getPromptAction().openCustomDialog({ builder });

    if (callback) {
      return callback(dialogId);
    }
  } catch (e) {
    hilog.error(Domain.UTILS, 'utils/dialog', '调用 `openCustomDialog` 失败');
  }

  return undefined;
}

export const closeDialog = (uiContext: UIContext, dialogId: number) => {
  try {
    uiContext.getPromptAction().closeCustomDialog(dialogId);
  } catch (error) {
  }
}