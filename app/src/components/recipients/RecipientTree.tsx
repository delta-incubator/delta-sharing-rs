import {
    DatabasePlugConnectedRegular,
    PlugConnectedRegular,
} from "@fluentui/react-icons";
import ucClient, { RecipientInfo } from "../../client";
import TreeBranch from "../TreeBranch";
import TreeLeaf, { type TreeLeafProps } from "../TreeLeaf";

type TreeProps = {
    setSize: number;
    setPos: number;
};

// Leaf component with the icon
const RecipientLeaf = ({
    info,
    ref,
    setPos,
    setSize,
}: Omit<TreeLeafProps<RecipientInfo>, "icon">) => {
    return (
        <TreeLeaf
            info={info}
            ref={ref}
            icon={<PlugConnectedRegular />}
            setSize={setSize}
            setPos={setPos}
        />
    );
};

const RecipientTree = ({ setSize, setPos }: TreeProps) => {
    return (
        <TreeBranch
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.recipients.list()}
            ItemComponent={RecipientLeaf}
            icon={<DatabasePlugConnectedRegular />}
            rootName="Recipients"
        />
    );
};

export default RecipientTree;
