export interface GameRequest {
    type: "AI" | "User";
    opponent?: String;
    aiType?: "Random" | "None";
}