import { HashMap } from '@kit.ArkTS';
import { IItem, IItemConfig } from '../types/Item';

export interface IItemRuntime
{
  code: string | undefined;
  progress: number | undefined;
  issuer: string;
  username: string;
  config: IItemConfig;
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

  private _issuer: string;

  private _username: string;

  private _config: IItemConfig;

  private _timeoutId: number | undefined;

  private _intervalId: number | undefined;

  constructor(runtime: IItemRuntime) {
    this._code = runtime.code;
    this._progress = runtime.progress;
    this._issuer = runtime.issuer;
    this._username = runtime.username;
    this._config = runtime.config;
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

  public set issuer(value: string) {
    this._issuer = value;
  }

  public get issuer(): string {
    return this._issuer;
  }

  public set username(value: string) {
    this._username = value;
  }

  public get username(): string {
    return this._username;
  }

  public set config(value: IItemConfig) {
    this._config = value;
  }

  public get config(): IItemConfig {
    return this._config;
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

@ObservedV2
export class ItemsRuntime
{
  @Trace
  private _ids: Set<string> = new Set();

  private _runtimes: HashMap<string, ItemRuntime> = new HashMap();

  constructor(items: undefined | Array<IItem> = undefined) {
    if ('undefined' === typeof items) {
      return;
    }

    try {
      this._runtimes.clear();
    } catch (error) {
    }

    items.forEach((item) => {
      this.init(item.id, item);
    })
  }

  get ids(): Set<string> {
    return this._ids;
  }

  start(id: string) {
    this.stop(id);

    const now = new Date();
    const runtime = this.get(id);

    let progress = runtime.config.period - (now.getSeconds() % runtime.config.period);
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

    this._ids.delete(id);
  }

  get(id: string): ItemRuntime {
    let runtime: undefined | ItemRuntime = undefined;

    try {
      runtime = this._runtimes.get(id);
    } catch (error) {
    }

    return runtime ?? new ItemRuntime({} as IItemRuntime);
  }

  private init(id: string, item: IItem): ItemRuntime {
    this.stop(id);

    const runtime = new ItemRuntime({
      code: item.code,
      progress: item.config.period,
      issuer: item.issuer,
      username: item.username,
      config: item.config,
    } as IItemRuntime);

    try {
      this._runtimes.set(id, runtime);
      this._ids.add(id);
    } catch (error) {
    }

    return runtime;
  }
}