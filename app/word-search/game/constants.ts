export const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
export const words = [
    "FLY",
    "MAGIC",
    "WINGS",
    "TEAPOT",
    "TEACUP",
    "PIXIE",
    "FAIRY",
    "DUST",
    "GARDEN",
    "PARTY",
    "GLITTER",
    "SPARKLE",
];
export const directionsMap = {
    down: { row: 1, col: 0 },
    up: { row: -1, col: 0 },
    left: { row: 0, col: -1 },
    right: { row: 0, col: 1 },
};
export const directionsList = Object.keys(directionsMap);
