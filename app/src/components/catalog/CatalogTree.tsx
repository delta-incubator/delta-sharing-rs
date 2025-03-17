import { DatabaseMultipleRegular } from "@fluentui/react-icons";
import ucClient from "../../client";
import CatalogItem from "./CatalogItem";
import TreeBranch from "../TreeBranch";

type CatalogTreeProps = {
    setSize: number;
    setPos: number;
};

const CatalogTree = ({ setSize, setPos }: CatalogTreeProps) => {
    return (
        <TreeBranch
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.catalogs.list()}
            ItemComponent={CatalogItem}
            icon={<DatabaseMultipleRegular />}
            rootName="Catalogs"
        />
    );
};

export default CatalogTree;
