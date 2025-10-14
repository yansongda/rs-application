if (!("finalizeConstruction" in ViewPU.prototype)) {
    Reflect.set(ViewPU.prototype, "finalizeConstruction", () => { });
}
import hilog from "@ohos:hilog";
import type { INavigationDestinationParam } from "./Index";
export function builder(param: Object, parent = null) {
    const __param__ = param;
    {
        (parent ? parent : this).observeComponentCreation2((elmtId, isInitialRender, param = __param__) => {
            if (isInitialRender) {
                let componentCall = new Detail(parent ? parent : this, { detailId: (param as INavigationDestinationParam).id }, undefined, elmtId, () => { }, { page: "totp/src/main/ets/pages/Detail.ets", line: 6, col: 3 });
                ViewV2.create(componentCall);
                let paramsLambda = () => {
                    return {
                        detailId: (param as INavigationDestinationParam).id
                    };
                };
                componentCall.paramsGenerator_ = paramsLambda;
            }
            else {
                (parent ? parent : this).updateStateVarsOfChildByElmtId(elmtId, {
                    detailId: (param as INavigationDestinationParam).id
                });
            }
        }, { name: "Detail" });
    }
}
class Detail extends ViewV2 {
    constructor(parent, params, __localStorage, elmtId = -1, paramsLambda, extraInfo) {
        super(parent, elmtId, extraInfo);
        this.initParam("detailId", (params && "detailId" in params) ? params.detailId : undefined);
        this.pages = new NavPathStack();
        this.finalizeConstruction();
    }
    public resetStateVarsOnReuse(params: Object): void {
        this.resetParam("detailId", (params && "detailId" in params) ? params.detailId : undefined);
    }
    @Param
    readonly detailId: string;
    pages: NavPathStack;
    aboutToAppear(): void {
        hilog.info(0, 'totp/detail', 'about to appear: ' + this.detailId);
    }
    initialRender() {
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            NavDestination.create(() => {
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    Column.create();
                    Column.debugLine("totp/src/main/ets/pages/Detail.ets(22:7)", "totp");
                }, Column);
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    Column.create();
                    Column.debugLine("totp/src/main/ets/pages/Detail.ets(23:9)", "totp");
                    Column.margin({ bottom: 20 });
                }, Column);
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    Text.create('实时秘钥');
                    Text.debugLine("totp/src/main/ets/pages/Detail.ets(24:11)", "totp");
                    Text.fontColor(Color.Grey);
                    Text.textAlign(TextAlign.Start);
                    Text.width('100%');
                    Text.margin({ left: 30, bottom: 10 });
                }, Text);
                Text.pop();
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    Column.create();
                    Column.debugLine("totp/src/main/ets/pages/Detail.ets(30:11)", "totp");
                }, Column);
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    Text.create('123456');
                    Text.debugLine("totp/src/main/ets/pages/Detail.ets(31:13)", "totp");
                    Text.textAlign(TextAlign.Center);
                    Text.fontSize(70);
                    Text.width('100%');
                }, Text);
                Text.pop();
                Column.pop();
                Column.pop();
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    Column.create();
                    Column.debugLine("totp/src/main/ets/pages/Detail.ets(39:9)", "totp");
                }, Column);
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    Text.create('基础信息');
                    Text.debugLine("totp/src/main/ets/pages/Detail.ets(40:11)", "totp");
                    Text.fontColor(Color.Grey);
                    Text.textAlign(TextAlign.Start);
                    Text.width('100%');
                    Text.margin({ left: 30, bottom: 10 });
                }, Text);
                Text.pop();
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    List.create({ space: 10 });
                    List.debugLine("totp/src/main/ets/pages/Detail.ets(46:11)", "totp");
                    List.width('100%');
                    List.height('100%');
                }, List);
                this.observeComponentCreation2((elmtId, isInitialRender) => {
                    ListItemGroup.create({ style: ListItemGroupStyle.CARD });
                    ListItemGroup.debugLine("totp/src/main/ets/pages/Detail.ets(47:13)", "totp");
                    ListItemGroup.divider({ strokeWidth: 1 });
                }, ListItemGroup);
                {
                    const itemCreation = (elmtId, isInitialRender) => {
                        ViewStackProcessor.StartGetAccessRecordingFor(elmtId);
                        ListItem.create(deepRenderFunction, true, { style: ListItemStyle.CARD });
                        if (!isInitialRender) {
                            ListItem.pop();
                        }
                        ViewStackProcessor.StopGetAccessRecording();
                    };
                    const itemCreation2 = (elmtId, isInitialRender) => {
                        ListItem.create(deepRenderFunction, true, { style: ListItemStyle.CARD });
                        ListItem.debugLine("totp/src/main/ets/pages/Detail.ets(48:15)", "totp");
                    };
                    const deepRenderFunction = (elmtId, isInitialRender) => {
                        itemCreation(elmtId, isInitialRender);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Row.create();
                            Row.debugLine("totp/src/main/ets/pages/Detail.ets(49:17)", "totp");
                        }, Row);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create('发行方');
                            Text.debugLine("totp/src/main/ets/pages/Detail.ets(50:19)", "totp");
                            Text.textAlign(TextAlign.Start);
                            Text.width('20%');
                        }, Text);
                        Text.pop();
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Row.create();
                            Row.debugLine("totp/src/main/ets/pages/Detail.ets(54:19)", "totp");
                            Row.width('80%');
                        }, Row);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create('Github');
                            Text.debugLine("totp/src/main/ets/pages/Detail.ets(55:21)", "totp");
                            Text.textAlign(TextAlign.End);
                            Text.width('90%');
                            Text.maxLines(1);
                            Text.textOverflow({ overflow: TextOverflow.Ellipsis });
                        }, Text);
                        Text.pop();
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create();
                            Text.debugLine("totp/src/main/ets/pages/Detail.ets(61:21)", "totp");
                            Text.textAlign(TextAlign.End);
                            Text.width('10%');
                        }, Text);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            SymbolSpan.create({ "id": 125832664, "type": 40000, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                            SymbolSpan.debugLine("totp/src/main/ets/pages/Detail.ets(62:23)", "totp");
                        }, SymbolSpan);
                        Text.pop();
                        Row.pop();
                        Row.pop();
                        ListItem.pop();
                    };
                    this.observeComponentCreation2(itemCreation2, ListItem);
                    ListItem.pop();
                }
                {
                    const itemCreation = (elmtId, isInitialRender) => {
                        ViewStackProcessor.StartGetAccessRecordingFor(elmtId);
                        ListItem.create(deepRenderFunction, true, { style: ListItemStyle.CARD });
                        if (!isInitialRender) {
                            ListItem.pop();
                        }
                        ViewStackProcessor.StopGetAccessRecording();
                    };
                    const itemCreation2 = (elmtId, isInitialRender) => {
                        ListItem.create(deepRenderFunction, true, { style: ListItemStyle.CARD });
                        ListItem.debugLine("totp/src/main/ets/pages/Detail.ets(71:15)", "totp");
                    };
                    const deepRenderFunction = (elmtId, isInitialRender) => {
                        itemCreation(elmtId, isInitialRender);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Row.create();
                            Row.debugLine("totp/src/main/ets/pages/Detail.ets(72:17)", "totp");
                            Row.width('100%');
                        }, Row);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create('用户名');
                            Text.debugLine("totp/src/main/ets/pages/Detail.ets(73:19)", "totp");
                            Text.textAlign(TextAlign.Start);
                            Text.width('20%');
                        }, Text);
                        Text.pop();
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Row.create();
                            Row.debugLine("totp/src/main/ets/pages/Detail.ets(77:19)", "totp");
                            Row.width('80%');
                        }, Row);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create('meaaaaaaaaaaaaaaaaaaaaaffffffffffffffffff@yansongda.cn');
                            Text.debugLine("totp/src/main/ets/pages/Detail.ets(78:21)", "totp");
                            Text.textAlign(TextAlign.End);
                            Text.width('90%');
                            Text.maxLines(1);
                            Text.textOverflow({ overflow: TextOverflow.Ellipsis });
                        }, Text);
                        Text.pop();
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create();
                            Text.debugLine("totp/src/main/ets/pages/Detail.ets(84:21)", "totp");
                            Text.textAlign(TextAlign.End);
                            Text.width('10%');
                        }, Text);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            SymbolSpan.create({ "id": 125832664, "type": 40000, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                            SymbolSpan.debugLine("totp/src/main/ets/pages/Detail.ets(85:23)", "totp");
                        }, SymbolSpan);
                        Text.pop();
                        Row.pop();
                        Row.pop();
                        ListItem.pop();
                    };
                    this.observeComponentCreation2(itemCreation2, ListItem);
                    ListItem.pop();
                }
                ListItemGroup.pop();
                List.pop();
                Column.pop();
                Column.pop();
            }, { moduleName: "totp", pagePath: "totp/src/main/ets/pages/Detail" });
            NavDestination.backgroundColor({ "id": 125831015, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            NavDestination.title({ "id": 16777250, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            NavDestination.onReady((context: NavDestinationContext) => {
                this.pages = context.pathStack;
            });
            NavDestination.debugLine("totp/src/main/ets/pages/Detail.ets(21:5)", "totp");
        }, NavDestination);
        NavDestination.pop();
    }
    public updateStateVars(params) {
        if (params === undefined) {
            return;
        }
        if ("detailId" in params) {
            this.updateParam("detailId", params.detailId);
        }
    }
    rerender() {
        this.updateDirtyElements();
    }
}
if (getPreviewComponentFlag()) {
    storePreviewComponents(1, "Detail", new Detail(undefined, {}));
    previewComponent();
}
else {
}
(function () {
    if (typeof NavigationBuilderRegister === "function") {
        NavigationBuilderRegister("detail", wrapBuilder(builder));
    }
})();
