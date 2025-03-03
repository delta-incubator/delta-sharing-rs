import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import {
    DatabasePlugConnectedRegular,
    PlugConnectedRegular,
} from "@fluentui/react-icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { RefObject, useCallback } from "react";
import ucClient, { RecipientInfo } from "../../client";
import { useNotify } from "../../context";
import { useTreeScope } from "../../hooks";
import DeleteDialog from "../DeleteDialog";
import ItemTree from "../TreeRoot";

type TreeProps = {
    setSize: number;
    setPos: number;
};

type LocInfo = {
    name: string;
} & RecipientInfo;

type RecipientItemProps = {
    info: LocInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const RecipientItem = ({ info, ref }: RecipientItemProps) => {
    const { scope, value, parentScope, parentValue } = useTreeScope(info.name);

    const notify = useNotify();
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.recipients.delete,
        onError: () => notify("error", `Failed to delete recipient.`),
        onSuccess: () => {
            notify("success", "Deleted recipient successfully.");
            queryClient.invalidateQueries({ queryKey: parentScope });
        },
    });

    // properties for the delete dialog
    const title = `Delete ${info.name}?`;
    const content = `Are you sure you want to delete ${info.name}?`;
    const onClick = useCallback(() => {
        mutation.mutate(info.name);
    }, [mutation, info]);

    return (
        <FlatTreeItem
            ref={ref}
            parentValue={parentValue}
            value={value}
            aria-level={scope.length}
            aria-setsize={1}
            aria-posinset={1}
            itemType="leaf"
        >
            <TreeItemLayout
                iconBefore={<PlugConnectedRegular />}
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

const ExtLocTree = ({ setSize, setPos }: TreeProps) => {
    return (
        <ItemTree
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.recipients.list()}
            itemComponent={RecipientItem}
            icon={<DatabasePlugConnectedRegular />}
            rootName="Recipients"
        />
    );
};

export default ExtLocTree;
