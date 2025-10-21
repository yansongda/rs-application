@ObservedV2
export class Config {
  @Trace
  private _avatar: string | undefined;

  public set avatar(value: string | undefined) {
    this._avatar = value;
  }

  public get avatar(): string | undefined {
    return this._avatar;
  }

  @Trace
  private _nickname: string | undefined;

  public set nickname(value: string | undefined) {
    this._nickname = value;
  }

  public get nickname(): string | undefined {
    return this._nickname;
  }

  @Trace
  private _slogan: string | undefined;

  public set slogan(value: string | undefined) {
    this._slogan = value;
  }

  public get slogan(): string | undefined {
    return this._slogan;
  }
}

@ObservedV2
export class User {
  private _id: string;

  public set id(value: string) {
    this._id = value;
  }

  public get id(): string {
    return this._id;
  }

  @Trace
  private _phone: string | undefined;

  public set phone(value: string | undefined) {
    this._phone = value;
  }

  public get phone(): string | undefined {
    return this._phone;
  }

  private _config: Config | undefined;

  public set config(value: Config | undefined) {
    this._config = value;
  }

  public get config(): Config | undefined {
    return this._config;
  }

  constructor(id: string) {
    this._id = id;
  }


}