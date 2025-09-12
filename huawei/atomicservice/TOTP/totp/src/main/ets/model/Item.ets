export interface IItem {
  id: string;
  issuer: string;
  username: string;
  code: string;
  config: ItemConfig;
}

export class Item {
  private id: string;
  private issuer: string;
  private username: string;
  private code: string;
  private config: ItemConfig;

  constructor() {
  }
}

export class ItemConfig {
  private period: number;

  constructor(period: number) {
    this.period = period;
  }
}