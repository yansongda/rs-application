import api from "@api/totp";
import { CODE } from "@constant/error";
import type { HttpError } from "@models/error";
import { WeixinError } from "@models/error";
import { substr } from "@utils/string";
import Message from "tdesign-miniprogram/message/index";
import Toast from "tdesign-miniprogram/toast/index";
import type {
  Item,
  ItemDeleteEvent,
  ItemDetailEvent,
  ItemMessageEvent,
} from "types/totp";

Page({
  data: {
    dialogVisible: false,
    currentItemId: 0,
    items: [] as Item[],
  },
  onShow() {
    this.all();
  },
  onUnload() {
    // this.clearRefreshInterval();
  },
  all() {
    Toast({
      message: "加载中...",
      theme: "loading",
      duration: 5000,
      direction: "column",
      preventScrollThrough: true,
    });

    api
      .all()
      .then((response) => {
        Toast({
          message: "加载成功",
          theme: "success",
          duration: 100,
          direction: "column",
        });

        this.setData({
          items: response.map((item) => ({
            ...item,
            issuer: substr(item.issuer, 7),
            username: substr(item.username, 50),
          })),
        });
      })
      .catch((e: HttpError) => {
        Toast({
          message: "加载失败",
          theme: "error",
          duration: 100,
          direction: "column",
        });

        Message.error({
          content: `加载失败：${e.message}`,
          duration: 5000,
          offset: [20, 32],
          context: this,
        });
      });
  },
  async create() {
    const scan = await wx.scanCode({ scanType: ["qrCode"] }).catch(() => {
      throw new WeixinError(CODE.WEIXIN_QR_CODE);
    });

    api
      .create(scan.result)
      .catch((e: HttpError) =>
        Message.error({
          content: e.message,
          duration: 5000,
          offset: [20, 32],
          context: this,
        }),
      )
      .finally(() => this.all());
  },
  async itemDetail(e: ItemDetailEvent) {
    const id = Number(e.detail);

    await wx.navigateTo({ url: `/pages/totp/detail/index?id=${id}` });
  },
  itemDelete(e: ItemDeleteEvent) {
    const currentItemId = Number(e.detail);

    this.setData({ dialogVisible: true, currentItemId });
  },
  itemMessage(e: ItemMessageEvent) {
    Message.error({
      content: e.detail,
      duration: 5000,
      offset: [20, 32],
      context: this,
    });
  },
  dialogConfirm() {
    api
      .deleteTotp(this.data.currentItemId)
      .catch((e: HttpError) =>
        Message.error({
          content: `删除失败：${e.message}`,
          duration: 5000,
          offset: [20, 32],
          context: this,
        }),
      )
      .finally(() => {
        this.dialogCancel();
        this.all();
      });
  },
  dialogCancel() {
    this.setData({ dialogVisible: false, currentItemId: 0 });
  },
});

export default {};
