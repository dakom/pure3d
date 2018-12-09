import * as React from "react";
import {SCENE} from "types/Types";
import "styles/errors.scss";

interface Props {
    message: string;
    scene:SCENE;
}

export const ErrorView = ({message, scene}:Props) => (
    <div className="error">
        <div className="label">
            {message}
            <br/>
            {`Scene: ${scene}`}
        </div>
    </div>
)
