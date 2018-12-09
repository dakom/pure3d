import * as React from "react";
import {SCENE} from "types/Types";
import {LoaderView} from "view/Loader-View";
import {SceneView} from "view/Scene-View";
type WasmLib = typeof import("../../target/webgl_demo");

interface Props {
    scene: SCENE;
}

interface State {
    loaded:boolean;
}

const loadWasm = (() => {
    let _loader:Promise<WasmLib>;
    let _wasmLib:WasmLib;

    const getLib = () => new Promise(resolve => 
           _wasmLib !== undefined
               ? resolve(_wasmLib)
               : getLoader()
    );

    const getLoader = () => {
        if(_loader === undefined) {
            _loader = import("../../target/webgl_demo");
            _loader.then(wasmLib => _wasmLib = wasmLib);
        }

        return _loader;
    }

    return getLoader;
})();

export class Scene extends React.Component<Props, State> {
    private canvasRef = React.createRef<HTMLCanvasElement>();

    readonly state:State = {
        loaded: false
    }

    componentDidMount() {
        this.loadScene();
    }

    componentDidUpdate(prevProps:Props) {
        if(prevProps.scene !== this.props.scene) {
            this.loadScene();
        }
    }

    loadScene() {
        loadWasm().then(wasmLib => {
        /*
         * 1. Mount the canvas - DONE!
         * 2. Pass canvas and onLoaded callback to WASM
         * 3. WASM will create context and load things
         * 4. WASM will call the onLoaded callback
         * 5. React will change state to loaded (and no longer show the UI Loader)
         */
            //TODO: pass canvasRef.current to WASM (real asset loading will upload to gpu)
            wasmLib.load_assets(this.props.scene, () => this.setState({loaded: false}));
        })
    }

    render() {
        const {scene} = this.props;
        const {loaded} = this.state;

        return (
            <React.Fragment>
                <SceneView canvasRef={this.canvasRef} />
                {!loaded && <LoaderView />}
            </React.Fragment>
        )


    }
}
