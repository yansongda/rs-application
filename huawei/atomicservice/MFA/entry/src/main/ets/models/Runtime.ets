import { HashMap } from '@kit.ArkTS';
import { Totp } from '../api/totp';
import { IItem, IItemConfig } from '../types/Item';

export interface IItemRuntime {
  id: string;
  code: string | undefined;
  progress: number | undefined;
  issuer: string;
  username: string;
  config: IItemConfig;
  timeoutId: number | undefined;
  intervalId: number | undefined;
}

@ObservedV2
export class ItemRuntime {
  constructor(runtime: IItemRuntime) {
    this._id = runtime.id;
    this._code = runtime.code;
    this._progress = runtime.progress;
    this._issuer = runtime.issuer;
    this._username = runtime.username;
    this._config = runtime.config;
    this._timeoutId = runtime.timeoutId;
    this._intervalId = runtime.intervalId;
  }

  private _id: string;

  public get id(): string {
    return this._id;
  }

  public set id(value: string) {
    this._id = value;
  }

  @Trace
  private _code: string | undefined;

  public get code(): string | undefined {
    return this._code;
  }

  public set code(value: string | undefined) {
    this._code = value;
  }

  @Trace
  private _progress: number | undefined;

  public get progress(): number | undefined {
    return this._progress;
  }

  public set progress(value: number | undefined) {
    this._progress = value;
  }

  private _issuer: string;

  public get issuer(): string {
    return this._issuer;
  }

  public set issuer(value: string) {
    this._issuer = value;
  }

  private _username: string;

  public get username(): string {
    return this._username;
  }

  public set username(value: string) {
    this._username = value;
  }

  private _config: IItemConfig;

  public get config(): IItemConfig {
    return this._config;
  }

  public set config(value: IItemConfig) {
    this._config = value;
  }

  private _timeoutId: number | undefined;

  public get timeoutId(): number | undefined {
    return this._timeoutId;
  }

  public set timeoutId(value: number | undefined) {
    this._timeoutId = value;
  }

  private _intervalId: number | undefined;

  public get intervalId(): number | undefined {
    return this._intervalId;
  }

  public set intervalId(value: number | undefined) {
    this._intervalId = value;
  }

  start() {
    this.stop();

    this.freshCode();

    const now = new Date();

    let progress = this._config.period - (now.getSeconds() % this._config.period);
    this._progress = progress;

    const intervalId = setInterval(() => {
      progress--;

      if (progress <= 0) {
        clearInterval(this._intervalId);
      }

      this._progress = progress;
    }, 1000);

    const timeoutId = setTimeout((): void => this.start(), Math.max(progress, 1) * 1000);

    this._timeoutId = timeoutId;
    this._intervalId = intervalId;
  }

  stop() {
    clearTimeout(this._timeoutId)
    clearInterval(this._intervalId)

    this._code = undefined;
    this._progress = undefined;
    this._timeoutId = undefined;
    this._intervalId = undefined;
  }

  async freshCode() {
    const detail = await Totp.detail(this._id);
    this._code = detail.code;
  }
}

@ObservedV2
export class ItemsRuntime {
  private _runtimes: HashMap<string, ItemRuntime> = new HashMap();

  constructor(items: undefined | Array<IItem> | Iterable<IItem> = undefined) {
    if ('undefined' === typeof items) {
      return;
    }

    try {
      this._runtimes.clear();
    } catch (error) {
    }

    for (const item of items) {
      this.init(item.id, item);
    }
  }

  @Trace
  private _ids: Set<string> = new Set();

  get ids(): Set<string> {
    return this._ids;
  }

  start(id: string) {
    this.get(id).start();
  }

  stop(id: string) {
    this.get(id).stop();
  }

  delete(id: string) {
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

  isEmpty(): boolean {
    return this._ids.size > 0;
  }

  clear() {
    this._ids.forEach((id: string): void => this.delete(id));
  }

  private init(id: string, item: IItem): ItemRuntime {
    this.stop(id);

    const runtime = new ItemRuntime({
      id: item.id,
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