import {
    FlatTreeItem,
    Spinner,
    TreeItemLayout,
} from "@fluentui/react-components";
import { Database20Regular } from "@fluentui/react-icons";
import { useQuery } from "@tanstack/react-query";
import { useCallback, useEffect, useRef, useState } from "react";
import ucClient, { CatalogInfo } from "../client";
import { TreeContext } from "../context";
import { TreeItemOnChange } from "../types";
import CreateSchema from "./SchemaCreate";
import { useTreeScope } from "../hooks";
import SchemaItem from "./SchemaItem";

// helper type that asserts the name property is a string
type LocCatalogInfo = {
    name: string;
} & CatalogInfo;

type CatalogItemProps = {
    parent: string[];
    catalog: LocCatalogInfo;
};

const CatalogItem = ({ parent, catalog }: CatalogItemProps) => {
    const [open, setOpen] = useState(false);

    const { scope, value, parentValue } = useTreeScope(parent, catalog.name);

    const { data, status } = useQuery({
        queryKey: scope,
        queryFn: ({ queryKey }) =>
            ucClient.listSchemas(queryKey[queryKey.length - 1]),
        enabled: open,
        refetchInterval: 30000,
    });

    const onOpenChange: TreeItemOnChange = useCallback(
        (_ev, data) => setOpen(data.open),
        [],
    );

    // we need to focus the first item when the subtree is opened
    const firstItemRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (open && status === "success") {
            firstItemRef.current?.focus();
        }
    }, [open, status]);

    return (
        <>
            <FlatTreeItem
                value={value}
                aria-level={parent.length + 1}
                aria-setsize={data ? data.length : 1}
                aria-posinset={1}
                itemType="branch"
                parentValue={parentValue}
                open={open}
                onOpenChange={onOpenChange}
            >
                <TreeItemLayout
                    iconBefore={<Database20Regular />}
                    expandIcon={
                        open && status === "pending" ? (
                            <Spinner size="tiny" />
                        ) : undefined
                    }
                    actions={<CreateSchema name={catalog.name} />}
                >
                    {catalog.name}
                </TreeItemLayout>
            </FlatTreeItem>
            {open &&
                status === "success" &&
                data.map(
                    (item, index) =>
                        item.name && (
                            <TreeContext.Provider value={scope}>
                                <SchemaItem
                                    key={`${value}.${item.name}`}
                                    ref={index === 0 ? firstItemRef : null}
                                    info={item as { name: string }}
                                />
                            </TreeContext.Provider>
                        ),
                )}
        </>
    );
};

export default CatalogItem;
