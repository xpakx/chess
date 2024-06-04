package io.github.xpakx.chess.user;

import io.github.xpakx.chess.user.dto.AuthenticationRequest;
import io.github.xpakx.chess.user.dto.AuthenticationResponse;
import io.github.xpakx.chess.user.dto.RefreshTokenRequest;
import io.github.xpakx.chess.user.dto.RegistrationRequest;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequiredArgsConstructor
public class AuthController {
    private final AuthService service;

    @PostMapping("/authenticate")
    public ResponseEntity<AuthenticationResponse> authenticate(
            @Valid @RequestBody AuthenticationRequest authenticationRequest) {
        return ResponseEntity.ok(
                service.generateAuthenticationToken(authenticationRequest)
        );
    }

    @PostMapping("/register")
    public ResponseEntity<AuthenticationResponse> register(
            @Valid @RequestBody RegistrationRequest registrationRequest) {
        return new ResponseEntity<>(
                service.register(registrationRequest),
                HttpStatus.CREATED
        );
    }

    @PostMapping("/refresh")
    public ResponseEntity<AuthenticationResponse> refreshToken(
            @Valid @RequestBody RefreshTokenRequest request) {
        return ResponseEntity.ok(
                service.refresh(request)
        );
    }
}
