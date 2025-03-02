import {
    DrawerBody,
    DrawerHeader,
    DrawerHeaderTitle,
    InlineDrawer,
    makeStyles,
    tokens,
} from "@fluentui/react-components";
import TreeView from "./TreeView";
import CreateCredential from "./credentials/CredentialCreate";
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

    if (
        display === "create" &&
        scope?.length === 1 &&
        scope[0] === "credentials"
    ) {
        return <CreateCredential />;
    }
    return "No content";
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
