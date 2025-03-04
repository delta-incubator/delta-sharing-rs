import { DatabasePlugConnectedRegular } from "@fluentui/react-icons";
import ucClient from "../../client";
import ItemTree from "../TreeBranch";
import ItemLeaf from "../TreeLeaf";

type TreeProps = {
    setSize: number;
    setPos: number;
};

const RecipientTree = ({ setSize, setPos }: TreeProps) => {
    return (
        <ItemTree
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.recipients.list()}
            ItemComponent={ItemLeaf}
            icon={<DatabasePlugConnectedRegular />}
            rootName="Recipients"
        />
    );
};

export default RecipientTree;
