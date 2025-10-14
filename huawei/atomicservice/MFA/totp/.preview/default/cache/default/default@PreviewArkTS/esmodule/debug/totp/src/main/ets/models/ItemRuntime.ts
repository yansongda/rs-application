import HashMap from "@ohos:util.HashMap";
import type { IItem } from '../types/Item';
import hilog from "@ohos:hilog";
export interface IItemRuntime {
    code: string | undefined;
    progress: number | undefined;
    timeoutId: number | undefined;
    intervalId: number | undefined;
}
@ObservedV2
export class ItemsRuntime {
    @Trace
    private instance: HashMap<number, IItemRuntime> = new HashMap();
    get(index: number): undefined | IItemRuntime {
        try {
            return this.instance.get(index);
        }
        catch (error) {
        }
        return undefined;
    }
    set(index: number, data: IItemRuntime) {
        try {
            this.instance.set(index, data);
        }
        catch (error) {
        }
    }
    setProgress(index: number, progress: number) {
        const runtime = this.get(index);
        if ('undefined' === typeof runtime) {
            return;
        }
        runtime.progress = progress;
    }
    setCode(index: number, code: string) {
        const runtime = this.get(index);
        if ('undefined' === typeof runtime) {
            return;
        }
        runtime.code = code;
    }
    initRuntime(index: number, item: IItem) {
        hilog.info(0, 'runtime item', 'enter initRuntime: %{public}i', index);
        this.clearRuntime(index);
        const period: number = item.config.period ?? 30;
        const now = new Date();
        let progress = period - (now.getSeconds() % period);
        const timeoutSeconds = progress;
        const intervalId = setInterval(() => {
            progress--;
            if (progress <= 0) {
                clearInterval(this.get(index)?.intervalId);
            }
            this.setProgress(index, progress);
        }, 1000);
        const timeoutId = setTimeout(() => {
            // TODO: 调用接口更新 code
            hilog.info(0, 'runtime item', 'enter refresh code: %{public}i', index);
            this.initRuntime(index, item);
        }, timeoutSeconds * 1000);
        this.set(index, {
            code: item.code,
            progress,
            intervalId,
            timeoutId,
        } as IItemRuntime);
    }
    clearRuntime(index: number) {
        hilog.info(0, 'runtime item', 'enter clearRuntime: %{public}i', index);
        const runtime = this.get(index);
        if ('undefined' === typeof runtime) {
            return;
        }
        clearTimeout(runtime.timeoutId);
        clearInterval(runtime.intervalId);
        try {
            this.instance.remove(index);
        }
        catch (error) {
        }
    }
}
