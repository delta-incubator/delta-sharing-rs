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
    Text,
    Tag,
} from "@fluentui/react-components";
import { Delete20Regular } from "@fluentui/react-icons";
import { useState, ReactNode, useMemo, useCallback } from "react";
import { useTreeContext, useNotify, useTypeName } from "../context";
import ucClient from "../client";
import { useMutation, useQueryClient } from "@tanstack/react-query";

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

function DeleteDialog({ title, content }: DeleteDialogProps) {
    const [open, setOpen] = useState(false);
    const styles = useStyles();

    const scope = useTreeContext();
    const typeName = useTypeName(scope);
    const { deleteFn } = useMemo(() => {
        if (scope.length === 2) {
            switch (scope[0]) {
                case "catalogs":
                    return {
                        deleteFn: ucClient.catalogs.delete,
                    };
                case "external_locations":
                    return {
                        deleteFn: ucClient.externalLocations.delete,
                    };
                case "shares":
                    return {
                        deleteFn: ucClient.shares.delete,
                    };
                case "credentials":
                    return {
                        deleteFn: ucClient.credentials.delete,
                    };
                case "recipients":
                    return {
                        deleteFn: ucClient.recipients.delete,
                    };
            }
        }

        if (scope.length === 3 && scope[0] === "catalogs") {
            return {
                deleteFn: (name: string) =>
                    ucClient.schemas.delete({ catalog: scope[1], name }),
                typeName: "Schema",
            };
        }

        throw new Error(`Unknown scope: ${scope}`);
    }, [scope]);

    const notify = useNotify();
    const queryClient = useQueryClient();
    const mutation = useMutation({
        mutationFn: deleteFn,
        onError: () =>
            notify("error", `Failed to delete ${typeName.toLowerCase()}.`),
        onSuccess: () => {
            setOpen(false);
            const fullName = scope.slice(1).join(".");
            const message = (
                <span>
                    <Text>{typeName}</Text>
                    <Tag>{fullName}</Tag>
                    <Text>deleted successfully</Text>
                </span>
            );
            notify("success", message);
            queryClient.invalidateQueries({
                queryKey: scope.slice(0, scope.length - 1),
            });
        },
    });

    const onClick = useCallback(() => {
        mutation.mutate(scope[scope.length - 1]);
    }, [mutation, scope]);

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
}

export default DeleteDialog;
