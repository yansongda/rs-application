import { CustomColors } from '@kit.ArkUI';

export class MainTheme implements CustomTheme {
  colors?: CustomColors;
  darkColors?: CustomColors;

  constructor(colors: CustomColors, darkColors?: CustomColors) {
    this.colors = colors;
    this.darkColors = darkColors;
  }
}

export class MainLightColors implements CustomColors {
  brand = "#5bb43b";
}

export class MainDarkColors implements CustomColors {
  brand = "#ff5b963b";
}