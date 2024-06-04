package io.github.xpakx.chess.user.dto;

import lombok.Builder;
import lombok.Getter;

@Builder
@Getter
public class AuthenticationResponse {
    private String token;
    private String refreshToken;
    private String username;
    private boolean moderatorRole;
}
