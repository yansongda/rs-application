import { HashMap } from '@kit.ArkTS';
import { IItem } from '../types/Item';

export interface IItemRuntime
{
  code: string | undefined;
  progress: number | undefined;
  period: number;
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

  // 为了避免每一次从原始数据中查询 index，这里做个冗余
  private _period: number = 30;

  private _timeoutId: number | undefined;

  private _intervalId: number | undefined;

  constructor(runtime: IItemRuntime) {
    this._code = runtime.code;
    this._progress = runtime.progress;
    this._period = runtime.period;
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

  public set period(value: number) {
    this._period = value;
  }

  public get period(): number {
    return this._period;
  }

  public get timeoutId(): number | undefined {
    return this._timeoutId;
  }

  public set timeoutId(value: number | undefined) {
    this._timeoutId = value;
  }

  public get intervalId(): number | undefined {
    return this._intervalId;
  }

  public set intervalId(value: number | undefined) {
    this._intervalId = value;
  }
}

export class ItemsRuntime
{
  private _items: Array<IItem> = [];

  private _runtimes: HashMap<string, ItemRuntime> = new HashMap();

  constructor(items: undefined | Array<IItem> = undefined) {
    if ('undefined' === typeof items) {
      return;
    }

    this._items = items;

    items.forEach((item) => {
      this.init(item.id, item);
    })
  }

  getKeys(): string[] {
    try {
      const keys = this._runtimes.keys();

      return Array.from(keys);
    } catch (error) {
    }

    return [];
  }

  start(id: string) {
    this.stop(id);

    const now = new Date();
    const runtime = this.get(id);

    let progress = runtime.period - (now.getSeconds() % runtime.period);
    runtime.progress = progress;

    const intervalId = setInterval(() => {
      progress--;
      if (progress <= 0) {
        clearInterval(this.get(id).intervalId);
      }
      runtime.progress = progress;
    }, 1000);

    const timeoutId = setTimeout(() => {
      // TODO: 调用接口更新 code
      runtime.code = Math.floor(Math.random() * 999999).toString().padStart(6, '0');
      this.start(id);
    }, Math.max(progress, 1) * 1000);

    runtime.timeoutId = timeoutId;
    runtime.intervalId = intervalId;
  }

  stop(id: string) {
    const runtime = this.get(id);

    clearTimeout(runtime.timeoutId)
    clearInterval(runtime.intervalId)

    runtime.intervalId = undefined;
    runtime.timeoutId = undefined;
  }

  clear(id: string) {
    this.stop(id);

    try {
      this._runtimes.remove(id);
    } catch (error) {
    }

    const index = this.getItemIndex(id);

    if (index) {
      this._items.splice(index, 1);
    }
  }

  get(id: string): ItemRuntime {
    let runtime: undefined | ItemRuntime = undefined;

    try {
      runtime = this._runtimes.get(id);
    } catch (error) {
    }

    return runtime ?? new ItemRuntime({} as IItemRuntime);
  }

  getIssuer(id: string): string {
    const index = this.getItemIndex(id);

    if ('undefined' !== typeof index) {
      return this._items[index].issuer;
    }

    return 'NaN';
  }

  getUsername(id: string): string {
    const index = this.getItemIndex(id);

    if ('undefined' !== typeof index) {
      return this._items[index].username;
    }

    return 'NaN';
  }

  private getItemIndex(id: string): undefined | number {
    const index = this._items.findIndex((item) => item.id === id);

    return -1 === index ? undefined : index;
  }

  private init(id: string, item: IItem): ItemRuntime {
    this.stop(id);

    const runtime = new ItemRuntime({
      code: item.code,
      progress: item.config.period,
      period: item.config.period,
    } as IItemRuntime);

    this.set(id, runtime);

    return runtime;
  }

  private set(id: string, runtime: ItemRuntime) {
    try {
      this._runtimes.set(id, runtime);
    } catch (error) {
    }
  }
}