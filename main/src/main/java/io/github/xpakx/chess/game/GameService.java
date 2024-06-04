package io.github.xpakx.chess.game;

import io.github.xpakx.chess.game.dto.*;
import io.github.xpakx.chess.game.error.GameNotFoundException;
import io.github.xpakx.chess.game.error.RequestProcessedException;
import io.github.xpakx.chess.game.error.UnauthorizedGameRequestChangeException;
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

    public List<GameSummary> getRequests(String username) {
        return gameRepository.findRequests(
                userRepository.findByUsername(username)
                        .orElseThrow(UserNotFoundException::new)
                        .getId()
                ).stream()
                .map((a) -> GameSummary.of(a, username)).toList();
    }

    public List<GameSummary> getActiveGames(String username) {
        return gameRepository.findActiveGames(
                userRepository.findByUsername(username)
                        .orElseThrow(UserNotFoundException::new)
                        .getId()
                ).stream()
                .map((a) -> GameSummary.of(a, username)).toList();
    }

    public List<GameSummary> getOldGames(String username) {
        return gameRepository.findFinishedGames(
                userRepository.findByUsername(username)
                        .orElseThrow(UserNotFoundException::new)
                        .getId()
                ).stream()
                .map((a) -> GameSummary.of(a, username)).toList();
    }

    public boolean acceptRequest(String username, Long requestId, AcceptRequest decision) {
        var game = gameRepository.findWithOpponentById(requestId)
                .orElseThrow(GameNotFoundException::new);
        if (game.getInvitation() != InvitationStatus.Issued) {
            throw new RequestProcessedException(
                    "Request already " + (game.getInvitation() == InvitationStatus.Accepted ? "accepted!" : "rejected!")
            );
        }
        if (!game.getOpponent().getUsername().equals(username)) {
            throw new UnauthorizedGameRequestChangeException();
        }
        if (decision.isAccepted()) {
            game.setInvitation(InvitationStatus.Accepted);
            game.setStartedAt(LocalDateTime.now());
        } else {
            game.setInvitation(InvitationStatus.Rejected);
        }
        gameRepository.save(game);
        return decision.isAccepted();
    }

    public void updateGame(Game game, UpdateEvent event) {
        game.setLastMoveAt(LocalDateTime.now());
        game.setStatus(event.getStatus());
        game.setCurrentState(event.getCurrentState());
        game.setUserTurn(event.isUserTurn());
        game.setLastMoveAt(event.getTimestamp());
        gameRepository.save(game);
    }

    public GameSummary getGame(String username, Long gameId) {
        var game = gameRepository.findWithUsersById(gameId).orElseThrow(GameNotFoundException::new);
        return GameSummary.of(game, username);
    }
}
