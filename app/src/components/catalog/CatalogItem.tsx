import {
    FlatTreeItem,
    TreeItemLayout,
    Spinner,
} from "@fluentui/react-components";
import { Database20Regular } from "@fluentui/react-icons";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
    RefObject,
    useCallback,
    useContext,
    useEffect,
    useRef,
    useState,
} from "react";
import ucClient, { CatalogInfo } from "../../client";
import { NotifyContext, TreeContext } from "../../context";
import { useTreeScope } from "../../hooks";
import { TreeItemOnChange } from "../../types";
import DeleteDialog from "../DeleteDialog";
import CreateSchema from "./SchemaCreate";
import SchemaItem from "./SchemaItem";

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

    const parentScope = useContext(TreeContext);
    const { scope, value, parentValue } = useTreeScope(info.name);
    const { data, status } = useQuery({
        queryKey: scope,
        queryFn: ({ queryKey }) =>
            ucClient.schemas.list(queryKey[queryKey.length - 1]),
        enabled: open,
        refetchInterval: 30000,
    });

    const queryClient = useQueryClient();
    const notify = useContext(NotifyContext);
    const mutation = useMutation({
        mutationFn: ucClient.catalogs.delete,
        onError: () => notify("error", `Failed to delete schema`),
        onSuccess: () => {
            notify("success", "Deleted schema successfully.");
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
                            <CreateSchema name={info.name} />
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
