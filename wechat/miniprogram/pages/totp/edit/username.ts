import api from "@api/totp";
import error from "@utils/error";
import type { EditUsernameRequest } from "miniprogram/types/totp";
import type { FormSubmit } from "miniprogram/types/wechat";
import Message from "tdesign-miniprogram/message/index";
import Toast from "tdesign-miniprogram/toast/index";

interface Query {
  id?: string;
  username?: string;
}

interface FormData {
  username: string;
}

Page({
  data: {
    id: 0,
    username: "",
  },
  onLoad(query: Query) {
    this.setData({
      id: Number(query.id || 0),
      username: query.username || "",
    });
  },
  async submit(e: FormSubmit<FormData>) {
    Toast({
      message: "更新中...",
      theme: "loading",
      duration: 5000,
      direction: "column",
      preventScrollThrough: true,
    });

    try {
      await api.editUsername({
        id: this.data.id,
        username: e.detail.value.username,
      } as EditUsernameRequest);

      Toast({
        message: "修改成功",
        theme: "success",
        duration: 1500,
        direction: "column",
        preventScrollThrough: true,
      });

      setTimeout(() => wx.navigateBack(), 1500);
    } catch (e: unknown) {
      Toast({
        message: "更新失败",
        theme: "error",
        duration: 100,
        direction: "column",
      });

      Message.error({
        content: `更新失败：${error.getErrorMessage(e)}`,
        duration: 5000,
        context: this,
        offset: [20, 32],
      });
    }
  },
  async cancel() {
    await wx.navigateBack();
  },
});
