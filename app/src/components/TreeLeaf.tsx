import {
    FlatTreeItem,
    TreeItemLayout,
    Text,
    Tag,
    TreeItemLayoutProps,
} from "@fluentui/react-components";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { RefObject, useCallback, useMemo } from "react";
import { useNotify, useTreeScope, useTypeName } from "../context";
import DeleteDialog from "./DeleteDialog";
import ucClient from "../client";

export type TreeLeafProps<Info> = {
    info: Info & { name: string };
    ref: RefObject<HTMLDivElement> | null;
    icon?: TreeItemLayoutProps["iconBefore"];
};

function TreeLeaf<Info>({ info, ref, icon }: TreeLeafProps<Info>) {
    const { scope, value, parentValue, parentScope } = useTreeScope();

    const typeName = useTypeName(scope);
    const { deleteFn } = useMemo(() => {
        if (scope.length === 2) {
            switch (scope[0]) {
                case "catalogs":
                    return {
                        deleteFn: ucClient.catalogs.delete,
                    };
                case "external_locations":
                    return {
                        deleteFn: ucClient.externalLocations.delete,
                    };
                case "shares":
                    return {
                        deleteFn: ucClient.shares.delete,
                    };
                case "credentials":
                    return {
                        deleteFn: ucClient.credentials.delete,
                    };
                case "recipients":
                    return {
                        deleteFn: ucClient.recipients.delete,
                    };
            }
        }

        if (scope.length === 3 && scope[0] === "catalogs") {
            return {
                deleteFn: (name: string) =>
                    ucClient.schemas.delete({ catalog: scope[1], name }),
            };
        }

        throw new Error(`Unknown scope: ${scope}`);
    }, [scope]);

    const notify = useNotify();
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: deleteFn,
        onError: () =>
            notify("error", `Failed to delete ${typeName.toLowerCase()}.`),
        onSuccess: () => {
            const fullName = scope.slice(1).join(".");
            const message = (
                <span>
                    <Text>{typeName}</Text>
                    <Tag>{fullName}</Tag>
                    <Text>deleted successfully</Text>
                </span>
            );
            notify("success", message);
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
                iconBefore={icon}
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
