import { DatabaseMultipleRegular } from "@fluentui/react-icons";
import ucClient from "../../client";
import CatalogItem from "./CatalogItem";
import ItemTree from "../TreeBranch";

type CatalogTreeProps = {
    setSize: number;
    setPos: number;
};

const CatalogTree = ({ setSize, setPos }: CatalogTreeProps) => {
    return (
        <ItemTree
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.catalogs.list()}
            itemComponent={CatalogItem}
            icon={<DatabaseMultipleRegular />}
            rootName="Catalogs"
        />
    );
};

export default CatalogTree;
