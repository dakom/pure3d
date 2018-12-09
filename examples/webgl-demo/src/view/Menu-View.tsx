import * as React from "react";
import "styles/menu.scss";
import {SCENE} from "types/Types";

interface Props {
    selected: SCENE;
    onSelect: (scene:SCENE) => void;
}

export const MenuView = ({selected, onSelect}:Props) => (
    <ul className="menu">
        {Object.keys(SCENE)
            .map(key => ({key, value: SCENE[key]}))
            .filter(({key, value}) => value !== SCENE.NONE)
            .map(({key, value}) => 
                <li 
                   key={key}
                   className={selected === value ? "selected" : ""}
                   onClick={() => onSelect(value)}>
                    {key}
                </li>
            )}
    </ul>
)
