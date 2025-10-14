if (!("finalizeConstruction" in ViewPU.prototype)) {
    Reflect.set(ViewPU.prototype, "finalizeConstruction", () => { });
}
import { ItemsRuntime } from "@bundle:com.atomicservice.6917576589568238756/totp/ets/models/ItemsRuntime";
import { AlertDialogV2 as AlertDialogV2 } from "@ohos:arkui.advanced.DialogV2";
import { AdvancedDialogV2Button as AdvancedDialogV2Button } from "@ohos:arkui.advanced.DialogV2";
import hilog from "@ohos:hilog";
import type { IItem, IItemConfig } from '../types/Item';
export interface INavigationDestinationParam {
    id: string;
}
export class Index extends ViewV2 {
    constructor(parent, params, __localStorage, elmtId = -1, paramsLambda, extraInfo) {
        super(parent, elmtId, extraInfo);
        this.initParam("pages", (params && "pages" in params) ? params.pages : undefined);
        this.itemsRuntime = new ItemsRuntime();
        this.swipeActionId = undefined;
        this.finalizeConstruction();
    }
    public resetStateVarsOnReuse(params: Object): void {
        this.resetParam("pages", (params && "pages" in params) ? params.pages : undefined);
    }
    @Param
    readonly pages: NavPathStack;
    private itemsRuntime: ItemsRuntime;
    private swipeActionId?: string;
    aboutToAppear() {
        // todo: 1、调接口查询所有数据
        let items: Array<IItem> = [];
        for (let index = 0; index < 100; index++) {
            items.push({
                id: index.toString(),
                issuer: '闫嵩达大' + index,
                username: 'meaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' + index + '@yansongda.cn',
                code: Math.floor(Math.random() * 999999).toString().padStart(6, '0'),
                config: {
                    period: 30,
                } as IItemConfig,
            } as IItem);
        }
        this.itemsRuntime = new ItemsRuntime(items);
    }
    initialRender() {
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            List.create();
            List.debugLine("totp/src/main/ets/pages/Index.ets(37:5)", "totp");
            List.height({ "id": 16777244, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            List.width({ "id": 16777226, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
        }, List);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Repeat<string>(Array.from(this.itemsRuntime.ids.values()), this).each((ri: RepeatItem<string>) => {
                {
                    const itemCreation2 = (elmtId, isInitialRender) => {
                        ListItem.create(() => { }, false);
                        ListItem.borderRadius({ "id": 16777278, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                        ListItem.height({ "id": 16777279, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                        ListItem.backgroundColor({ "id": 16777285, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                        ListItem.swipeAction({
                            end: {
                                builder: () => { this.swipeAction(ri.item.valueOf()); }
                            },
                            edgeEffect: SwipeEdgeEffect.None
                        });
                        ListItem.onAttach(() => {
                            this.itemsRuntime.start(ri.item.valueOf());
                        });
                        ListItem.onDetach(() => {
                            this.itemsRuntime.stop(ri.item.valueOf());
                        });
                        ListItem.onClick(() => {
                            this.pages.pushPathByName('detail', { id: ri.item.valueOf() } as INavigationDestinationParam);
                        });
                        ListItem.debugLine("totp/src/main/ets/pages/Index.ets(40:11)", "totp");
                    };
                    const observedDeepRender = () => {
                        this.observeComponentCreation2(itemCreation2, ListItem);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Column.create();
                            Column.debugLine("totp/src/main/ets/pages/Index.ets(41:13)", "totp");
                        }, Column);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Row.create();
                            Row.debugLine("totp/src/main/ets/pages/Index.ets(42:15)", "totp");
                            Row.height({ "id": 16777273, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                            Row.padding({ left: { "id": 16777276, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" }, right: { "id": 16777277, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" } });
                        }, Row);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Column.create();
                            Column.debugLine("totp/src/main/ets/pages/Index.ets(43:17)", "totp");
                            Column.width({ "id": 16777246, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                        }, Column);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Column.create();
                            Column.debugLine("totp/src/main/ets/pages/Index.ets(44:19)", "totp");
                        }, Column);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create(this.itemsRuntime.getIssuer(ri.item.valueOf()));
                            Text.debugLine("totp/src/main/ets/pages/Index.ets(45:21)", "totp");
                            Text.fontSize({ "id": 16777275, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                            Text.fontColor({ "id": 16777284, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                            Text.textAlign(TextAlign.Center);
                            Text.maxLines(1);
                            Text.textOverflow({ overflow: TextOverflow.Ellipsis });
                        }, Text);
                        Text.pop();
                        Column.pop();
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Column.create();
                            Column.debugLine("totp/src/main/ets/pages/Index.ets(53:19)", "totp");
                            Column.margin({ top: { "id": 16777283, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" } });
                        }, Column);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create(this.itemsRuntime.getUsername(ri.item.valueOf()));
                            Text.debugLine("totp/src/main/ets/pages/Index.ets(54:21)", "totp");
                            Text.fontSize({ "id": 16777282, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                            Text.fontColor({ "id": 16777289, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                            Text.textAlign(TextAlign.Center);
                            Text.maxLines(2);
                            Text.textOverflow({ overflow: TextOverflow.Ellipsis });
                        }, Text);
                        Text.pop();
                        Column.pop();
                        Column.pop();
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Column.create();
                            Column.debugLine("totp/src/main/ets/pages/Index.ets(65:17)", "totp");
                            Column.width({ "id": 16777242, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                        }, Column);
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Text.create(this.itemsRuntime.get(ri.item.valueOf()).code);
                            Text.debugLine("totp/src/main/ets/pages/Index.ets(66:19)", "totp");
                            Text.fontSize({ "id": 16777274, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                        }, Text);
                        Text.pop();
                        Column.pop();
                        Row.pop();
                        this.observeComponentCreation2((elmtId, isInitialRender) => {
                            Progress.create({
                                value: this.itemsRuntime.get(ri.item.valueOf()).progress,
                                total: this.itemsRuntime.get(ri.item.valueOf()).period,
                                type: ProgressType.Linear
                            });
                            Progress.debugLine("totp/src/main/ets/pages/Index.ets(74:15)", "totp");
                            Progress.height({ "id": 16777280, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
                        }, Progress);
                        Column.pop();
                        ListItem.pop();
                    };
                    observedDeepRender();
                }
            })
                .virtualScroll({ totalCount: this.itemsRuntime.ids.size }).render(isInitialRender);
        }, Repeat);
        List.pop();
    }
    swipeAction(id: string, parent = null) {
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Row.create();
            Row.debugLine("totp/src/main/ets/pages/Index.ets(109:5)", "totp");
        }, Row);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Button.createWithChild();
            Button.debugLine("totp/src/main/ets/pages/Index.ets(110:7)", "totp");
            Button.height({ "id": 16777243, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Button.width({ "id": 16777281, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Button.type(ButtonType.Normal);
            Button.backgroundColor({ "id": 16777287, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Button.onClick(() => {
                this.pages.pushPathByName('detail', { id } as INavigationDestinationParam);
            });
        }, Button);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Text.create({ "id": 16777248, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Text.debugLine("totp/src/main/ets/pages/Index.ets(111:9)", "totp");
            Text.fontColor({ "id": 16777288, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
        }, Text);
        Text.pop();
        Button.pop();
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Button.createWithChild();
            Button.debugLine("totp/src/main/ets/pages/Index.ets(121:7)", "totp");
            Button.height({ "id": 16777243, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Button.width({ "id": 16777281, "type": 10007, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Button.type(ButtonType.Normal);
            Button.backgroundColor({ "id": 16777286, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Button.onClick(() => {
                this.swipeActionId = id;
                this.getUIContext().getPromptAction().openCustomDialog({
                    builder: () => {
                        this.deleteDialog();
                    }
                }).catch(() => {
                    hilog.error(0, 'totp/index', '调用 `openCustomDialog` 失败');
                });
            });
        }, Button);
        this.observeComponentCreation2((elmtId, isInitialRender) => {
            Text.create({ "id": 16777247, "type": 10003, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
            Text.debugLine("totp/src/main/ets/pages/Index.ets(122:9)", "totp");
            Text.fontColor({ "id": 16777288, "type": 10001, params: [], "bundleName": "com.atomicservice.6917576589568238756", "moduleName": "totp" });
        }, Text);
        Text.pop();
        Button.pop();
        Row.pop();
    }
    deleteDialog(parent = null) {
        {
            this.observeComponentCreation2((elmtId, isInitialRender) => {
                if (isInitialRender) {
                    let componentCall = new AlertDialogV2(this, {
                        primaryTitle: '确认删除',
                        content: '删除后将无法恢复，请谨慎操作',
                        primaryButton: new AdvancedDialogV2Button({
                            content: '取消',
                        }),
                        secondaryButton: new AdvancedDialogV2Button({
                            content: '删除',
                            role: ButtonRole.ERROR,
                            action: () => {
                                if ('undefined' == typeof this.swipeActionId) {
                                    return;
                                }
                                // todo: 1、调接口删除数据
                                this.itemsRuntime.clear(this.swipeActionId);
                                this.swipeActionId = undefined;
                            }
                        }),
                    }, undefined, elmtId, () => { }, { page: "totp/src/main/ets/pages/Index.ets", line: 143, col: 5 });
                    ViewV2.create(componentCall);
                    let paramsLambda = () => {
                        return {
                            primaryTitle: '确认删除',
                            content: '删除后将无法恢复，请谨慎操作',
                            primaryButton: new AdvancedDialogV2Button({
                                content: '取消',
                            }),
                            secondaryButton: new AdvancedDialogV2Button({
                                content: '删除',
                                role: ButtonRole.ERROR,
                                action: () => {
                                    if ('undefined' == typeof this.swipeActionId) {
                                        return;
                                    }
                                    // todo: 1、调接口删除数据
                                    this.itemsRuntime.clear(this.swipeActionId);
                                    this.swipeActionId = undefined;
                                }
                            })
                        };
                    };
                    componentCall.paramsGenerator_ = paramsLambda;
                }
                else {
                    this.updateStateVarsOfChildByElmtId(elmtId, {
                        primaryTitle: '确认删除',
                        content: '删除后将无法恢复，请谨慎操作',
                        primaryButton: new AdvancedDialogV2Button({
                            content: '取消',
                        }),
                        secondaryButton: new AdvancedDialogV2Button({
                            content: '删除',
                            role: ButtonRole.ERROR,
                            action: () => {
                                if ('undefined' == typeof this.swipeActionId) {
                                    return;
                                }
                                // todo: 1、调接口删除数据
                                this.itemsRuntime.clear(this.swipeActionId);
                                this.swipeActionId = undefined;
                            }
                        })
                    });
                }
            }, { name: "AlertDialogV2" });
        }
    }
    public updateStateVars(params) {
        if (params === undefined) {
            return;
        }
        if ("pages" in params) {
            this.updateParam("pages", params.pages);
        }
    }
    rerender() {
        this.updateDirtyElements();
    }
}
if (getPreviewComponentFlag()) {
    storePreviewComponents(1, "Index", new Index(undefined, {}));
    previewComponent();
}
else {
}
