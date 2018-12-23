import * as React from "react";
import {MenuView} from "view/Menu-View";
import {SCENE} from "types/Types";
interface Props {
    onSelect: (menu:SCENE) => void;
    menu: SCENE;
}

interface State {
}

export class Menu extends React.Component<Props, State> {
    render() {
        const {menu, onSelect} = this.props;
        return <MenuView selected={menu} onSelect={onSelect} />
    }
}
