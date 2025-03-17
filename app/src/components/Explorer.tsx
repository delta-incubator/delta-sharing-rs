import {
    DrawerBody,
    DrawerHeader,
    DrawerHeaderTitle,
    InlineDrawer,
    makeStyles,
    tokens,
} from "@fluentui/react-components";
import TreeView from "./TreeView";
import CreateSchema from "./catalog/SchemaCreate";
import CreateTable from "./catalog/TableCreate";
import CreateCatalog from "./catalog/CatalogCreate";
import CreateCredential from "./credentials/CredentialCreate";
import CreateExternalLocation from "./external_locations/ExternalLocationCreate";
import CreateRecipient from "./recipients/RecipientCreate";
import CreateShare from "./shares/ShareCreate";
import { ExplorerProvider, ExplorerPropsInner, useExplorer } from "../context";
import { useState } from "react";

const useStyles = makeStyles({
    root: {
        display: "flex",
        height: "100%",
        width: "100%",
        userSelect: "auto",
    },

    container: {
        position: "relative",
    },

    drawer: {
        width: "320px",
        borderRightColor: tokens.colorNeutralForeground4,
        borderRightWidth: "1px",
        borderRightStyle: "solid",
        height: "100%",
    },

    content: {
        flex: "1",
    },
});

function ExplorerContent() {
    const { display, scope } = useExplorer();

    if (display === "create") {
        if (scope?.length === 1) {
            if (scope[0] === "catalogs") {
                return <CreateCatalog />;
            } else if (scope[0] === "credentials") {
                return <CreateCredential />;
            } else if (scope[0] === "external_locations") {
                return <CreateExternalLocation />;
            } else if (scope[0] === "recipients") {
                return <CreateRecipient />;
            } else if (scope[0] === "shares") {
                return <CreateShare />;
            }
        } else if (scope && scope[0] === "catalogs") {
            if (scope?.length === 2) {
                return <CreateSchema />;
            } else if (scope?.length === 3) {
                return <CreateTable />;
            }
        }
    }

    return "no content";
}

function Explorer() {
    const styles = useStyles();
    const [state, setState] = useState<ExplorerPropsInner>({});

    return (
        <ExplorerProvider value={{ ...state, update: setState }}>
            <div className={styles.root}>
                <div className={styles.container}>
                    <InlineDrawer open className={styles.drawer}>
                        <DrawerHeader>
                            <DrawerHeaderTitle>
                                Catalog Browser
                            </DrawerHeaderTitle>
                        </DrawerHeader>
                        <DrawerBody>
                            <TreeView />
                        </DrawerBody>
                    </InlineDrawer>
                </div>
                <div className={styles.content}>
                    <ExplorerContent />
                </div>
            </div>
        </ExplorerProvider>
    );
}

export default Explorer;
