export class Delay {
  static sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  static async execute<T>(fn: () => T, ms: number): Promise<T> {
    await Delay.sleep(ms);

    return fn();
  }
}