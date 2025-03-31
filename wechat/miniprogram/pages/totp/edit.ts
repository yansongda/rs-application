import api from "@api/totp";
import type { HttpError } from "@models/error";
import Message from "tdesign-miniprogram/message/index";
import Toast from "tdesign-miniprogram/toast/index";
import type { UpdateRequest } from "types/totp";
import type { FormSubmit } from "types/wechat";

interface Query {
  id?: string;
}

interface FormData {
  issuer: string;
  username: string;
}

Page({
  data: {
    dialogVisible: false,
    dialogConfirmBtn: { content: "重试", variant: "base" },
    id: 0,
    issuer: "",
    username: "",
  },
  onLoad(query: Query) {
    this.data.id = Number(query.id || 0);
  },
  onShow() {
    Toast({
      message: "加载中...",
      theme: "loading",
      duration: 5000,
      direction: "column",
    });

    api
      .detail(this.data.id)
      .then(({ id, issuer, username }) => {
        Toast({
          message: "加载成功",
          theme: "success",
          duration: 100,
          direction: "column",
        });

        this.setData({ id, issuer: issuer ?? "", username: username ?? "" });
      })
      .catch(() => {
        Toast({
          message: "加载失败",
          theme: "error",
          duration: 100,
          direction: "column",
        });

        this.setData({ dialogVisible: true });
      });
  },
  submit(e: FormSubmit<FormData>) {
    Toast({
      message: "更新中...",
      theme: "loading",
      duration: 5000,
      direction: "column",
      preventScrollThrough: true,
    });

    api
      .update({ id: this.data.id, ...e.detail.value } as UpdateRequest)
      .then(() => {
        Toast({
          message: "更新成功",
          theme: "success",
          duration: 1500,
          direction: "column",
          preventScrollThrough: true,
        });

        setTimeout(() => wx.navigateBack(), 1500);
      })
      .catch((e: HttpError) => {
        Toast({
          message: "更新失败",
          theme: "error",
          duration: 100,
          direction: "column",
        });

        Message.error({
          content: `更新失败：${e.message}`,
          duration: 5000,
          context: this,
          offset: [20, 32],
        });
      });
  },
  async cancel() {
    await wx.navigateBack();
  },
  dialogConfirm() {
    this.setData({ dialogVisible: false });

    this.onShow();
  },
  dialogCancel() {
    this.setData({ dialogVisible: false });
  },
});
