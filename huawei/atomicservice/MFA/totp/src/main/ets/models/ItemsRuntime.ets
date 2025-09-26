import { HashMap } from '@kit.ArkTS';
import { IItem } from '../types/Item';
import { hilog } from '@kit.PerformanceAnalysisKit';

export interface IItemRuntime
{
  code: string | undefined;
  progress: number | undefined;
  timeoutId: number | undefined;
  intervalId: number | undefined;
}

@ObservedV2
export class ItemRuntime
{
  @Trace
  private _code: string | undefined;

  @Trace
  private _progress: number | undefined;

  private _timeoutId: number | undefined;

  private _intervalId: number | undefined;

  constructor(runtime: IItemRuntime) {
    this._code = runtime.code;
    this._progress = runtime.progress;
    this._timeoutId = runtime.timeoutId;
    this._intervalId = runtime.intervalId;
  }

  public set code(value: string | undefined) {
    this._code = value;
  }

  public get code(): string | undefined {
    return this._code;
  }

  public set progress(value: number | undefined) {
    this._progress = value;
  }

  public get progress(): number | undefined {
    return this._progress;
  }

  public get timeoutId(): number | undefined {
    return this._timeoutId;
  }

  public get intervalId(): number | undefined {
    return this._intervalId;
  }
}

export class ItemsRuntime
{
  private instance: HashMap<number, ItemRuntime> = new HashMap();

  get(index: number): undefined | ItemRuntime {
    try {
      return this.instance.get(index);
    } catch (error) {
    }

    return undefined;
  }

  set(index: number, data: ItemRuntime) {
    try {
      this.instance.set(index, data);
    } catch (error) {
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

    this.set(index, new ItemRuntime({
      code: item.code,
      progress,
      intervalId,
      timeoutId,
    } as IItemRuntime));
  }

  clearRuntime(index: number) {
    hilog.info(0, 'runtime item', 'enter clearRuntime: %{public}i', index);

    const runtime = this.get(index);

    if ('undefined' === typeof runtime) {
      return;
    }

    clearTimeout(runtime.timeoutId)
    clearInterval(runtime.intervalId)

    try {
      this.instance.remove(index);
    } catch (error) {
    }
  }
}