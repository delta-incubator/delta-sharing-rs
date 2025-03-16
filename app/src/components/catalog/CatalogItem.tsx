import { Database20Regular } from "@fluentui/react-icons";
import { RefObject } from "react";
import ucClient, { CatalogInfo, SchemaInfo } from "../../client";
import TreeBranch from "../TreeBranch";
import TreeLeaf, { type TreeLeafProps } from "../TreeLeaf";

// helper type that asserts the name property is a string
type LocCatalogInfo = {
    name: string;
} & CatalogInfo;

type CatalogItemProps = {
    info: LocCatalogInfo;
    ref: RefObject<HTMLDivElement> | null;
    setSize: number;
    setPos: number;
};

// Leaf component for schemas
const SchemaLeaf = ({
    info,
    ref,
    setPos,
    setSize,
}: Omit<TreeLeafProps<SchemaInfo>, "icon">) => {
    return <TreeLeaf info={info} ref={ref} setSize={setSize} setPos={setPos} />;
};

const CatalogItem = ({ info }: CatalogItemProps) => {
    // List function for schemas
    const listSchemas = () => ucClient.schemas.list(info.name);

    return (
        <TreeBranch
            setSize={1}
            setPos={1}
            listFn={listSchemas}
            ItemComponent={SchemaLeaf}
            icon={<Database20Regular />}
            rootName={info.name}
        />
    );
};

export default CatalogItem;
