import type { TabbarTap } from "types/tdesign";

Component({
  data: {
    value: "pages/home/index",
    list: [
      { value: "pages/home/index", label: "首页", icon: "home" },
      { value: "pages/user/index", label: "我的", icon: "user" },
    ],
  },
  lifetimes: {
    ready() {
      const pages = getCurrentPages();
      const curPage = pages[pages.length - 1];

      this.setData({ value: curPage.route });
    },
  },
  methods: {
    onChange(e: TabbarTap<unknown, unknown>) {
      const path = e.detail.value;

      wx.switchTab({ url: `/${path}` });
    },
  },
});
