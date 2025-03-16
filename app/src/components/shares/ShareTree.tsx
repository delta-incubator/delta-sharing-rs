import { ShareMultipleRegular, ShareRegular } from "@fluentui/react-icons";
import ucClient, { ShareInfo } from "../../client";
import TreeBranch from "../TreeBranch";
import TreeLeaf, { type TreeLeafProps } from "../TreeLeaf";

type ShareTreeProps = {
    setSize: number;
    setPos: number;
};

// Leaf component with the icon
const ShareLeaf = ({ info, ref }: Omit<TreeLeafProps<ShareInfo>, "icon">) => {
    return <TreeLeaf info={info} ref={ref} icon={<ShareRegular />} />;
};

const ShareTree = ({ setSize, setPos }: ShareTreeProps) => {
    return (
        <TreeBranch
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.shares.list()}
            ItemComponent={ShareLeaf}
            icon={<ShareMultipleRegular />}
            rootName="Shares"
        />
    );
};

export default ShareTree;
