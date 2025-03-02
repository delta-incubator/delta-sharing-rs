import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { RefObject, useCallback, useContext } from "react";
import ucClient, { SchemaInfo } from "../client";
import { NotifyContext, TreeContext } from "../context";
import { useTreeScope } from "../hooks";
import DeleteDialog from "./DeleteDialog";

// helper type that asserts the name property is a string
type LocCSchemaInfo = {
    name: string;
} & SchemaInfo;

type SchemaItemProps = {
    info: LocCSchemaInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const SchemaItem = ({ info, ref }: SchemaItemProps) => {
    const parentScope = useContext(TreeContext);
    const { value, parentValue } = useTreeScope(parentScope, info.name);

    const queryKey = useContext(TreeContext);
    const notify = useContext(NotifyContext);
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.deleteSchema,
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
            catalog: parentScope[parentScope.length - 1],
            name: info.name,
        });
    }, [mutation, queryKey, info]);

    return (
        <FlatTreeItem
            ref={ref}
            parentValue={parentValue}
            value={value}
            aria-level={parentScope.length + 1}
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
