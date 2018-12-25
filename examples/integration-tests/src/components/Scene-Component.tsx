import * as React from "react";
import {SCENE} from "types/Types";
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

    loadScene() {
        loadWasm().then(wasmLib => 
            wasmLib.run(
                this.canvasRef.current, 
                this.props.scene, 
                () => {
                    this.setState({phase: PHASE.READY})
                }
            )
        )
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
