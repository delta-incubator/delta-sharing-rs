import "./App.css";
import {
    makeStyles,
    tokens,
    Toolbar,
    ToolbarButton,
    shorthands,
} from "@fluentui/react-components";
import Explorer from "./components/Explorer_";
import { SettingsRegular } from "@fluentui/react-icons";

const useStyles = makeStyles({
    root: {
        display: "flex",
        height: "100vh",
        width: "100vw",
        flexDirection: "column",
    },

    toolbar: {
        display: "flex",
        justifyContent: "space-between",
        padding: "0 1rem",

        borderBottomColor: tokens.colorNeutralForeground4,
        borderBottomWidth: "1px",
        borderBottomStyle: "solid",
    },

    button: {
        ...shorthands.borderColor(tokens.colorBrandStroke2),
        ":hover": {
            ...shorthands.borderColor(tokens.colorNeutralStroke1),
        },
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
                        className={styles.button}
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
