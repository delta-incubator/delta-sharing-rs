import { KeyMultipleRegular, KeyRegular } from "@fluentui/react-icons";
import ucClient, { CredentialInfo } from "../../client";
import TreeBranch from "../TreeBranch";
import TreeLeaf, { type TreeLeafProps } from "../TreeLeaf";

type CatalogTreeProps = {
    setSize: number;
    setPos: number;
};

// Leaf component with the icon
const CredentialLeaf = ({
    info,
    ref,
    setPos,
    setSize,
}: Omit<TreeLeafProps<CredentialInfo>, "icon">) => {
    return (
        <TreeLeaf
            info={info}
            ref={ref}
            icon={<KeyRegular />}
            setSize={setSize}
            setPos={setPos}
        />
    );
};

const CredentialTree = ({ setSize, setPos }: CatalogTreeProps) => {
    return (
        <TreeBranch
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.credentials.list()}
            ItemComponent={CredentialLeaf}
            icon={<KeyMultipleRegular />}
            rootName="Credentials"
        />
    );
};

export default CredentialTree;
