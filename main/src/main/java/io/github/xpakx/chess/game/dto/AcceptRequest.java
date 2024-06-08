package io.github.xpakx.chess.game.dto;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class AcceptRequest {
    private AcceptStatus status;

    public boolean isAccepted() {
        return status == AcceptStatus.Accepted;
    }
}
