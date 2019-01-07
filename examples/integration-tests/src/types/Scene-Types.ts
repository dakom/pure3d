export enum SCENE {
    NONE = "",
    QUAD = "Quad",
    TEXTURED_QUAD = "Textured Quad",
}

export const sceneIdLookup = new Map<SCENE, string>();

sceneIdLookup.set(SCENE.QUAD, "quad");
sceneIdLookup.set(SCENE.TEXTURED_QUAD, "quad_texture");
