import { KeyMultipleRegular } from "@fluentui/react-icons";
import ucClient from "../../client";
import ItemTree from "../TreeBranch";
import ItemLeaf from "../TreeLeaf";

type CatalogTreeProps = {
    setSize: number;
    setPos: number;
};

const CredentialTree = ({ setSize, setPos }: CatalogTreeProps) => {
    return (
        <ItemTree
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.credentials.list()}
            itemComponent={ItemLeaf}
            icon={<KeyMultipleRegular />}
            rootName="Credentials"
        />
    );
};

export default CredentialTree;
