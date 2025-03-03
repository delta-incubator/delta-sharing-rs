import { CloudCubeRegular } from "@fluentui/react-icons";
import ucClient, { ExternalLocationInfo } from "../../client";
import ItemTree from "../TreeRoot";
import { RefObject, useCallback } from "react";
import { useNotify } from "../../context";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import DeleteDialog from "../DeleteDialog";
import { CubeLinkRegular } from "@fluentui/react-icons";
import { useTreeScope } from "../../hooks";

type TreeProps = {
    setSize: number;
    setPos: number;
};

type LocInfo = {
    name: string;
} & ExternalLocationInfo;

type ExternalLocationItemProps = {
    info: LocInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const ExternalLocationItem = ({ info, ref }: ExternalLocationItemProps) => {
    const { scope, value, parentScope, parentValue } = useTreeScope(info.name);

    const notify = useNotify();
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.externalLocations.delete,
        onError: () => notify("error", `Failed to delete external location.`),
        onSuccess: () => {
            notify("success", "Deleted external location successfully.");
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
                iconBefore={<CubeLinkRegular />}
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
            listFn={() => ucClient.externalLocations.list()}
            itemComponent={ExternalLocationItem}
            icon={<CloudCubeRegular />}
            rootName="External Locations"
        />
    );
};

export default ExtLocTree;
