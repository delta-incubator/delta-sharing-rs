import {
    FlatTreeItem,
    Spinner,
    TreeItemLayout,
} from "@fluentui/react-components";
import { Database20Regular } from "@fluentui/react-icons";
import { useQuery } from "@tanstack/react-query";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import ucClient, { CatalogInfo } from "../client";
import { TreeItemOnChange } from "../types";
import CreateSchema from "./SchemaCreate";
import DeleteSchema from "./SchemaDelete";

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
    const parentValue = useMemo(() => parent.join("."), [parent]);
    const value = `${parentValue}.${catalog.name}`;

    console.log({ queryKey: [...parent, catalog.name, "list"] });

    const { data, status } = useQuery({
        queryKey: [...parent, catalog.name],
        queryFn: ({ queryKey }) => {
            return ucClient.listSchemas(queryKey[queryKey.length - 1]);
        },
        enabled: open,
        refetchInterval: 30000,
    });

    console.log({ data, status });

    const handleOpenChange: TreeItemOnChange = useCallback(
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
                onOpenChange={handleOpenChange}
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
                data.map((item, index) => (
                    <FlatTreeItem
                        key={`${value}.${item.name}`}
                        ref={index === 0 ? firstItemRef : null}
                        parentValue={value}
                        value={`${value}.${item.name}`}
                        aria-level={3}
                        aria-setsize={data.length}
                        aria-posinset={index + 1}
                        itemType="leaf"
                    >
                        <TreeItemLayout
                            actions={
                                item.name && (
                                    <DeleteSchema
                                        catalog={catalog.name}
                                        name={item.name}
                                    />
                                )
                            }
                        >
                            {item.name}
                        </TreeItemLayout>
                    </FlatTreeItem>
                ))}
        </>
    );
};

export default CatalogItem;
