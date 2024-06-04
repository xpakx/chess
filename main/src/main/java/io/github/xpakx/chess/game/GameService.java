package io.github.xpakx.chess.game;

import io.github.xpakx.chess.game.dto.*;
import io.github.xpakx.chess.game.error.UserNotFoundException;
import io.github.xpakx.chess.user.UserRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;
import java.util.List;
import java.util.Random;

@Service
@RequiredArgsConstructor
public class GameService {
    private final GameRepository gameRepository;
    private final UserRepository userRepository;

    public NewGameResponse newGame(String username, GameRequest request) {
        Game game;
        if (request.getType() == GameType.AI) {
            game = newGameAgainstAI(username, request);
        } else {
            game = newGameAgainstUser(username, request);
        }
        return new NewGameResponse(game.getId());
    }

    private Game newGameAgainstUser(String username, GameRequest request) {
        var newGame = new Game();
        newGame.setUser(userRepository.findByUsername(username).orElseThrow(UserNotFoundException::new));
        newGame.setOpponent(userRepository.findByUsername(request.getOpponent()).orElseThrow(UserNotFoundException::new));
        newGame.setType(GameType.User);
        newGame.setAiType(AIType.None);
        newGame.setStatus(GameStatus.NotFinished);
        newGame.setInvitation(InvitationStatus.Issued);
        var emptyState = createEmptyGameState();
        newGame.setCurrentState(emptyState);
        Random random = new Random();
        newGame.setUserStarts(random.nextBoolean());
        newGame.setUserTurn(newGame.isUserStarts());
        newGame.setNonCaptureMoves(0);
        return gameRepository.save(newGame);
    }

    private Game newGameAgainstAI(String username, GameRequest request) {
        var newGame = new Game();
        newGame.setUser(userRepository.findByUsername(username).orElseThrow(UserNotFoundException::new));
        newGame.setType(GameType.AI);
        newGame.setAiType(request.getAiType());
        newGame.setStatus(GameStatus.NotFinished);
        newGame.setInvitation(InvitationStatus.Accepted);
        var emptyState = createEmptyGameState();
        newGame.setCurrentState(emptyState);
        Random random = new Random();
        newGame.setUserStarts(random.nextBoolean());
        newGame.setUserTurn(newGame.isUserStarts());
        newGame.setNonCaptureMoves(0);
        return gameRepository.save(newGame);
    }

    private String createEmptyGameState() {
        return ""; // TODO
    }
}
