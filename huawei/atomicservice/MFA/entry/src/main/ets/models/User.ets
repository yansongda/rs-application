import { Type } from '@kit.ArkUI';

@ObservedV2
export class Config {
  @Trace
  private _avatar: Resource | string | undefined;

  public get avatar(): Resource | string | undefined {
    return this._avatar;
  }

  public set avatar(value: Resource | string | undefined) {
    this._avatar = value;
  }

  @Trace
  private _nickname: Resource | string | undefined;

  public get nickname(): Resource | string | undefined {
    return this._nickname;
  }

  public set nickname(value: Resource | string | undefined) {
    this._nickname = value;
  }

  @Trace
  private _slogan: Resource | string | undefined;

  public get slogan(): Resource | string | undefined {
    return this._slogan;
  }

  public set slogan(value: Resource | string | undefined) {
    this._slogan = value;
  }
}

@ObservedV2
export class User {
  constructor() {
    const config = new Config();

    config.avatar = $r('app.media.default_avatar');
    config.nickname = $r('app.string.models_user_default_nickname');
    config.slogan = $r('app.string.models_user_default_slogan');

    this._config = config;
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
  private _config: Config | undefined;

  public get config(): Config | undefined {
    return this._config;
  }

  public set config(value: Config | undefined) {
    this._config = value;
  }
}