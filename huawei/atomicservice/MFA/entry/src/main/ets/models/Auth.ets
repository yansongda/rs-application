@ObservedV2
export class Authorization {
  @Trace
  private _access_token: string;

  public set access_token(value: string) {
    this._access_token = value;
  }

  public get access_token(): string {
    return this._access_token;
  }

  @Trace
  private _refresh_token: string;

  public set refresh_token(value: string) {
    this._refresh_token = value;
  }

  public get refresh_token(): string {
    return this._refresh_token;
  }

  @Trace
  private _expired_at: Date;

  public set expired_at(value: Date) {
    this._expired_at = value;
  }

  public get expired_at(): Date {
    return this._expired_at;
  }

  constructor(access_token: string, refresh_token: string, expired_in: number) {
    this._access_token = access_token;
    this._refresh_token = refresh_token;
    this._expired_at = new Date(Date.now() + (expired_in - 600) * 1000);
  }
}