if (!("finalizeConstruction" in ViewPU.prototype)) {
    Reflect.set(ViewPU.prototype, "finalizeConstruction", () => { });
}
import authentication from "@hms:core.authentication";
import type { BusinessError as BusinessError } from "@ohos:base";
import hilog from "@ohos:hilog";
import scanBarcode from "@hms:core.scan.scanBarcode";
import { Index } from "@bundle:com.atomicservice.6917576589568238756/totp/ets/pages/Index";
class Home extends ViewV2 {
    constructor(parent, params, __localStorage, elmtId = -1, paramsLambda, extraInfo) {
        super(parent, elmtId, extraInfo);
        this.pages = new NavPathStack();
        this.finalizeConstruction();
    }
    public resetStateVarsOnReuse(params: Object): void {
        this.pages = new NavPathStack();
    }
    @Local
    private pages: NavPathStack;
    aboutToAppear() {
        // this.loginWithHuaweiID();
    }
    initialRender() {
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Stack.create({ alignContent: Alignment.BottomEnd });
            Stack.debugLine("totp/src/main/ets/pages/Home.ets(18:5)", "totp");
            Stack.backgroundColor({ "id": 125831015, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Stack.expandSafeArea([SafeAreaType.SYSTEM], [SafeAreaEdge.TOP, SafeAreaEdge.BOTTOM]);
        }, Stack);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Navigation.create(this.pages, { moduleName: "totp", pagePath: "totp/src/main/ets/pages/Home", isUserCreateStack: true });
            Navigation.debugLine("totp/src/main/ets/pages/Home.ets(19:7)", "totp");
            Navigation.title({ builder: () => {
                    this.title.call(this);
                } });
            Navigation.titleMode(NavigationTitleMode.Mini);
            Navigation.mode(NavigationMode.Stack);
            Navigation.hideBackButton(true);
        }, Navigation);
        {
            this.observeComponentCreation2((elmtId, isInitialRender) => {
                if (isInitialRender) {
                    let componentCall = new Index(this, { pages: this.pages }, undefined, elmtId, () => { }, { page: "totp/src/main/ets/pages/Home.ets", line: 20, col: 9 });
                    ViewV2.create(componentCall);
                    let paramsLambda = () => {
                        return {
                            pages: this.pages
                        };
                    };
                    componentCall.paramsGenerator_ = paramsLambda;
                }
                else {
                    this.updateStateVarsOfChildByElmtId(elmtId, {
                        pages: this.pages
                    });
                }
            }, { name: "Index" });
        }
        Navigation.pop();
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Button.createWithChild({ type: ButtonType.ROUNDED_RECTANGLE });
            Button.debugLine("totp/src/main/ets/pages/Home.ets(27:7)", "totp");
            Button.fontColor(Color.White);
            Button.padding({ top: { "id": 16777268, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" }, bottom: { "id": 16777265, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" }, left: { "id": 16777266, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" }, right: { "id": 16777267, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" } });
            Button.margin({ bottom: { "id": 16777263, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" }, right: { "id": 16777264, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" } });
            Button.onClick(() => {
                scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then((result) => {
                    // todo: 添加
                    hilog.info(0, 'add button result', result.originalValue);
                });
            });
        }, Button);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Row.create();
            Row.debugLine("totp/src/main/ets/pages/Home.ets(28:9)", "totp");
        }, Row);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Text.create();
            Text.debugLine("totp/src/main/ets/pages/Home.ets(29:11)", "totp");
            Text.margin({ right: { "id": 16777262, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" } });
        }, Text);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            SymbolSpan.create({ "id": 125831597, "type": 40000, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            SymbolSpan.debugLine("totp/src/main/ets/pages/Home.ets(30:13)", "totp");
            SymbolSpan.fontSize({ "id": 16777224, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            SymbolSpan.fontColor([Color.White]);
        }, SymbolSpan);
        Text.pop();
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Text.create({ "id": 16777245, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Text.debugLine("totp/src/main/ets/pages/Home.ets(36:11)", "totp");
            Text.fontSize({ "id": 16777224, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
        }, Text);
        Text.pop();
        Row.pop();
        Button.pop();
        Stack.pop();
    }
    title(parent = null) {
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Row.create();
            Row.debugLine("totp/src/main/ets/pages/Home.ets(56:5)", "totp");
            Row.width('100%');
            Row.margin({ top: { "id": 16777272, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" } });
        }, Row);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Image.create({ "id": 16777218, "type": 20000, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Image.debugLine("totp/src/main/ets/pages/Home.ets(57:7)", "totp");
            Image.width({ "id": 16777270, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Image.height({ "id": 16777270, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Image.margin({ left: { "id": 16777269, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" }, right: { "id": 16777269, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" } });
        }, Image);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Text.create({ "id": 16777253, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Text.debugLine("totp/src/main/ets/pages/Home.ets(58:7)", "totp");
            Text.fontWeight(FontWeight.Bold);
            Text.fontSize({ "id": 16777271, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
        }, Text);
        Text.pop();
        Row.pop();
    }
    private loginWithHuaweiID() {
        const loginRequest = new authentication.HuaweiIDProvider().createLoginWithHuaweiIDRequest();
        loginRequest.forceLogin = false;
        const controller = new authentication.AuthenticationController();
        controller.executeRequest(loginRequest).then((data) => {
            const loginWithHuaweiIDResponse = data as authentication.LoginWithHuaweiIDResponse;
            const authCode = loginWithHuaweiIDResponse.data?.authorizationCode;
            // Send authCode to the backend in exchange for unionID, session
        }).catch((error: BusinessError) => {
            hilog.error(0, 'testTag', 'error: %{public}s', JSON.stringify(error));
            if (error.code === authentication.AuthenticationErrorCode.ACCOUNT_NOT_LOGGED_IN) {
                // HUAWEI ID is not logged in, it is recommended to jump to the login guide page
            }
        });
    }
    rerender() {
        this.updateDirtyElements();
    }
    static getEntryName(): string {
        return "Home";
    }
}
registerNamedRoute(() => new Home(undefined, {}), "", { bundleName: "com.atomicservice.6917576589568238756", moduleName: "totp", pagePath: "pages/Home", pageFullPath: "totp/src/main/ets/pages/Home", integratedHsp: "false", moduleType: "followWithHap" });
