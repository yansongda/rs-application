import { HashMap } from '@kit.ArkTS';
import { IItem, IItemConfig } from '../types/Item';

export interface IItemRuntime {
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
  @Trace
  private _code: string | undefined;

  public set code(value: string | undefined) {
    this._code = value;
  }

  public get code(): string | undefined {
    return this._code;
  }

  @Trace
  private _progress: number | undefined;

  public set progress(value: number | undefined) {
    this._progress = value;
  }

  public get progress(): number | undefined {
    return this._progress;
  }

  private _issuer: string;

  public set issuer(value: string) {
    this._issuer = value;
  }

  public get issuer(): string {
    return this._issuer;
  }

  private _username: string;

  public set username(value: string) {
    this._username = value;
  }

  public get username(): string {
    return this._username;
  }

  private _config: IItemConfig;

  public set config(value: IItemConfig) {
    this._config = value;
  }

  public get config(): IItemConfig {
    return this._config;
  }

  private _timeoutId: number | undefined;

  public set timeoutId(value: number | undefined) {
    this._timeoutId = value;
  }

  public get timeoutId(): number | undefined {
    return this._timeoutId;
  }

  private _intervalId: number | undefined;

  public set intervalId(value: number | undefined) {
    this._intervalId = value;
  }

  public get intervalId(): number | undefined {
    return this._intervalId;
  }

  constructor(runtime: IItemRuntime) {
    this._code = runtime.code;
    this._progress = runtime.progress;
    this._issuer = runtime.issuer;
    this._username = runtime.username;
    this._config = runtime.config;
    this._timeoutId = runtime.timeoutId;
    this._intervalId = runtime.intervalId;
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

  freshCode() {
    // TODO: 调用接口更新 code
    this._code = Math.floor(Math.random() * 999999).toString().padStart(6, '0');
  }
}

@ObservedV2
export class ItemsRuntime {
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