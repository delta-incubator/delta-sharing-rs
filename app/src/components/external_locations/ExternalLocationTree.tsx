import { CloudCubeRegular } from "@fluentui/react-icons";
import ucClient from "../../client";
import ItemTree from "../TreeBranch";
import ItemLeaf from "../TreeLeaf";

type TreeProps = {
    setSize: number;
    setPos: number;
};

const ExtLocTree = ({ setSize, setPos }: TreeProps) => {
    return (
        <ItemTree
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.externalLocations.list()}
            ItemComponent={ItemLeaf}
            icon={<CloudCubeRegular />}
            rootName="External Locations"
        />
    );
};

export default ExtLocTree;
