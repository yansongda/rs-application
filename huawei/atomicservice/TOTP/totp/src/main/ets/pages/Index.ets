import { authentication } from '@kit.AccountKit';
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { IItem, IItemConfig } from '../types/Item';

const DOMAIN = 0x0000;

@Entry
@ComponentV2
struct Index {
  @Local items: Array<IItem> = [];

  aboutToAppear() {
    hilog.info(DOMAIN, 'testTag', '%{public}s', 'Ability onCreate');
    // this.loginWithHuaweiID();
    for (let index = 0; index < 100; index++) {
      this.items.push({
        id: index.toString(),
        issuer: 'github',
        username: 'me@yansongda.cnfffffffgithubaaaaaaaagithubaaaaaaaagithubaaaaaaaagithubaaaaaaaacnfffffffgithubaaaaaaaagithubaaaaaaaagithubaaaaaaaagithubaaaaaaaacnfffffffgithubaaaaaaaagithubaaaaaaaagithubaaaaaaaagithubaaaaaaaacnfffffffgithubaaaaaaaagithubaaaaaaaagithubaaaaaaaagithubaaaaaaaa',
        code: '123456',
        config: {
          period: 30,
        } as IItemConfig,
      } as IItem)
    }
  }

  build() {
    Column() {
      List() {
        Repeat<IItem>(this.items)
          .each((ri: RepeatItem<IItem>) => {
            ListItem() {
              Column() {
                Row() {
                  Column() {
                    Column() {
                      Text(ri.item.issuer)
                        .fontSize('30')
                        .fontColor('#636363')
                        .textAlign(TextAlign.Center)
                        .maxLines(1)
                        .textOverflow({ overflow: TextOverflow.Ellipsis })
                    }

                    Column() {
                      Text(ri.item.username)
                        .fontSize('12')
                        .fontColor('gray')
                        .textAlign(TextAlign.Center)
                        .maxLines(2)
                        .textOverflow({ overflow: TextOverflow.Ellipsis })
                    }
                  }
                  .width('40%')

                  Column() {
                    Text(ri.item.code)
                      .fontSize('50')
                  }
                  .width('60%')
                }
                .height('98')

                Progress({
                  value: 24,
                  total: 100,
                  type: ProgressType.Linear
                })
                  .height('2')
              }
              .padding({top: 0, bottom: 0, left: 10, right: 10})
            }
            .height('100')
            .backgroundColor('white')
          })
          .virtualScroll({ totalCount: this.items.length })
      }
      .height('100%')
      .width('100%')
    }
    .backgroundColor('#f3f3f3')
  }

  /**
   * Sample code for using HUAWEI ID to log in to atomic service.
   * According to the Atomic Service Review Guide, when a atomic service has an account system,
   * the option to log in with a HUAWEI ID must be provided.
   * The following presets the atomic service to use the HUAWEI ID silent login function.
   * To enable the atomic service to log in successfully using the HUAWEI ID, please refer
   * to the HarmonyOS HUAWEI ID Access Guide to configure the client ID and fingerprint certificate.
   */
  private loginWithHuaweiID() {
    // Create a login request and set parameters
    const loginRequest = new authentication.HuaweiIDProvider().createLoginWithHuaweiIDRequest();
    // Whether to forcibly launch the HUAWEI ID login page when the user is not logged in with the HUAWEI ID
    loginRequest.forceLogin = false;
    // Execute login request
    const controller = new authentication.AuthenticationController();
    controller.executeRequest(loginRequest).then((data) => {
      const loginWithHuaweiIDResponse = data as authentication.LoginWithHuaweiIDResponse;
      const authCode = loginWithHuaweiIDResponse.data?.authorizationCode;
      // Send authCode to the backend in exchange for unionID, session

    }).catch((error: BusinessError) => {
      hilog.error(DOMAIN, 'testTag', 'error: %{public}s', JSON.stringify(error));
      if (error.code === authentication.AuthenticationErrorCode.ACCOUNT_NOT_LOGGED_IN) {
        // HUAWEI ID is not logged in, it is recommended to jump to the login guide page

      }
    });
  }
}