import { CustomColors } from '@kit.ArkUI';

export class MainTheme implements CustomTheme {
  colors?: CustomColors;
  darkColors?: CustomColors;

  constructor(colors: CustomColors, darkColors?: CustomColors) {
    this.colors = colors;
    this.darkColors = darkColors;
  }

  static create() {
    return new MainTheme(new MainLightColors(), new MainDarkColors());
  }
}

class MainLightColors implements CustomColors {
  brand = $r('app.color.brand');
}

class MainDarkColors implements CustomColors {
  brand = $r('app.color.brand');
}