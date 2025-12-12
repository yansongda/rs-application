import { CustomColors } from '@kit.ArkUI';

class MainTheme implements CustomTheme {
  colors?: CustomColors;

  constructor(colors: CustomColors) {
    this.colors = colors;
  }
}

class MainLightColors implements CustomColors {
  brand = "#5bb43b";
}