import {
    FlatTreeItem,
    TreeItemLayout,
    Spinner,
    Button,
    TreeItemLayoutProps,
} from "@fluentui/react-components";
import { AddRegular } from "@fluentui/react-icons";
import { useQuery } from "@tanstack/react-query";
import {
    ComponentType,
    RefObject,
    useCallback,
    useEffect,
    useRef,
    useState,
} from "react";
import { TreeContext, useExplorer, useTreeContext } from "../context";
import { TreeItemOnChange } from "../types";

type Icon = TreeItemLayoutProps["iconBefore"];

type TreeRootProps<Item> = {
    setSize: number;
    setPos: number;
    listFn: () => Promise<Item[]>;
    ItemComponent: ComponentType<{
        info: Item & { name: string };
        ref: RefObject<HTMLDivElement> | null;
        setSize: number;
        setPos: number;
    }>;
    icon: Icon;
    rootName: string;
};

export const CreateItem = ({ scope }: { scope: string[] }) => {
    const { update } = useExplorer();
    const onClick = useCallback(() => {
        update({ display: "create", scope });
    }, [update]);

    return (
        <Button appearance="subtle" onClick={onClick} icon={<AddRegular />} />
    );
};

function ItemTree<Item extends { name?: string }>({
    setSize,
    setPos,
    listFn,
    icon,
    ItemComponent,
    rootName,
}: TreeRootProps<Item>) {
    const [open, setOpen] = useState(false);
    const onOpenChange: TreeItemOnChange = useCallback(
        (_ev, data) => setOpen(data.open),
        [setOpen],
    );

    const rootScope = useTreeContext();
    const rootValue = rootScope[0];
    const { data, status } = useQuery({
        queryKey: rootScope,
        queryFn: listFn,
        enabled: open,
        refetchInterval: 30000,
    });

    const firstItemRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (open && status === "success") firstItemRef.current?.focus();
    }, [open, status]);

    return (
        <>
            <TreeContext.Provider value={rootScope}>
                <FlatTreeItem
                    value={rootValue}
                    aria-level={rootScope.length}
                    aria-setsize={setSize}
                    aria-posinset={setPos}
                    itemType="branch"
                    open={open}
                    onOpenChange={onOpenChange}
                >
                    <TreeItemLayout
                        iconBefore={icon}
                        expandIcon={
                            open && status === "pending" ? (
                                <Spinner size="extra-tiny" />
                            ) : undefined
                        }
                        actions={<CreateItem scope={rootScope} />}
                    >
                        {rootName}
                    </TreeItemLayout>
                </FlatTreeItem>
            </TreeContext.Provider>
            {open &&
                status === "success" &&
                data.map(
                    (item, index) =>
                        item.name && (
                            <TreeContext.Provider
                                value={[...rootScope, item.name]}
                            >
                                <ItemComponent
                                    key={`${rootValue}.${item.name}`}
                                    ref={index === 0 ? firstItemRef : null}
                                    // @ts-expect-error
                                    info={item}
                                    setSize={data.length}
                                    setPos={index + 1}
                                />
                            </TreeContext.Provider>
                        ),
                )}
        </>
    );
}

export default ItemTree;
