import {
    Dialog,
    DialogTrigger,
    DialogSurface,
    DialogTitle,
    DialogBody,
    DialogActions,
    DialogContent,
    Button,
    Field,
    Input,
    InputProps,
    useToastController,
    Toast,
    ToastTitle,
    useId,
    Toaster,
    ToastIntent,
    Tab,
    TabList,
    TabValue,
    TabListProps,
    makeStyles,
} from "@fluentui/react-components";
import { Add20Regular } from "@fluentui/react-icons";
import { useState, useCallback, Dispatch, SetStateAction } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import ucClient from "../client";
import { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";

type InputChange = NonNullable<InputProps["onChange"]>;
type TabSelect = NonNullable<TabListProps["onTabSelect"]>;

const useStyles = makeStyles({
    tabs: {
        marginTop: "1rem",
        marginBottom: "1rem",
        marginLeft: "1rem",
        marginRight: "1rem",
    },
});

const Managed = ({
    values,
    setValues,
}: {
    values: CreateCatalogRequestJson;
    setValues: Dispatch<SetStateAction<CreateCatalogRequestJson>>;
}) => {
    const onChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, storageRoot: data.value }));
    }, []);

    return (
        <>
            <Field label="Storage root">
                <Input value={values.storageRoot} onChange={onChange} />
            </Field>
        </>
    );
};

const Sharing = ({
    values,
    setValues,
}: {
    values: CreateCatalogRequestJson;
    setValues: Dispatch<SetStateAction<CreateCatalogRequestJson>>;
}) => {
    const onProviderChange: InputChange = useCallback(
        (_ev, data) => {
            setValues((curr) => ({ ...curr, providerName: data.value }));
        },
        [setValues],
    );

    const onShareChange: InputChange = useCallback(
        (_ev, data) => {
            setValues((curr) => ({ ...curr, shareName: data.value }));
        },
        [setValues],
    );

    return (
        <>
            <Field label="Provider name">
                <Input
                    value={values.providerName}
                    onChange={onProviderChange}
                />
            </Field>
            <Field label="Share name">
                <Input value={values.shareName} onChange={onShareChange} />
            </Field>
        </>
    );
};

const Default = () => {
    const [values, setValues] = useState<CreateCatalogRequestJson>({});
    const [open, setOpen] = useState(false);
    const styles = useStyles();

    const [selectedValue, setSelectedValue] = useState<TabValue>("managed");
    const onTabSelect: TabSelect = useCallback((_ev, data) => {
        setSelectedValue(data.value);
    }, []);

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
        mutationFn: ucClient.createCatalog,
        onError: () => {
            notify("error", "Failed to create catalog");
        },
        onSuccess: () => {
            notify("success", "Catalog created successfully");
            queryClient.invalidateQueries({ queryKey: ["catalogs", "list"] });
            setOpen(false);
            setValues({});
        },
    });

    const onChange: InputChange = useCallback((_ev, data) => {
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
                        onClick={() => {
                            console.log("Add clicked");
                        }}
                    />
                </DialogTrigger>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>Create a new Catalog</DialogTitle>
                        <DialogContent>
                            <Field label="Name">
                                <Input
                                    value={values.name}
                                    onChange={onChange}
                                />
                            </Field>
                            <TabList
                                selectedValue={selectedValue}
                                onTabSelect={onTabSelect}
                            >
                                <Tab value="managed">Managed</Tab>
                                <Tab value="sharing">Sharing</Tab>
                            </TabList>
                            <div className={styles.tabs}>
                                {selectedValue === "managed" && (
                                    <Managed
                                        values={values}
                                        setValues={setValues}
                                    />
                                )}
                                {selectedValue === "sharing" && (
                                    <Sharing
                                        values={values}
                                        setValues={setValues}
                                    />
                                )}
                            </div>
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
