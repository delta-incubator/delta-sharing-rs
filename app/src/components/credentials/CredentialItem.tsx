import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { RefObject, useCallback, useContext } from "react";
import ucClient, { CredentialInfo } from "../../client";
import { NotifyContext, TreeContext } from "../../context";
import { useTreeScope } from "../../hooks";
import DeleteDialog from "../DeleteDialog";

// helper type that asserts the name property is a string
type LocCredentialInfo = {
    name: string;
} & CredentialInfo;

type SchemaItemProps = {
    info: LocCredentialInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const CredentialItem = ({ info, ref }: SchemaItemProps) => {
    const queryKey = useContext(TreeContext);
    const { value, parentValue } = useTreeScope(queryKey, info.name);

    const notify = useContext(NotifyContext);
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.credentials.delete,
        onError: () => notify("error", `Failed to delete credential`),
        onSuccess: () => {
            notify("success", "Deleted credential successfully.");
            queryClient.invalidateQueries({ queryKey });
        },
    });

    // properties for the delete dialog
    const title = `Delete ${info.name}?`;
    const content = `Are you sure you want to delete ${info.name}?`;
    const onClick = useCallback(() => {
        mutation.mutate(info.name);
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

export default CredentialItem;
