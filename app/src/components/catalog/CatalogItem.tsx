import {
    DatabaseRegular,
    TableRegular,
    TableSimpleMultipleRegular,
} from "@fluentui/react-icons";
import { RefObject } from "react";
import ucClient, { CatalogInfo, SchemaInfo, TableInfo } from "../../client";
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

// Leaf component for tables
const TableLeaf = ({
    info,
    ref,
    setSize,
    setPos,
}: Omit<TreeLeafProps<TableInfo>, "icon">) => {
    return (
        <TreeLeaf
            info={info}
            ref={ref}
            icon={<TableRegular />}
            setSize={setSize}
            setPos={setPos}
        />
    );
};

// Branch component for schemas with tables as children
function SchemaItem({
    info,
    setSize,
    setPos,
}: {
    info: SchemaInfo & { name: string; catalogName: string };
    ref: RefObject<HTMLDivElement> | null;
    setSize: number;
    setPos: number;
}) {
    // List function for tables
    const listTables = () => {
        return ucClient.tables.list({
            catalog: info.catalogName,
            schema: info.name,
        });
    };

    return (
        <TreeBranch
            setSize={setSize}
            setPos={setPos}
            listFn={listTables}
            ItemComponent={TableLeaf}
            icon={<TableSimpleMultipleRegular />}
            rootName={info.name}
        />
    );
}

const CatalogItem = ({ info }: CatalogItemProps) => {
    // List function for schemas
    const listSchemas = () => ucClient.schemas.list({ catalog: info.name });

    return (
        <TreeBranch
            setSize={1}
            setPos={1}
            listFn={listSchemas}
            // @ts-expect-error: catalogName is not optional. need to propagete more constrained type
            ItemComponent={SchemaItem}
            icon={<DatabaseRegular />}
            rootName={info.name}
        />
    );
};

export default CatalogItem;
