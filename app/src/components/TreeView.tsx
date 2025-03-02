import { FlatTree } from "@fluentui/react-components";
import CatalogTree from "./catalog/CatalogTree";
import CredentialTree from "./credentials/CredentialTree";
import { TreeContext } from "../context";

export const TreeView = () => {
    return (
        <FlatTree appearance="subtle">
            <TreeContext.Provider value={["catalogs"]}>
                <CatalogTree setSize={2} setPos={1} />
            </TreeContext.Provider>
            <TreeContext.Provider value={["credentials"]}>
                <CredentialTree setSize={2} setPos={2} />
            </TreeContext.Provider>
        </FlatTree>
    );
};

export default TreeView;
