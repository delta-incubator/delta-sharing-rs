import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { RefObject, useCallback } from "react";
import ucClient, { SchemaInfo } from "../../client";
import { useTreeContext, useNotify } from "../../context";
import { useTreeScope } from "../../hooks";
import DeleteDialog from "../DeleteDialog";

// helper type that asserts the name property is a string
type LocCSchemaInfo = {
    name: string;
} & SchemaInfo;

type SchemaItemProps = {
    info: LocCSchemaInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const SchemaItem = ({ info, ref }: SchemaItemProps) => {
    const { value, parentValue } = useTreeScope(info.name);

    const queryKey = useTreeContext();
    const notify = useNotify();
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.schemas.delete,
        onError: () => notify("error", `Failed to delete schema`),
        onSuccess: () => {
            notify("success", "Deleted schema successfully.");
            queryClient.invalidateQueries({ queryKey });
        },
    });

    // properties for the delete dialog
    const title = `Delete ${info.name}?`;
    const content = `Are you sure you want to delete ${info.name}?`;
    const onClick = useCallback(() => {
        mutation.mutate({
            catalog: queryKey[queryKey.length - 1],
            name: info.name,
        });
    }, [mutation, queryKey, info]);

    return (
        <FlatTreeItem
            ref={ref}
            parentValue={parentValue}
            value={value}
            aria-level={queryKey.length + 1}
            aria-setsize={1}
            aria-posinset={1}
            itemType="leaf"
        >
            <TreeItemLayout
                actions={
                    <DeleteDialog
                        onClick={onClick}
                        title={title}
                        content={content}
                    />
                }
            >
                {info.name}
            </TreeItemLayout>
        </FlatTreeItem>
    );
};

export default SchemaItem;
