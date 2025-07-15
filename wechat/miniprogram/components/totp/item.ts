import api from "@api/totp";
import type { HttpError } from "@models/error";

Component({
  properties: {
    itemId: Number,
    username: String,
    issuer: String,
    code: String,
    period: Number,
  },

  data: {
    remainSeconds: 0,
    refreshCodeTimeoutIdentity: -1,
    countdownIntervalIdentity: -1,
  },

  lifetimes: {
    attached() {
      this.countdownRefresh();
    },
    detached() {
      this.clear();
    },
  },

  pageLifetimes: {
    show() {
      this.countdownRefresh();
    },
    hide() {
      this.clear();
    },
  },

  methods: {
    countdownRefresh() {
      this.clear();

      const period = this.data.period ?? 30;
      const now = new Date();
      const remainSeconds = period - (now.getSeconds() % period);
      const msUntilNextPeriod = remainSeconds * 1000 - now.getMilliseconds();

      this.data.refreshCodeTimeoutIdentity = setTimeout(() => {
        this.refreshCode(this.data.itemId);
        this.countdownRefresh();
      }, msUntilNextPeriod);

      let countdown = remainSeconds;
      this.setData({ remainSeconds: countdown });
      this.data.countdownIntervalIdentity = setInterval(() => {
        countdown--;
        if (countdown <= 0) {
          clearInterval(this.data.countdownIntervalIdentity);
        }
        this.setData({ remainSeconds: countdown });
      }, 1000);
    },
    refreshCode(id: number) {
      api
        .detail(id)
        .then((response) => this.setData({ code: response.code }))
        .catch((e: HttpError) =>
          this.triggerEvent("message", `更新验证码失败：${e.message}`),
        );
    },
    async detail() {
      this.triggerEvent("detail", this.data.itemId);
    },
    delete() {
      this.triggerEvent("delete", this.data.itemId);
    },
    clear() {
      if (this.data.refreshCodeTimeoutIdentity > 0) {
        clearTimeout(this.data.refreshCodeTimeoutIdentity);
        this.data.refreshCodeTimeoutIdentity = -1;
      }

      if (this.data.countdownIntervalIdentity > 0) {
        clearInterval(this.data.countdownIntervalIdentity);
        this.data.refreshCodeTimeoutIdentity = -1;
      }
    },
  },
});
