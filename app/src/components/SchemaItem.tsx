import { FlatTreeItem, TreeItemLayout } from "@fluentui/react-components";
import { RefObject, useContext } from "react";
import { SchemaInfo } from "../client";
import DeleteSchema from "./SchemaDelete";
import { useTreeScope } from "../hooks";
import { TreeContext } from "../context";

// helper type that asserts the name property is a string
type LocCSchemaInfo = {
    name: string;
} & SchemaInfo;

type SchemaItemProps = {
    info: LocCSchemaInfo;
    ref: RefObject<HTMLDivElement> | null;
};

const SchemaItem = ({ info, ref }: SchemaItemProps) => {
    const scope = useContext(TreeContext);
    const { value, parentValue } = useTreeScope(scope, info.name);

    return (
        <FlatTreeItem
            ref={ref}
            parentValue={parentValue}
            value={value}
            aria-level={scope.length + 1}
            aria-setsize={1}
            aria-posinset={1}
            itemType="leaf"
        >
            <TreeItemLayout actions={<DeleteSchema name={info.name} />}>
                {info.name}
            </TreeItemLayout>
        </FlatTreeItem>
    );
};

export default SchemaItem;
