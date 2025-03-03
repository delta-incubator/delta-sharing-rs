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
    makeStyles,
    Tab,
    TabList,
    TabValue,
    tokens,
} from "@fluentui/react-components";
import { Add20Regular } from "@fluentui/react-icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
    Dispatch,
    FormEventHandler,
    SetStateAction,
    useCallback,
    useState,
} from "react";
import ucClient from "../../client";
import { useNotify, useTreeContext } from "../../context";
import { CreateCatalogRequestJson } from "../../gen/delta_sharing/catalogs/v1/svc_pb";
import { InputChange, TabSelect } from "../../types";

const useStyles = makeStyles({
    tabs: {
        padding: "10px 0 10px 10px",
        display: "flex",
        flexDirection: "column",
        rowGap: tokens.spacingVerticalL,
    },
});

const useCallbacks = (
    setValues: Dispatch<SetStateAction<CreateCatalogRequestJson>>,
) => {
    const onNameChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, name: data.value }));
    }, []);
    const onStorageChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, storageRoot: data.value }));
    }, []);
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

    return { onNameChange, onStorageChange, onProviderChange, onShareChange };
};

const Default = () => {
    const [values, setValues] = useState<CreateCatalogRequestJson>({});
    const [open, setOpen] = useState(false);
    const styles = useStyles();

    const [selectedValue, setSelectedValue] = useState<TabValue>("managed");
    const onTabSelect: TabSelect = useCallback((_ev, data) => {
        setSelectedValue(data.value);
    }, []);

    const notify = useNotify();
    const queryClient = useQueryClient();
    const queryKey = useTreeContext();
    const mutation = useMutation({
        mutationFn: ucClient.catalogs.create,
        onError: () => notify("error", "Failed to create catalog"),
        onSuccess: () => {
            notify("success", "Catalog created successfully");
            queryClient.invalidateQueries({ queryKey });
            setOpen(false);
            setValues({});
        },
    });

    const { onNameChange, onStorageChange, onProviderChange, onShareChange } =
        useCallbacks(setValues);

    const handleSubmit: FormEventHandler<HTMLFormElement> = useCallback(
        (ev) => {
            ev.preventDefault();
            mutation.mutate(values);
        },
        [mutation, values],
    );

    return (
        <Dialog open={open} onOpenChange={(_ev, data) => setOpen(data.open)}>
            <DialogTrigger disableButtonEnhancement>
                <Button
                    icon={<Add20Regular />}
                    appearance="subtle"
                    title="Add"
                />
            </DialogTrigger>
            <DialogSurface>
                <form onSubmit={handleSubmit}>
                    <DialogBody>
                        <DialogTitle>Create a new Catalog</DialogTitle>
                        <DialogContent>
                            <TabList
                                selectedValue={selectedValue}
                                onTabSelect={onTabSelect}
                            >
                                <Tab value="managed">Managed</Tab>
                                <Tab value="sharing">Sharing</Tab>
                            </TabList>
                            <div className={styles.tabs}>
                                <Field label="Name">
                                    <Input
                                        value={values.name}
                                        onChange={onNameChange}
                                        autoComplete="off"
                                        autoCapitalize="off"
                                        autoCorrect="off"
                                    />
                                </Field>
                                {selectedValue === "managed" && (
                                    <Field label="Storage root">
                                        <Input
                                            value={values.storageRoot}
                                            onChange={onStorageChange}
                                            autoComplete="off"
                                            autoCapitalize="off"
                                            autoCorrect="off"
                                        />
                                    </Field>
                                )}
                                {selectedValue === "sharing" && (
                                    <>
                                        <Field label="Provider name">
                                            <Input
                                                value={values.providerName}
                                                onChange={onProviderChange}
                                                autoComplete="off"
                                                autoCapitalize="off"
                                                autoCorrect="off"
                                            />
                                        </Field>
                                        <Field label="Share name">
                                            <Input
                                                value={values.shareName}
                                                onChange={onShareChange}
                                                autoComplete="off"
                                                autoCapitalize="off"
                                                autoCorrect="off"
                                            />
                                        </Field>
                                    </>
                                )}
                            </div>
                        </DialogContent>
                        <DialogActions>
                            <Button type="submit" appearance="primary">
                                Create
                            </Button>
                            <DialogTrigger disableButtonEnhancement>
                                <Button appearance="secondary">Close</Button>
                            </DialogTrigger>
                        </DialogActions>
                    </DialogBody>
                </form>
            </DialogSurface>
        </Dialog>
    );
};

export default Default;
