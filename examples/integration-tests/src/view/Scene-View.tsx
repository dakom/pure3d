import * as React from "react";
import "styles/scene.scss";

interface Props {
    canvasRef: React.RefObject<HTMLCanvasElement>;
}

export const SceneView = ({canvasRef}:Props) => (
    <canvas ref={canvasRef} id="canvas" className="canvas" touch-action="none"></canvas>
)
