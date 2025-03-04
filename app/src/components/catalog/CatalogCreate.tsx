import {
    makeStyles,
    Tab,
    TabList,
    TabValue,
    tokens,
} from "@fluentui/react-components";
import { useCallback, useState } from "react";
import ucClient, { CreateCatalogRequest } from "../../client";
import { InputChange, TabSelect } from "../../types";
import { CreateResource, CreateFormState, Input } from "../forms";

const useStyles = makeStyles({
    tabs: {
        padding: "10px 0 10px 10px",
        display: "flex",
        flexDirection: "column",
        rowGap: tokens.spacingVerticalL,
    },
});

function CatalogForm({
    values,
    setValues,
}: CreateFormState<CreateCatalogRequest>) {
    const styles = useStyles();

    const [selectedValue, setSelectedValue] = useState<TabValue>("managed");
    const onTabSelect: TabSelect = useCallback((_ev, data) => {
        setSelectedValue(data.value);
    }, []);

    const onNameChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, name: data.value }));
    }, []);
    const onCommentChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, comment: data.value }));
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

    return (
        <>
            <TabList selectedValue={selectedValue} onTabSelect={onTabSelect}>
                <Tab value="managed">Managed</Tab>
                <Tab value="sharing">Sharing</Tab>
            </TabList>
            <div className={styles.tabs}>
                <Input
                    label="Name"
                    value={values.name ?? ""}
                    onChange={onNameChange}
                />
                <Input
                    label="Comment"
                    value={values.comment ?? ""}
                    onChange={onCommentChange}
                />
                {selectedValue === "managed" && (
                    <Input
                        label="Storage root"
                        value={values.storageRoot ?? ""}
                        onChange={onStorageChange}
                        type="url"
                    />
                )}
                {selectedValue === "sharing" && (
                    <>
                        <Input
                            label="Provider name"
                            value={values.providerName ?? ""}
                            onChange={onProviderChange}
                        />
                        <Input
                            label="Share name"
                            value={values.shareName ?? ""}
                            onChange={onShareChange}
                        />
                    </>
                )}
            </div>
        </>
    );
}

function CreateCatalog() {
    return (
        <CreateResource
            createFn={ucClient.catalogs.create}
            FormComponent={CatalogForm}
            resourceType="catalog"
            defaultValues={{
                properties: {},
            }}
            typeName="CreateCatalogRequest"
        />
    );
}

export default CreateCatalog;
