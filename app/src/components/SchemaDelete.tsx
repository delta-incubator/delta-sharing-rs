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
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useCallback, useContext, useState } from "react";
import ucClient from "../client";
import { NotifyContext, TreeContext } from "../context";

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

type Props = { name: string };

const Default = ({ name }: Props) => {
    const [open, setOpen] = useState(false);
    const styles = useStyles();
    const queryKey = useContext(TreeContext);
    const notify = useContext(NotifyContext);
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.deleteSchema,
        onError: () => notify("error", "Failed to delete schema"),
        onSuccess: () => {
            notify("success", "Schema deleted successfully");
            queryClient.invalidateQueries({ queryKey });
            setOpen(false);
        },
    });

    const onClick = useCallback(() => {
        mutation.mutate({ catalog: queryKey[queryKey.length - 1], name });
    }, [mutation]);

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
                    <DialogTitle>Delete Schema</DialogTitle>
                    <DialogContent>
                        Are you sure you want to delete this schema?
                    </DialogContent>
                    <DialogActions>
                        <Button
                            className={styles.delete}
                            appearance="primary"
                            onClick={onClick}
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
};

export default Default;
