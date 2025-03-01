import {
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    Field,
    Input,
    Toast,
    Toaster,
    ToastIntent,
    ToastTitle,
    useId,
    useToastController,
} from "@fluentui/react-components";
import { Add20Regular } from "@fluentui/react-icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useCallback, useState } from "react";
import ucClient, { CreateSchemaRequest } from "../client";
import { InputChange } from "../types";

type Props = { name: string };

const Default = ({ name }: Props) => {
    const [values, setValues] = useState<CreateSchemaRequest>({
        catalogName: name,
    });
    const [open, setOpen] = useState(false);

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
        mutationFn: ucClient.createSchema,
        onError: () => {
            notify("error", "Failed to create schema");
        },
        onSuccess: () => {
            notify("success", "Schema created successfully");
            queryClient.invalidateQueries({
                queryKey: ["catalogs", name],
            });
            setOpen(false);
            setValues({});
        },
    });

    const onNameChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, name: data.value }));
    }, []);

    const onClick = useCallback(() => {
        mutation.mutate(values);
    }, [mutation, values]);

    return (
        <>
            <Toaster toasterId={toasterId} />
            <Dialog
                open={open}
                onOpenChange={(_ev, data) => setOpen(data.open)}
            >
                <DialogTrigger disableButtonEnhancement>
                    <Button
                        icon={<Add20Regular />}
                        appearance="subtle"
                        title="Add"
                    />
                </DialogTrigger>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>Create a new Schema</DialogTitle>
                        <DialogContent>
                            <Field label="Name">
                                <Input
                                    value={values.name}
                                    onChange={onNameChange}
                                />
                            </Field>
                        </DialogContent>
                        <DialogActions>
                            <Button appearance="primary" onClick={onClick}>
                                Create
                            </Button>
                            <DialogTrigger disableButtonEnhancement>
                                <Button appearance="secondary">Close</Button>
                            </DialogTrigger>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </>
    );
};

export default Default;
