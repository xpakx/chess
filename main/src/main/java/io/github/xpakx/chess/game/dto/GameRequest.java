package io.github.xpakx.chess.game.dto;

import io.github.xpakx.chess.game.AIType;
import io.github.xpakx.chess.game.GameType;
import jakarta.validation.constraints.AssertTrue;
import jakarta.validation.constraints.NotNull;
import lombok.Getter;
import lombok.Setter;

import java.util.Objects;

@Getter
@Setter
public class GameRequest {
    @NotNull(message = "Game type cannot be null!")
    private GameType type;
    private String opponent;
    private AIType aiType;

    @AssertTrue(message = "User game request must have opponent username!")
    public boolean isOpponentIdSetForNonAIType() {
        return type != GameType.User || Objects.nonNull(opponent);
    }

    @AssertTrue(message = "AI game request should not have opponent username!")
    public boolean isOpponentIdUnsetForNonUserType() {
        return type == GameType.User || Objects.isNull(opponent);
    }

    @AssertTrue(message = "AI game must have specified AI type!")
    public boolean isAITypeSetForAIGame() {
        return type == GameType.User || (Objects.nonNull(aiType) && aiType != AIType.None);
    }

    @AssertTrue(message = "non-AI game cannot have AI type!")
    public boolean isAITypeUnsetForNonAIGame() {
        return type == GameType.AI || Objects.isNull(aiType) || aiType == AIType.None;
    }
}
