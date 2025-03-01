import {
    FlatTree,
    FlatTreeItem,
    Spinner,
    TreeItemLayout,
    TreeItemValue,
} from "@fluentui/react-components";
import { DatabaseMultiple20Regular } from "@fluentui/react-icons";
import { useQuery } from "@tanstack/react-query";
import { useCallback, useEffect, useRef, useState } from "react";
import ucClient from "../client";
import CreateCatalog from "./CatalogCreate";
import CatalogItem from "./CatalogItem";
import { TreeItemOnChange } from "../types";

export const TreeView = () => {
    return (
        <>
            <FlatTree aria-label="Lazy Loading">
                <CatalogTree value="Catalogs" />
            </FlatTree>
        </>
    );
};

type CatalogTreeProps = {
    value: TreeItemValue;
};

const CATALOGS_ROOT = "catalogs";

const CatalogTree = ({ value }: CatalogTreeProps) => {
    const [open, setOpen] = useState(false);

    const { data, status } = useQuery({
        queryKey: [CATALOGS_ROOT],
        queryFn: () => {
            return ucClient.listCatalogs();
        },
        enabled: open,
        refetchInterval: 30000,
    });

    const onOpenChange: TreeItemOnChange = useCallback(
        (_ev, data) => setOpen(data.open),
        [setOpen],
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
                value={CATALOGS_ROOT}
                aria-level={1}
                aria-setsize={4}
                aria-posinset={1}
                itemType="branch"
                open={open}
                onOpenChange={onOpenChange}
                about="All catalogs"
            >
                <TreeItemLayout
                    iconBefore={<DatabaseMultiple20Regular />}
                    expandIcon={
                        open && status === "pending" ? (
                            <Spinner size="tiny" />
                        ) : undefined
                    }
                    actions={<CreateCatalog />}
                >
                    {value.toString()}
                </TreeItemLayout>
            </FlatTreeItem>
            {open &&
                status === "success" &&
                data.map(
                    (item) =>
                        item.name && (
                            <CatalogItem
                                key={`${value}.${item.name}`}
                                parent={[CATALOGS_ROOT]}
                                catalog={item as { name: string }}
                            />
                        ),
                )}
        </>
    );
};

export default TreeView;
