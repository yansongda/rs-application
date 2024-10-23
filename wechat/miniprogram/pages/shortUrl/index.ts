import api from "@api/shortUrl";
import type { HttpError } from "@models/error";
import Message from "tdesign-miniprogram/message/index";
import Toast from "tdesign-miniprogram/toast/index";
import type { CreateResponse } from "types/shortUrl";
import type { FormSubmit } from "types/wechat";

interface FormData {
	url: string;
}

Page({
	data: {
		url: "",
		short: "",
	},
	submit(e: FormSubmit<FormData>) {
		Toast({
			message: "生成中...",
			theme: "loading",
			duration: 5000,
			direction: "column",
			preventScrollThrough: true,
		});

		api
			.create(e.detail.value.url)
			.then((response: CreateResponse) => {
				Toast({
					message: "生成成功",
					theme: "success",
					duration: 100,
					direction: "column",
				});

				this.setData({ short: response.short });
			})
			.catch((e: HttpError) => {
				Toast({
					message: "生成失败",
					theme: "error",
					duration: 100,
					direction: "column",
				});

				Message.error({
					content: `生成失败：${e.message}`,
					duration: 5000,
					offset: [20, 32],
					context: this,
				});
			});
	},
	async copy() {
		if (this.data.short === "") {
			return;
		}

		await wx.setClipboardData({ data: this.data.short });
	},
});
