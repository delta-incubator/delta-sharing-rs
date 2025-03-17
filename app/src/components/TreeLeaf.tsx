import {
    FlatTreeItem,
    TreeItemLayout,
    TreeItemLayoutProps,
} from "@fluentui/react-components";
import { RefObject } from "react";
import { useTreeScope } from "../context";
import DeleteDialog from "./DeleteDialog";

export type TreeLeafProps<Info> = {
    info: Info & { name: string };
    ref: RefObject<HTMLDivElement> | null;
    icon?: TreeItemLayoutProps["iconBefore"];
    setSize: number;
    setPos: number;
};

function TreeLeaf<Info>({
    info,
    ref,
    icon,
    setSize,
    setPos,
}: TreeLeafProps<Info>) {
    const { scope, value, parentValue } = useTreeScope();

    const title = `Delete ${info.name}?`;
    const content = `Are you sure you want to delete ${info.name}?`;

    return (
        <FlatTreeItem
            ref={ref}
            parentValue={parentValue}
            value={value}
            aria-level={scope.length}
            aria-setsize={setSize}
            aria-posinset={setPos}
            itemType="leaf"
        >
            <TreeItemLayout
                iconBefore={icon}
                actions={<DeleteDialog title={title} content={content} />}
            >
                {info.name}
            </TreeItemLayout>
        </FlatTreeItem>
    );
}

export default TreeLeaf;
