import {
    FlatTreeItem,
    TreeItemLayout,
    Spinner,
} from "@fluentui/react-components";
import { Database20Regular } from "@fluentui/react-icons";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { RefObject, useCallback, useEffect, useRef, useState } from "react";
import ucClient, { CatalogInfo } from "../../client";
import {
    useNotify,
    useTreeContext,
    useTreeScope,
    TreeProvider,
} from "../../context";
import { TreeItemOnChange } from "../../types";
import DeleteDialog from "../DeleteDialog";
import { CreateItem } from "../TreeBranch";
import ItemLeaf from "../TreeLeaf";

// helper type that asserts the name property is a string
type LocCatalogInfo = {
    name: string;
} & CatalogInfo;

type CatalogItemProps = {
    info: LocCatalogInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const CatalogItem = ({ info, ref }: CatalogItemProps) => {
    const [open, setOpen] = useState(false);
    const onOpenChange: TreeItemOnChange = useCallback(
        (_ev, data) => setOpen(data.open),
        [],
    );

    const parentScope = useTreeContext();
    const { scope, value, parentValue } = useTreeScope(info.name);
    const { data, status } = useQuery({
        queryKey: scope,
        queryFn: ({ queryKey }) =>
            ucClient.schemas.list(queryKey[queryKey.length - 1]),
        enabled: open,
        refetchInterval: 30000,
    });

    const queryClient = useQueryClient();
    const notify = useNotify();
    const mutation = useMutation({
        mutationFn: ucClient.catalogs.delete,
        onError: () => notify("error", `Failed to delete catalog`),
        onSuccess: () => {
            notify("success", "Deleted catalog successfully.");
            queryClient.invalidateQueries({ queryKey: parentScope });
        },
    });

    // properties for the delete dialog
    const title = `Delete ${info.name}?`;
    const content = `Are you sure you want to delete ${info.name}?`;
    const onClick = useCallback(() => {
        mutation.mutate(info.name);
    }, [mutation, info]);

    // we need to focus the first item when the subtree is opened
    const firstItemRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (open && status === "success") firstItemRef.current?.focus();
    }, [open, status]);

    return (
        <>
            <FlatTreeItem
                ref={ref}
                value={value}
                aria-level={parentScope.length + 1}
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
                            <Spinner size="extra-tiny" />
                        ) : undefined
                    }
                    actions={
                        <>
                            <DeleteDialog
                                onClick={onClick}
                                title={title}
                                content={content}
                            />
                            <CreateItem scope={scope} />
                        </>
                    }
                >
                    {info.name}
                </TreeItemLayout>
            </FlatTreeItem>
            {open &&
                status === "success" &&
                data.map(
                    (item, index) =>
                        item.name && (
                            <TreeProvider value={scope}>
                                <ItemLeaf
                                    key={`${value}.${item.name}`}
                                    ref={index === 0 ? firstItemRef : null}
                                    info={item as { name: string }}
                                />
                            </TreeProvider>
                        ),
                )}
        </>
    );
};

export default CatalogItem;
