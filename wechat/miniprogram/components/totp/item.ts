import api from "@api/totp";
import type { HttpError } from "@models/error";

Component({
  properties: {
    item: {
      type: Object,
    },
  },

  data: {
    remainSeconds: 0,
    refreshCodeTimeoutIdentity: -1,
    countdownIntervalIdentity: -1,
  },

  lifetimes: {
    attached() {
      console.log("attached: ", this.data.item);
      this.countdownRefresh();
    },
    detached() {
      console.log("detached: ", this.data.item);
      this.clear();
    },
  },

  methods: {
    countdownRefresh() {
      this.clear();

      const period = this.data.item.config.period ?? 30;
      const now = new Date();
      const remainSeconds = period - (now.getSeconds() % period);
      const msUntilNextPeriod = remainSeconds * 1000 - now.getMilliseconds();

      this.data.refreshCodeTimeoutIdentity = setTimeout(() => {
        this.refreshCode(this.data.item.id);
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
        .then((response) => this.setData({ [`item.code`]: response.code }))
        .catch((e: HttpError) => this.triggerEvent("refreshCodeFailed", e));
    },
    clear() {
      if (this.data.refreshCodeTimeoutIdentity) {
        clearTimeout(this.data.refreshCodeTimeoutIdentity);
      }
      if (this.data.countdownIntervalIdentity) {
        clearInterval(this.data.countdownIntervalIdentity);
      }
    },
  },
});
