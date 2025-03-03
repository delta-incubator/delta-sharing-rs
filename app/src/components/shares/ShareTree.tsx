import {
    ShareMultipleRegular,
    ShareAndroidRegular,
} from "@fluentui/react-icons";
import ucClient, { ShareInfo } from "../../client";
import ItemTree from "../TreeRoot";
import { RefObject, useCallback } from "react";
import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import DeleteDialog from "../DeleteDialog";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useNotify } from "../../context";
import { useTreeScope } from "../../hooks";

type LocInfo = {
    name: string;
} & ShareInfo;

type ShareItemProps = {
    info: LocInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const ShareItem = ({ info, ref }: ShareItemProps) => {
    const { scope, value, parentScope, parentValue } = useTreeScope(info.name);

    const notify = useNotify();
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.shares.delete,
        onError: () => notify("error", `Failed to delete share.`),
        onSuccess: () => {
            notify("success", "Deleted share successfully.");
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
                iconBefore={<ShareAndroidRegular />}
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

type ShareTreeProps = {
    setSize: number;
    setPos: number;
};

const ShareTree = ({ setSize, setPos }: ShareTreeProps) => {
    return (
        <ItemTree
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.shares.list()}
            itemComponent={ShareItem}
            icon={<ShareMultipleRegular />}
            rootName="Shares"
        />
    );
};

export default ShareTree;
