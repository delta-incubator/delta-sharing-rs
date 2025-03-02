import {
    DrawerBody,
    DrawerHeader,
    DrawerHeaderTitle,
    InlineDrawer,
    makeStyles,
    tokens,
} from "@fluentui/react-components";
import TreeView from "./TreeView";

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
        margin: `${tokens.spacingVerticalXL} ${tokens.spacingHorizontalXL}`,
        flex: "1",
    },
});

function Explorer() {
    const styles = useStyles();

    return (
        <div className={styles.root}>
            <div className={styles.container}>
                <InlineDrawer open className={styles.drawer}>
                    <DrawerHeader>
                        <DrawerHeaderTitle>Catalog Browser</DrawerHeaderTitle>
                    </DrawerHeader>
                    <DrawerBody>
                        <TreeView />
                    </DrawerBody>
                </InlineDrawer>
            </div>
            <p className={styles.content}>More Info here</p>
        </div>
    );
}

export default Explorer;
