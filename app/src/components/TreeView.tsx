import { FlatTree } from "@fluentui/react-components";
import CatalogTree from "./catalog/CatalogTree";
import { TreeContext } from "../context";

export const TreeView = () => {
    return (
        <FlatTree appearance="subtle">
            <TreeContext.Provider value={["catalogs"]}>
                <CatalogTree value="Catalogs" setSize={1} setPos={1} />
            </TreeContext.Provider>
        </FlatTree>
    );
};

export default TreeView;
