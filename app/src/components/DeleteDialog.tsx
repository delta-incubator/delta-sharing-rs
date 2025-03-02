import {
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    makeStyles,
    tokens,
} from "@fluentui/react-components";
import { Delete20Regular } from "@fluentui/react-icons";
import { useState, ReactNode } from "react";

const useStyles = makeStyles({
    delete: {
        backgroundColor: tokens.colorStatusDangerBackground3,
        "&:hover": {
            backgroundColor: tokens.colorStatusDangerBackground3Hover,
        },
    },

    deleteIcon: {
        "&:hover": {
            color: tokens.colorStatusDangerBackground3Hover,
        },
    },
});

type DeleteDialogProps = {
    onClick: () => void;
    title: ReactNode;
    content: ReactNode;
};

function DeleteDialog({ onClick, title, content }: DeleteDialogProps) {
    const [open, setOpen] = useState(false);
    const styles = useStyles();

    return (
        <Dialog open={open} onOpenChange={(_ev, data) => setOpen(data.open)}>
            <DialogTrigger disableButtonEnhancement>
                <Button
                    icon={<Delete20Regular className={styles.deleteIcon} />}
                    appearance="subtle"
                    title="Delete"
                />
            </DialogTrigger>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogContent>{content}</DialogContent>
                    <DialogActions>
                        <Button
                            className={styles.delete}
                            appearance="primary"
                            onClick={() => {
                                onClick();
                                setOpen(false);
                            }}
                        >
                            Delete
                        </Button>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="secondary">Cancel</Button>
                        </DialogTrigger>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
}

export default DeleteDialog;
