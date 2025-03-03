import { FlatTree } from "@fluentui/react-components";
import CatalogTree from "./catalog/CatalogTree";
import CredentialTree from "./credentials/CredentialTree";
import ExternalLocationTree from "./external_locations/ExternalLocationTree";
import { TreeContext } from "../context";

export const TreeView = () => {
    return (
        <FlatTree appearance="subtle">
            <TreeContext.Provider value={["catalogs"]}>
                <CatalogTree setSize={3} setPos={1} />
            </TreeContext.Provider>
            <TreeContext.Provider value={["credentials"]}>
                <CredentialTree setSize={3} setPos={2} />
            </TreeContext.Provider>
            <TreeContext.Provider value={["external_locations"]}>
                <ExternalLocationTree setSize={3} setPos={3} />
            </TreeContext.Provider>
        </FlatTree>
    );
};

export default TreeView;
