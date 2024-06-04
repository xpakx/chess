package io.github.xpakx.chess.game;

import io.github.xpakx.chess.game.dto.GameRequest;
import io.github.xpakx.chess.game.dto.NewGameResponse;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.security.Principal;
import java.util.List;

@RestController
@RequiredArgsConstructor
public class GameController {
    private final GameService service;

    @PostMapping("/game")
    public ResponseEntity<NewGameResponse> newGame(@Valid @RequestBody GameRequest request, Principal principal) {
        return new ResponseEntity<>(
                service.newGame(principal.getName(), request),
                HttpStatus.CREATED
        );
    }

}
