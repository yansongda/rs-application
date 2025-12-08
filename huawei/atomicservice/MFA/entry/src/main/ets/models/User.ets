import { Type } from '@kit.ArkUI';

@ObservedV2
export class Config {
  @Trace
  private _avatar: string | undefined;

  public get avatar(): string | undefined {
    return this._avatar;
  }

  public set avatar(value: string | undefined) {
    this._avatar = value;
  }

  @Trace
  private _nickname: string | undefined;

  public get nickname(): string | undefined {
    return this._nickname;
  }

  public set nickname(value: string | undefined) {
    this._nickname = value;
  }

  @Trace
  private _slogan: string | undefined;

  public get slogan(): string | undefined {
    return this._slogan;
  }

  public set slogan(value: string | undefined) {
    this._slogan = value;
  }
}

@ObservedV2
export class User {
  constructor() {
    this._config = new Config();
  }

  @Trace
  private _id: string | undefined;

  public get id(): string | undefined {
    return this._id;
  }

  public set id(value: string | undefined) {
    this._id = value;
  }

  @Trace
  private _phone: string | undefined;

  public get phone(): string | undefined {
    return this._phone;
  }

  public set phone(value: string | undefined) {
    this._phone = value;
  }

  @Type(Config)
  @Trace
  private _config: Config;

  public get config(): Config {
    return this._config;
  }

  public set config(value: Config) {
    this._config = value;
  }
}