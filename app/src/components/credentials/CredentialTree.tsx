import {
    FlatTreeItem,
    TreeItemLayout,
    Spinner,
    Button,
} from "@fluentui/react-components";
import { KeyMultipleRegular, AddRegular } from "@fluentui/react-icons";
import { useQuery } from "@tanstack/react-query";
import { useCallback, useContext, useEffect, useRef, useState } from "react";
import ucClient from "../../client";
import { TreeContext, useExplorer, useTreeScope } from "../../context";
import { TreeItemOnChange } from "../../types";
import CredentialItem from "./CredentialItem";

type CatalogTreeProps = {
    setSize: number;
    setPos: number;
};

const useCredentials = (open: boolean) => {
    const rootScope = useContext(TreeContext);
    const { data, status } = useQuery({
        queryKey: rootScope,
        queryFn: () => ucClient.credentials.list(),
        enabled: open,
        refetchInterval: 30000,
    });

    return { data: data || [], status };
};

const CreateCredential = () => {
    const { update } = useExplorer();
    const scope = useTreeScope();
    const onClick = useCallback(() => {
        update({ display: "create", scope });
    }, [update]);

    return (
        <Button appearance="subtle" onClick={onClick} icon={<AddRegular />} />
    );
};

const CredentialTree = ({ setSize, setPos }: CatalogTreeProps) => {
    const [open, setOpen] = useState(false);
    const onOpenChange: TreeItemOnChange = useCallback(
        (_ev, data) => setOpen(data.open),
        [setOpen],
    );

    const rootScope = useContext(TreeContext);
    const rootValue = rootScope[0];
    const { data, status } = useCredentials(open);

    const firstItemRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (open && status === "success") firstItemRef.current?.focus();
    }, [open, status]);

    return (
        <>
            <FlatTreeItem
                value={rootValue}
                aria-level={1}
                aria-setsize={setSize}
                aria-posinset={setPos}
                itemType="branch"
                open={open}
                onOpenChange={onOpenChange}
            >
                <TreeItemLayout
                    iconBefore={<KeyMultipleRegular />}
                    expandIcon={
                        open && status === "pending" ? (
                            <Spinner size="extra-tiny" />
                        ) : undefined
                    }
                    actions={<CreateCredential />}
                >
                    Credentials
                </TreeItemLayout>
            </FlatTreeItem>
            {open &&
                status === "success" &&
                data.map(
                    (item, index) =>
                        item.name && (
                            <TreeContext.Provider value={rootScope}>
                                <CredentialItem
                                    key={`${rootValue}.${item.name}`}
                                    ref={index === 0 ? firstItemRef : null}
                                    info={item as { name: string }}
                                />
                            </TreeContext.Provider>
                        ),
                )}
        </>
    );
};

export default CredentialTree;
