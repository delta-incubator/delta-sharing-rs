import {
    FlatTreeItem,
    TreeItemLayout,
    TreeItemValue,
    Spinner,
} from "@fluentui/react-components";
import { DatabaseMultiple20Regular } from "@fluentui/react-icons";
import { useQuery } from "@tanstack/react-query";
import { useCallback, useContext, useEffect, useRef, useState } from "react";
import ucClient from "../../client";
import { TreeContext } from "../../context";
import { TreeItemOnChange } from "../../types";
import CreateCatalog from "./CatalogCreate";
import CatalogItem from "./CatalogItem";

type CatalogTreeProps = {
    value: TreeItemValue;
    setSize: number;
    setPos: number;
};

const CatalogTree = ({ setSize, setPos }: CatalogTreeProps) => {
    const [open, setOpen] = useState(false);
    const onOpenChange: TreeItemOnChange = useCallback(
        (_ev, data) => setOpen(data.open),
        [setOpen],
    );

    const rootScope = useContext(TreeContext);
    const rootValue = rootScope[0];
    const { data, status } = useQuery({
        queryKey: rootScope,
        queryFn: () => ucClient.catalogs.list(),
        enabled: open,
        refetchInterval: 30000,
    });

    const firstItemRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (open && status === "success") firstItemRef.current?.focus();
    }, [open, status]);

    return (
        <>
            <FlatTreeItem
                value={rootValue}
                aria-level={1}
                aria-setsize={setSize}
                aria-posinset={setPos}
                itemType="branch"
                open={open}
                onOpenChange={onOpenChange}
            >
                <TreeItemLayout
                    iconBefore={<DatabaseMultiple20Regular />}
                    expandIcon={
                        open && status === "pending" ? (
                            <Spinner size="extra-tiny" />
                        ) : undefined
                    }
                    actions={<CreateCatalog />}
                >
                    Catalogs
                </TreeItemLayout>
            </FlatTreeItem>
            {open &&
                status === "success" &&
                data.map(
                    (item, index) =>
                        item.name && (
                            <TreeContext.Provider value={rootScope}>
                                <CatalogItem
                                    key={`${rootValue}.${item.name}`}
                                    ref={index === 0 ? firstItemRef : null}
                                    info={item as { name: string }}
                                />
                            </TreeContext.Provider>
                        ),
                )}
        </>
    );
};

export default CatalogTree;
