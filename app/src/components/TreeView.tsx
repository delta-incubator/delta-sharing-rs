import {
    FlatTree,
    FlatTreeItem,
    makeStyles,
    Spinner,
    TreeItemLayout,
    TreeItemOpenChangeData,
    TreeItemOpenChangeEvent,
    TreeItemValue,
} from "@fluentui/react-components";
import { useQuery } from "@tanstack/react-query";
import { useCallback, useEffect, useRef, useState } from "react";
import ucClient from "../client";
import CreateCatalog from "./CatalogCreate";
import CatalogItem from "./CatalogItem";

type SubtreeProps = {
    value: TreeItemValue;
    onDataLoading?(): void;
    onDataLoaded?(): void;
};

const useStyles = makeStyles({
    screenReadersOnly: {
        position: "absolute",
        width: "1px",
        height: "1px",
        margin: "-1",
        overflow: "hidden",
        clip: "rect(0,0,0,0)",
        whiteSpace: "nowrap",
    },
});

export const TreeView = () => {
    const [ariaMessage, setAriaMessage] = useState("");

    const styles = useStyles();
    return (
        <>
            <FlatTree aria-label="Lazy Loading">
                <CatalogTree
                    value="Catalogs"
                    onDataLoaded={useCallback(
                        () => setAriaMessage(`people items loaded`),
                        [],
                    )}
                    onDataLoading={useCallback(
                        () => setAriaMessage(`loading people items...`),
                        [],
                    )}
                />
            </FlatTree>
            <div
                aria-live="polite"
                aria-atomic="true"
                className={styles.screenReadersOnly}
            >
                {ariaMessage}
            </div>
        </>
    );
};

const CatalogTree = ({ onDataLoaded, onDataLoading, value }: SubtreeProps) => {
    const [open, setOpen] = useState(false);

    const { data, status } = useQuery({
        queryKey: ["catalogs", "list"],
        queryFn: () => {
            onDataLoading?.();
            return ucClient.listCatalogs();
        },
        enabled: open,
        refetchInterval: 30000,
    });

    const handleOpenChange = useCallback(
        (_e: TreeItemOpenChangeEvent, data: TreeItemOpenChangeData) => {
            setOpen(data.open);
        },
        [setOpen],
    );

    // we need to focus the first item when the subtree is opened
    const firstItemRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (open && status === "success") {
            onDataLoaded?.();
            firstItemRef.current?.focus();
        }
    }, [open, status, onDataLoaded]);

    return (
        <>
            <FlatTreeItem
                value="catalogs"
                aria-level={1}
                aria-setsize={3}
                aria-posinset={1}
                itemType="branch"
                open={open}
                onOpenChange={handleOpenChange}
                about="All catalogs"
            >
                <TreeItemLayout
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
                        item.name !== undefined && (
                            <CatalogItem
                                key={`${value}.${item.name}`}
                                parent={["catalogs"]}
                                catalog={item as { name: string }}
                            />
                        ),
                )}
        </>
    );
};

export default TreeView;
