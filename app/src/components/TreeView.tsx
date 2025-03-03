import { FlatTree } from "@fluentui/react-components";
import CatalogTree from "./catalog/CatalogTree";
import CredentialTree from "./credentials/CredentialTree";
import ExternalLocationTree from "./external_locations/ExternalLocationTree";
import RecipientTree from "./recipients/RecipientTree";
import ShareTree from "./shares/ShareTree";
import { TreeContext } from "../context";

export const TreeView = () => {
    return (
        <FlatTree appearance="subtle">
            <TreeContext.Provider value={["catalogs"]}>
                <CatalogTree setSize={5} setPos={1} />
            </TreeContext.Provider>
            <TreeContext.Provider value={["credentials"]}>
                <CredentialTree setSize={5} setPos={2} />
            </TreeContext.Provider>
            <TreeContext.Provider value={["external_locations"]}>
                <ExternalLocationTree setSize={5} setPos={3} />
            </TreeContext.Provider>
            <TreeContext.Provider value={["recipients"]}>
                <RecipientTree setSize={5} setPos={4} />
            </TreeContext.Provider>
            <TreeContext.Provider value={["shares"]}>
                <ShareTree setSize={5} setPos={5} />
            </TreeContext.Provider>
        </FlatTree>
    );
};

export default TreeView;
