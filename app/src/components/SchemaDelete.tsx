import {
    Dialog,
    DialogTrigger,
    DialogSurface,
    DialogTitle,
    DialogBody,
    DialogActions,
    DialogContent,
    Button,
    useToastController,
    Toast,
    ToastTitle,
    useId,
    Toaster,
    ToastIntent,
    makeStyles,
    tokens,
} from "@fluentui/react-components";
import { Delete20Regular } from "@fluentui/react-icons";
import { useState, useCallback } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import ucClient from "../client";

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

type Props = { catalog: string; name: string };

const Default = ({ catalog, name }: Props) => {
    const [open, setOpen] = useState(false);
    const styles = useStyles();

    const toasterId = useId("toaster");
    const { dispatchToast } = useToastController(toasterId);
    const notify = useCallback(
        (intent: ToastIntent, message: string) =>
            dispatchToast(
                <Toast>
                    <ToastTitle>{message}</ToastTitle>
                </Toast>,
                { position: "top", intent },
            ),
        [],
    );

    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: ucClient.deleteSchema,
        onError: () => {
            notify("error", "Failed to delete schema");
        },
        onSuccess: () => {
            notify("success", "Schema deleted successfully");
            queryClient.invalidateQueries({
                queryKey: ["catalogs", catalog],
            });
            setOpen(false);
        },
    });

    const onClick = useCallback(() => {
        mutation.mutate({ catalog, name });
    }, [mutation]);

    return (
        <>
            <Toaster toasterId={toasterId} />
            <Dialog
                open={open}
                onOpenChange={(_ev, data) => setOpen(data.open)}
            >
                <DialogTrigger disableButtonEnhancement>
                    <Button
                        icon={<Delete20Regular className={styles.deleteIcon} />}
                        appearance="subtle"
                        title="Add"
                    />
                </DialogTrigger>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>Create a new Schema</DialogTitle>
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
        </>
    );
};

export default Default;
