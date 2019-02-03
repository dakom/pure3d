import * as React from "react";
import {SCENE, sceneIdLookup} from "types/Types";
import {LoaderView} from "view/Loader-View";
import {ErrorView} from "view/Error-View";
import {SceneView} from "view/Scene-View";
type WasmLib = typeof import("../../target/integration_tests");

enum PHASE {
    LOADING = "loading",
    ERROR = "error",
    READY = "ready"
}
interface Props {
    scene: SCENE;
}

interface State {
    phase: PHASE;
    errorMessage?: string;
}

const loadWasm = (() => {
    let _loader:Promise<WasmLib>;
    let _wasmLib:WasmLib;

    const getLib = ():Promise<WasmLib> => new Promise(resolve => {
           _wasmLib !== undefined
               ? resolve(_wasmLib)
               : resolve(getLoader())
    });

    const getLoader = () => {
        if(_loader === undefined) {
            _loader = import("../../target/integration_tests");
            _loader.then(wasmLib => _wasmLib = wasmLib);
        }

        return _loader;
    }

    return getLib;
})();

export class Scene extends React.Component<Props, State> {
    private canvasRef = React.createRef<HTMLCanvasElement>();

    private cleanup = () => {};

    readonly state:State = {
        phase: PHASE.LOADING
    }

    componentDidMount() {
        this.loadScene();
    }

    componentDidUpdate(prevProps:Props) {
        if(prevProps.scene !== this.props.scene) {
            this.loadScene();
        }
    }

    componentWillUnmount() {
        this.cleanup();
    }

    loadScene() {
        this.cleanup();

        loadWasm().then(wasmLib => 
            wasmLib.run(
                this.canvasRef.current, 
                sceneIdLookup.get(this.props.scene), 
            )
        )
        .then(wasm_cleanup => {
            this.setState({phase: PHASE.READY})
            this.cleanup = () => {
                console.log("FREEEING!!!");
                wasm_cleanup();
                this.cleanup = () => {};
            }
        })
        .catch(errorMessage => {
            console.error(errorMessage)
            this.setState({phase: PHASE.ERROR, errorMessage});
        })
    }

    render() {
        const {scene} = this.props;
        const {phase, errorMessage} = this.state;

        return (
            <React.Fragment>
                <SceneView canvasRef={this.canvasRef} />
                {(() => {
                    switch(phase) {
                        case PHASE.LOADING:
                            return <LoaderView />
                        case PHASE.ERROR:
                            return <ErrorView message={errorMessage} scene={scene} />
                        case PHASE.READY:
                        default: return null;
                    }
                })()}
                
            </React.Fragment>
        )


    }
}
