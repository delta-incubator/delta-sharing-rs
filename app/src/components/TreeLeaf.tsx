import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { RefObject, useCallback, useMemo } from "react";
import { useNotify, useTreeScope } from "../context";
import DeleteDialog from "./DeleteDialog";
import ucClient from "../client";

// helper type that asserts the name property is a string
type ItemProps<Info> = {
    info: Info & { name: string };
    ref: RefObject<HTMLDivElement> | null;
};

function TreeLeaf<Info>({ info, ref }: ItemProps<Info>) {
    const { scope, value, parentValue, parentScope } = useTreeScope(info.name);

    const { deleteFn, typeName } = useMemo(() => {
        if (scope.length === 2) {
            switch (scope[0]) {
                case "catalogs":
                    return {
                        deleteFn: ucClient.catalogs.delete,
                        typeName: "Catalog",
                    };
                case "external_locations":
                    return {
                        deleteFn: ucClient.externalLocations.delete,
                        typeName: "External location",
                    };
                case "shares":
                    return {
                        deleteFn: ucClient.shares.delete,
                        typeName: "Share",
                    };
                case "credentials":
                    return {
                        deleteFn: ucClient.credentials.delete,
                        typeName: "Credential",
                    };
                case "recipients":
                    return {
                        deleteFn: ucClient.recipients.delete,
                        typeName: "Recipient",
                    };
            }
        }

        if (scope.length === 3 && scope[0] === "catalogs") {
            return {
                deleteFn: (name: string) =>
                    ucClient.schemas.delete({ catalog: scope[1], name }),
                typeName: "Schema",
            };
        }

        throw new Error(`Unknown scope: ${scope}`);
    }, [scope, info]);

    const notify = useNotify();
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: deleteFn,
        onError: () =>
            notify("error", `Failed to delete ${typeName.toLowerCase()}.`),
        onSuccess: () => {
            notify("success", `${typeName} deleted successfully.`);
            queryClient.invalidateQueries({ queryKey: parentScope });
        },
    });

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
}

export default TreeLeaf;
