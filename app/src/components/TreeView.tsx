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
import { TreeContext } from "../context";

export const TreeView = () => {
    return (
        <>
            <FlatTree appearance="subtle">
                <CatalogTree value="Catalogs" />
            </FlatTree>
        </>
    );
};

type CatalogTreeProps = {
    value: TreeItemValue;
};

const CATALOGS_ROOT = "catalogs";
const CATALOGS_SCOPE = [CATALOGS_ROOT];

const CatalogTree = ({ value }: CatalogTreeProps) => {
    const [open, setOpen] = useState(false);
    const onOpenChange: TreeItemOnChange = useCallback(
        (_ev, data) => setOpen(data.open),
        [setOpen],
    );

    const { data, status } = useQuery({
        queryKey: CATALOGS_SCOPE,
        queryFn: () => ucClient.listCatalogs(),
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
                value={CATALOGS_ROOT}
                aria-level={1}
                aria-setsize={5}
                aria-posinset={1}
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
                    {value.toString()}
                </TreeItemLayout>
            </FlatTreeItem>
            {open &&
                status === "success" &&
                data.map(
                    (item, index) =>
                        item.name && (
                            <TreeContext.Provider value={CATALOGS_SCOPE}>
                                <CatalogItem
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

export default TreeView;
