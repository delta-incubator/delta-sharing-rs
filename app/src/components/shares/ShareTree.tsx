import { ShareMultipleRegular } from "@fluentui/react-icons";
import ucClient from "../../client";
import ItemTree from "../TreeBranch";
import ItemLeaf from "../TreeLeaf";

type ShareTreeProps = {
    setSize: number;
    setPos: number;
};

const ShareTree = ({ setSize, setPos }: ShareTreeProps) => {
    return (
        <ItemTree
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.shares.list()}
            itemComponent={ItemLeaf}
            icon={<ShareMultipleRegular />}
            rootName="Shares"
        />
    );
};

export default ShareTree;
