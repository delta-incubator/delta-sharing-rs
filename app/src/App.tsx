import "./App.css";
import {
    makeStyles,
    tokens,
    Toolbar,
    ToolbarButton,
} from "@fluentui/react-components";
import Explorer from "./components/Explorer";
import { SettingsRegular } from "@fluentui/react-icons";

const useStyles = makeStyles({
    root: {
        display: "flex",
        height: "100vh",
        width: "100vw",
        flexDirection: "column",
    },

    toolbar: {
        borderBottomColor: tokens.colorNeutralForeground4,
        borderBottomWidth: "1px",
        borderBottomStyle: "solid",
    },

    content: {
        flex: 1,
    },
});

function App() {
    const styles = useStyles();

    return (
        <div className={styles.root}>
            <div className={styles.toolbar}>
                <Toolbar size="medium">
                    <ToolbarButton
                        appearance="subtle"
                        icon={<SettingsRegular />}
                    />
                </Toolbar>
            </div>
            <div className={styles.content}>
                <Explorer />
            </div>
        </div>
    );
}

export default App;
