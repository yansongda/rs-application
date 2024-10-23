import { DEFAULT } from "@constant/user";
import utils from "@utils/user";
import type { User } from "types/user";

Page({
	data: {
		avatar: DEFAULT.avatar,
		nickname: DEFAULT.nickname,
		slogan: DEFAULT.slogan,
	},
	async onShow() {
		const user: User = await utils.detail();

		this.setData({
			avatar: user.avatar,
			nickname: user.nickname,
			slogan: user.slogan,
		});
	},
	onHide() {},
	onReady() {},
});

export default {};
