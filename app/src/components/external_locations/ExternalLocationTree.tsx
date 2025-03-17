import { CloudCubeRegular, CubeRegular } from "@fluentui/react-icons";
import ucClient, { ExternalLocationInfo } from "../../client";
import TreeBranch from "../TreeBranch";
import TreeLeaf, { type TreeLeafProps } from "../TreeLeaf";

type TreeProps = {
    setSize: number;
    setPos: number;
};

// Leaf component with the icon
const ExtLocLeaf = ({
    info,
    ref,
    setPos,
    setSize,
}: Omit<TreeLeafProps<ExternalLocationInfo>, "icon">) => {
    return (
        <TreeLeaf
            info={info}
            ref={ref}
            icon={<CubeRegular />}
            setSize={setSize}
            setPos={setPos}
        />
    );
};

const ExtLocTree = ({ setSize, setPos }: TreeProps) => {
    return (
        <TreeBranch
            setSize={setSize}
            setPos={setPos}
            listFn={() => ucClient.externalLocations.list()}
            ItemComponent={ExtLocLeaf}
            icon={<CloudCubeRegular />}
            rootName="External Locations"
        />
    );
};

export default ExtLocTree;
