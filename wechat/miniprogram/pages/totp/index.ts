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
    currentItemId: "0",
    items: [] as Item[],
    isSortMode: false,
    dragItems: [] as any[],
    draggingIndex: -1,
    dragCurrentY: 0,
  },
  onShow() {
    this.all();
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
    const id = e.detail;

    await wx.navigateTo({ url: `/pages/totp/detail/index?id=${id}` });
  },
  itemDelete(e: ItemDeleteEvent) {
    const currentItemId = e.detail;

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
    this.setData({ dialogVisible: false, currentItemId: "0" });
  },
  enterSortMode() {
    this.setData({
      isSortMode: true,
      dragItems: this.data.items.map((item, index) => ({
        ...item,
        y: index * 100,
      })),
    });
  },
  exitSortMode() {
    this.setData({ isSortMode: false });
  },
  
  onDragStart(e: any) {
    const { index } = e.currentTarget.dataset;
    this.setData({ draggingIndex: index });
  },
  onDragChange(e: any) {
    if (e.detail.source === "touch") {
      this.data.dragCurrentY = e.detail.y;
    }
  },
  onDragEnd(e: any) {
    const { index } = e.currentTarget.dataset;
    if (this.data.draggingIndex !== index) return;

    const y = this.data.dragCurrentY;
    let newIndex = Math.round(y / 100);
    newIndex = Math.max(0, Math.min(this.data.dragItems.length - 1, newIndex));

    if (newIndex !== index) {
      const newDragItems = [...this.data.dragItems];
      const [movedItem] = newDragItems.splice(index, 1);
      newDragItems.splice(newIndex, 0, movedItem);

      newDragItems.forEach((item, i) => {
        item.y = i * 100;
      });

      this.setData({ dragItems: newDragItems });
    } else {
      // Snap back if index didn't change
      const newDragItems = [...this.data.dragItems];
      newDragItems[index].y = index * 100;
      this.setData({ dragItems: newDragItems });
    }

    this.setData({ draggingIndex: -1 });
  },
  saveSort() {
    const reorderedItems = this.data.dragItems.map((di) => {
      const { y, ...rest } = di;
      return rest;
    });
    this.exitSortMode();
    this.onSortChange({ detail: reorderedItems });
  },

  onSortChange(e: { detail: Item[] }) {
    const originalItems = this.data.items.slice();
    const reorderedItems = e.detail;
    const sortItems = reorderedItems.map((item, index) => ({
      id: item.id,
      sort: reorderedItems.length - 1 - index,
    }));

    api
      .sort(sortItems)
      .then(() => {
        this.setData({ items: reorderedItems });
      })
      .catch((err: HttpError) => {
        this.setData({ items: originalItems });
        Message.error({
          content: `排序失败：${err.message}`,
          duration: 5000,
          offset: [20, 32],
          context: this,
        });
      });
  },
});
