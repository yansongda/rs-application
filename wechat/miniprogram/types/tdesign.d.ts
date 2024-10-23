import type { Tap } from "./wechat";

export interface TabbarTap<T, D> extends Tap<T, D> {
	detail: {
		value: string;
	};
}

export interface SwipeTap<DETAIL, T, D> extends Tap<T, D> {
	detail: DETAIL;
}
