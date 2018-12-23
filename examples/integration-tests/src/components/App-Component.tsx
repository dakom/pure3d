import * as React from "react";
import {Menu} from "components/Menu-Component";
import {SCENE} from "types/Types";
import {Scene} from "components/Scene-Component";
import {debugSettings} from "config/Config";
interface Props {
}

interface State {
    scene: SCENE;
}

export class App extends React.Component<Props, State> {
    readonly state:State = {
        scene: debugSettings.scene
    }

    render() {
        const {scene} = this.state;
        return <React.Fragment>
                <Scene scene={scene} />
                <Menu 
                    onSelect={scene => this.setState({scene})} 
                    menu={scene}
                />
              </React.Fragment>
    }
}
